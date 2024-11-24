use cm_lbm::dimensions::*;
use cm_lbm::io::*;
use cm_lbm::kernel::*;
use cm_lbm::wgpu_util;

#[tokio::main]
async fn main() {
    let driver = wgpu_util::setup_wgpu().await;

    let d = LatticeDimensions {
        rows: 12,
        cols: 12,
        total: 12 * 12,
        q: 9,
        size: 1.0,
    };

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
    macros.compute([2, 2, 1], &mut encoder, &densities);
    let submission = driver.queue.submit(Some(encoder.finish()));
    driver
        .device
        .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));
    let mut b = VTKBuilder2D::new(&d);
    b.add_densities(&driver, &densities);
    b.add_pressure(&driver, &macros);
    b.export("vtu_test/frame_0.vtu");

    let s = Stream2D::new(&driver.device, &densities.bindgroup_layout, &u);

    for i in 1..13 {
        let encoder_label = "read_encoder";
        let mut encoder = driver
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(encoder_label),
            });
        s.stream([2, 2, 1], &mut encoder, &mut densities);
        macros.compute([2, 2, 1], &mut encoder, &densities);
        let submission = driver.queue.submit(Some(encoder.finish()));
        driver
            .device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));

        let mut b = VTKBuilder2D::new(&d);
        b.add_densities(&driver, &densities);
        b.add_pressure(&driver, &macros);
        b.export(&format!("vtu_test/frame_{:03}.vtu", i));
    }
}
