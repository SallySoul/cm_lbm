use cm_lbm::dimensions::*;
use cm_lbm::io::*;
use cm_lbm::kernel::*;
use cm_lbm::wgpu_util;

#[tokio::main]
async fn main() {
    let driver = wgpu_util::setup_wgpu().await;
    let rows = 120;
    let cols = 200;
    let d = LatticeDimensions {
        rows,
        cols,
        total: rows * cols,
        q: 9,
        size: 1.0,
    };
    let c_params = CollisionParams {
        c_s: 0.5,
        delta_t: 0.1,
        tau: 0.5,
    };
    let work_group_x = (cols / 4) + 1;
    let work_group_y = (rows / 4) + 1;
    let group_n = [work_group_x as u32, work_group_y as u32, 1];
    let t = 300;

    let u = LatticeDimensionsUniform::new(&driver.device, d.clone());

    let mut densities = Densities::new(&driver.device, &d);

    let mut densities_raw = Vec::with_capacity(d.q as usize);
    for i in 0..d.q {
        densities_raw.push(vec![0.0; d.total as usize]);
        densities_raw[i as usize][0] = 1.0;
    }
    let input_data = DensitiesData {
        densities: densities_raw,
    };
    densities.set_data(&driver, input_data);

    let macros = Macros::new(&driver.device, &densities, &u);

    let encoder_label = "read_encoder";
    let mut encoder = driver
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(encoder_label),
        });
    macros.compute(group_n, &mut encoder, &densities);
    let submission = driver.queue.submit(Some(encoder.finish()));
    driver
        .device
        .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));
    let mut b = VTKBuilder2D::new(&d);
    b.add_densities(&driver, &densities);
    b.add_macros(&driver, &macros);
    b.export("vtu_test/frame_0.vtu");

    let s = Stream2D::new(&driver.device, &densities.bindgroup_layout, &u);
    let c = BGKCollision::new(&driver.device, &densities, &macros, &u, c_params); 

    for i in 1..t {
        println!("t: {}", i);
        let encoder_label = "read_encoder";
        let mut encoder = driver
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(encoder_label),
            });
        s.stream(group_n, &mut encoder, &mut densities);
        c.collide(group_n, &mut encoder, &mut densities, &macros); 
        macros.compute(group_n, &mut encoder, &densities);
        let submission = driver.queue.submit(Some(encoder.finish()));
        driver
            .device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));

        let mut b = VTKBuilder2D::new(&d);
        b.add_densities(&driver, &densities);
        b.add_macros(&driver, &macros);
        b.export(&format!("vtu_test/frame_{:03}.vtu", i));
    }
}
