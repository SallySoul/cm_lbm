use crate::*;
use std::sync::{Arc, Mutex};

pub struct ReadMapBuffer {
    map_buffer: std::sync::Arc<wgpu::Buffer>,
    size_bytes: u64,
}

impl ReadMapBuffer {
    pub fn new(device: &wgpu::Device, grid_dimensions: &AABB3, components: usize) -> Self {
        let size_bytes =
            (components * box_buffer_size(grid_dimensions) * std::mem::size_of::<f32>()) as u64;
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

    pub fn clone_data(&self, driver: &Driver, buffer: &wgpu::Buffer) -> Vec<f32> {
        let data_buffer = Arc::new(Mutex::new(Vec::new()));
        let capturable = data_buffer.clone();
        self.read_data(driver, buffer, move |slice| {
            *capturable.lock().unwrap() = slice.to_vec();
        });
        let result: Vec<f32> = data_buffer.lock().unwrap().clone();
        result
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
    pub fn new(device: &wgpu::Device, grid_dimensions: &AABB3, components: usize) -> Self {
        let size_bytes =
            (components * box_buffer_size(grid_dimensions) * std::mem::size_of::<f32>()) as u64;
        let map_buffer_label = "write_data_map";
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

/*
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
        let size = 1.0;
        let lattice_dimensions = crate::LatticeDimensions {
            rows,
            cols,
            total: rows * cols,
            q,
            size,
        };
        assert_eq!(lattice_dimensions.total, 100);

        // Make Densities
        let densities = crate::kernel::Densities::new(&driver.device, &lattice_dimensions);

        let density_read_map = ReadMapBuffer::new(&driver.device, &lattice_dimensions, 1);
        let velocity_read_map = ReadMapBuffer::new(&driver.device, &lattice_dimensions, 1);


        // Write and Read
        write_map.write_data(&driver, &densities.input_buffers[0], |slice| {
            assert_eq!(slice.len(), 100);
            for (i, x) in slice.iter_mut().enumerate() {
                *x = i as f32;
            }
        });

        read_map.read_data(&driver, &densities.input_buffers[0], |slice| {
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
            &densities.input_buffers[0],
            0,
            &densities.output_buffers[0],
            0,
            lattice_dimensions.float_buffer_byte_size(),
        );

        let submission = driver.queue.submit(Some(encoder.finish()));

        driver
            .device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission));

        read_map.read_data(&driver, &densities.output_buffers[0], |slice| {
            assert_eq!(slice.len(), 100);
            for (i, x) in slice.iter().enumerate() {
                assert_eq!(*x, i as f32);
            }
        });
    }
}
*/
