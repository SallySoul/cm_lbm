use crate::*;

pub struct SolverParams {
    pub bounce_back: BounceBack,
    pub ic_params_uniform: BCParamsUniform,
    pub bc_params_uniform: BCParamsUniform,
    pub grid_dimensions: AABB3,
    pub collision_type: CollisionType,
    pub stream_figure: bool,
}

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
    pub fn new(driver: &Driver, params: SolverParams) -> Self {
        let device = &driver.device;
        let grid_dimensions = &params.grid_dimensions;
        let grid_dimensions_uniform =
            GridDimensionsUniform::new(device, grid_dimensions);
        let distributions = Distributions::new(device, grid_dimensions);

        let moments = Moments::new(
            device,
            grid_dimensions,
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
            &params.ic_params_uniform,
            &grid_dimensions_uniform,
            &work_groups,
            params.stream_figure,
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
            grid_dimensions,
            &grid_dimensions_uniform,
            &distributions,
            params.bc_params_uniform,
        );

        let slip_surfaces = SlipSurfaces::new(
            device,
            grid_dimensions,
            &grid_dimensions_uniform,
            &moments,
            &distributions,
        );

        let stream = Stream::new(
            device,
            grid_dimensions,
            &grid_dimensions_uniform,
            &distributions,
        );

        let collision = Collision::new(
            device,
            grid_dimensions,
            &grid_dimensions_uniform,
            &moments,
            &params.bounce_back,
            &distributions,
            params.collision_type,
        );

        Solver {
            grid_dimensions: params.grid_dimensions,
            work_groups,
            grid_dimensions_uniform,
            bounce_back: params.bounce_back,
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

    // Run the solver for n_it - 1 iterations
    // and write out a snapshot every n_out snapshots (including zero not n_it).
    pub fn run(
        &mut self,
        driver: &Driver,
        output_dir: &str,
        n_it: usize,
        n_out: usize,
    ) {
        self.write_vtk(driver, &format!("{}/moments_{:09}.vtu", output_dir, 0));
        for iter in 1..n_it {
            //println!("iter: {}", iter);
            self.apply_stream(driver);

            run_submission(driver, |encoder| {
                self.moments(encoder);
                self.apply_dirichlet(encoder);
                self.apply_collision(encoder);
                self.apply_dirichlet(encoder);
            });

            let write_output = (iter + 1) % n_out == 0;
            if write_output {
                run_submission(driver, |encoder| self.moments(encoder));
                self.write_vtk(
                    driver,
                    &format!("{}/moments_{:09}.vtu", output_dir, iter),
                );
            }
        }
    }
}
