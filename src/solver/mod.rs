mod bc_uniform;
mod bounce_back;
mod cm_mrt;
mod collision;
mod dimensions_uniform;
mod dirichlet;
mod distributions;
mod faces;
mod init_op;
mod moments;
mod shader_template;
mod slip_surfaces;
mod stream;
mod cm_mrt_template;

pub use bc_uniform::*;
pub use bounce_back::*;
pub use cm_mrt::*;
pub use collision::*;
pub use dimensions_uniform::*;
pub use dirichlet::*;
pub use distributions::*;
pub use faces::*;
pub use init_op::*;
pub use moments::*;
pub use shader_template::*;
pub use slip_surfaces::*;
pub use stream::*;
pub use cm_mrt_template::*;

use crate::*;

pub struct Solver {
    grid_dimensions: AABB3,

    work_groups: [u32; 3],

    grid_dimensions_uniform: GridDimensionsUniform,

    bounce_back: BounceBack,

    distributions: Distributions,

    moments: Moments,

    dirichlet: Dirichlet,

    slip_surfaces: SlipSurfaces,

    stream: Stream,

    collision: Collision,
}

impl Solver {
    pub fn new(
        driver: &Driver,
        bounce_back: BounceBack,
        ic_params_uniform: BCParamsUniform,
        bc_params_uniform: BCParamsUniform,
        grid_dimensions: AABB3,
        omega: f32,
        stream_figure: bool,
    ) -> Self {
        let device = &driver.device;
        let grid_dimensions_uniform =
            GridDimensionsUniform::new(device, &grid_dimensions);
        let distributions = Distributions::new(device, &grid_dimensions);

        let moments = Moments::new(
            device,
            &grid_dimensions,
            &distributions,
            &grid_dimensions_uniform,
        );

        // They don't use inclusive ranges or something?
        // add one fixes it, or I don't need it?
        // TODO: see if we can remove it?
        let max = (grid_dimensions.column(1) - grid_dimensions.column(0))
            .add_scalar(1);
        let work_groups = [
            (max[0] / 4 + 1) as u32,
            (max[1] / 4 + 1) as u32,
            (max[2] / 4 + 1) as u32,
        ];

        // At any rate, set initial conditions, ensure moments are correct
        set_initial_conditions(
            driver,
            &distributions,
            &ic_params_uniform,
            &grid_dimensions_uniform,
            &work_groups,
            stream_figure,
        );
        run_submission(driver, |encoder| {
            moments.compute(
                &work_groups,
                encoder,
                &distributions,
                &grid_dimensions_uniform,
            );
        });

        let dirichlet = Dirichlet::new(
            device,
            &grid_dimensions,
            &grid_dimensions_uniform,
            &distributions,
            bc_params_uniform,
        );

        let slip_surfaces = SlipSurfaces::new(
            device,
            &grid_dimensions,
            &grid_dimensions_uniform,
            &moments,
            &distributions,
        );

        let stream = Stream::new(
            device,
            &grid_dimensions,
            &grid_dimensions_uniform,
            &distributions,
        );

        let collision = Collision::new(
            device,
            &grid_dimensions,
            &grid_dimensions_uniform,
            &moments,
            &bounce_back,
            &distributions,
            omega,
        );

        Solver {
            grid_dimensions,
            work_groups,
            grid_dimensions_uniform,
            bounce_back,
            distributions,
            moments,
            dirichlet,
            slip_surfaces,
            stream,
            collision,
        }
    }

    pub fn moments(&self, encoder: &mut wgpu::CommandEncoder) {
        self.moments.compute(
            &self.work_groups,
            encoder,
            &self.distributions,
            &self.grid_dimensions_uniform,
        );
    }

    pub fn apply_dirichlet(&self, encoder: &mut wgpu::CommandEncoder) {
        self.dirichlet.apply(
            encoder,
            &self.distributions,
            &self.grid_dimensions_uniform,
        );
    }

    pub fn apply_slip_surfaces(&self, encoder: &mut wgpu::CommandEncoder) {
        self.slip_surfaces.apply(
            encoder,
            &self.moments,
            &self.distributions,
            &self.grid_dimensions_uniform,
        );
    }

    pub fn apply_stream(&mut self, driver: &Driver) {
        run_submission(driver, |encoder| {
            self.stream.apply(
                encoder,
                &self.distributions,
                &self.grid_dimensions_uniform,
            )
        });
        self.distributions.swap();
    }

    pub fn apply_collision(&mut self, encoder: &mut wgpu::CommandEncoder) {
        self.collision.apply(
            encoder,
            &self.distributions,
            &self.moments,
            &self.bounce_back,
        );
    }

    pub fn write_vtk(&self, driver: &Driver, path: &str) {
        println!("  writing VTK: {}", path);
        let mut grid = VTKGrid::new(&self.grid_dimensions);
        let (densities, velocities) = self.moments.get_data(driver);
        grid.add_attribute("density".to_string(), 1, densities);
        grid.add_attribute("velocity".to_string(), 3, velocities);
        grid.write(path);
    }
}
