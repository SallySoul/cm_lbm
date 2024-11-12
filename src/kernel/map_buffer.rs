use crate::wgpu_util::Driver;

pub struct MapBuffer {
    map_buffer: std::sync::Arc<wgpu::Buffer>,
    size_bytes: u64,
}

impl MapBuffer {
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

        MapBuffer {
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
            .map_async(wgpu::MapMode::Read, move |result| {
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

        // Make Densities

        // Make Map Buffer

        // Write and Read
    }
}
