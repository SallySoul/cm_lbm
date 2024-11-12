use crate::wgpu_util::Driver;

pub struct ReadMapBuffer {
    map_buffer: std::sync::Arc<wgpu::Buffer>,
    size_bytes: u64,
}

impl ReadMapBuffer {
    pub fn new(device: &wgpu::Device, dimensions: &crate::LatticeDimensions) -> Self {
        let size_bytes = dimensions.float_buffer_byte_size();
        let map_buffer_label = "data_map";
        let map_buffer: std::sync::Arc<wgpu::Buffer> =
            std::sync::Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(map_buffer_label),
                size: size_bytes,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));

        ReadMapBuffer {
            map_buffer,
            size_bytes,
        }
    }

    pub fn read_data<F: FnOnce(&[f32]) + Sync + Send + 'static>(
        &self,
        driver: &Driver,
        buffer: &wgpu::Buffer,
        f: F,
    ) {
        let encoder_label = "read_encoder";
        let mut encoder = driver
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(encoder_label),
            });

        encoder.copy_buffer_to_buffer(buffer, 0, &self.map_buffer, 0, self.size_bytes);

        let submission = driver.queue.submit(Some(encoder.finish()));

        driver
            .device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));

        let capturable = self.map_buffer.clone();
        self.map_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                result.expect("READ FAILED");
                let view = capturable.slice(..).get_mapped_range();
                let data_view: &[f32] = bytemuck::cast_slice(&view);
                f(data_view);
                drop(view);
                capturable.unmap();
            });

        driver.device.poll(wgpu::Maintain::Wait);
    }
}

pub struct WriteMapBuffer {
    map_buffer: std::sync::Arc<wgpu::Buffer>,
    size_bytes: u64,
}

impl WriteMapBuffer {
    pub fn new(device: &wgpu::Device, dimensions: &crate::LatticeDimensions) -> Self {
        let size_bytes = dimensions.float_buffer_byte_size();
        let map_buffer_label = "data_map";
        let map_buffer: std::sync::Arc<wgpu::Buffer> =
            std::sync::Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(map_buffer_label),
                size: size_bytes,
                usage: wgpu::BufferUsages::MAP_WRITE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }));

        WriteMapBuffer {
            map_buffer,
            size_bytes,
        }
    }

    pub fn write_data<F: FnOnce(&mut [f32]) + Sync + Send + 'static>(
        &self,
        driver: &Driver,
        buffer: &wgpu::Buffer,
        f: F,
    ) {
        let encoder_label = "write_encoder";
        let mut encoder = driver
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(encoder_label),
            });

        let capturable = self.map_buffer.clone();
        self.map_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Write, move |result| {
                result.expect("READ FAILED");
                let mut view = capturable.slice(..).get_mapped_range_mut();
                let data_view: &mut [f32] = bytemuck::cast_slice_mut(&mut view);
                f(data_view);
                drop(view);
                capturable.unmap();
            });

        driver.device.poll(wgpu::Maintain::Wait);

        encoder.copy_buffer_to_buffer(&self.map_buffer, 0, buffer, 0, self.size_bytes);

        let submission = driver.queue.submit(Some(encoder.finish()));

        driver
            .device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::wgpu_util::*;

    #[tokio::test]
    async fn map_buffer_test() {
        // Make Dimensions
        let driver = crate::setup_wgpu().await;

        // Dimensions
        let rows = 10;
        let cols = 10;
        let q = 9;
        let lattice_dimensions = crate::LatticeDimensions {
            rows,
            cols,
            total: rows * cols,
            q,
        };
        assert_eq!(lattice_dimensions.total, 100);

        // Make Densities
        let densities = crate::kernel::Densities::new(&driver.device, &lattice_dimensions);

        let write_map = WriteMapBuffer::new(&driver.device, &lattice_dimensions);
        let read_map = ReadMapBuffer::new(&driver.device, &lattice_dimensions);

        // Write and Read
        write_map.write_data(&driver, &densities.input_buffer, |slice| {
            assert_eq!(slice.len(), 100);
            for (i, x) in slice.iter_mut().enumerate() {
                *x = i as f32;
            }
        });

        read_map.read_data(&driver, &densities.input_buffer, |slice| {
            assert_eq!(slice.len(), 100);
            for (i, x) in slice.iter().enumerate() {
                assert_eq!(*x, i as f32);
            }
        });

        let mut encoder = driver
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("test_copy"),
            });

        encoder.copy_buffer_to_buffer(
            &densities.input_buffer,
            0,
            &densities.output_buffer,
            0,
            lattice_dimensions.float_buffer_byte_size(),
        );

        let submission = driver.queue.submit(Some(encoder.finish()));

        driver
            .device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));

        read_map.read_data(&driver, &densities.output_buffer, |slice| {
            assert_eq!(slice.len(), 100);
            for (i, x) in slice.iter().enumerate() {
                assert_eq!(*x, i as f32);
            }
        });
    }
}
