use crate::entity_manager::UpdateContext;
use crate::*;
use bytemuck::{Pod, Zeroable};
use std::path::PathBuf;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Rectangle {
    position: [f32; 3],
    size: [f32; 2],
    image_index: u32,
}

#[derive(Debug)]
pub enum SurfaceSource {
    File { path: PathBuf },
    Dmabuf { fd: std::fs::File },
}

#[derive(Debug)]
pub struct RectangleInfo {
    texture_id: EntityId,
    texture_view_id: EntityId,

    source: SurfaceSource,
    position: [f32; 3],
    size: [f32; 2],
}
impl RectangleInfo {
    pub fn new(
        texture_id: EntityId,
        texture_view_id: EntityId,
        source: SurfaceSource,
        position: [u32; 3],
        size: [u32; 2],
    ) -> Self {
        let position = [position[0] as f32, position[1] as f32, position[2] as f32];
        let size = [size[0] as f32, size[1] as f32];
        Self {
            source,
            position,
            size,
            texture_id,
            texture_view_id,
        }
    }

    pub fn generate_data(&self, image_index: u32) -> Rectangle {
        Rectangle {
            position: self.position,
            size: self.size,
            image_index,
        }
    }
}

#[derive(Debug)]
pub struct RectangleManager {
    rectangle_id_counter: usize,
    rectangle_stack: Vec<usize>,
    rectangle_data_buffer: BufferManager<Rectangle, RectangleInfo>,
}
impl RectangleManager {
    pub fn new(update_context: &mut UpdateContext) -> Self {
        let rectangle_id_counter = 0;
        let rectangle_stack = Vec::new();
        let rectangle_data_buffer = BufferManager::new(
            update_context,
            String::from("RectangleManager buffer"),
            32,
            crate::wgpu::BufferUsage::VERTEX,
        );
        Self {
            rectangle_id_counter,
            rectangle_stack,
            rectangle_data_buffer,
        }
    }

    pub fn buffer_id(&self) -> &EntityId {
        self.rectangle_data_buffer.id()
    }

    pub fn book_id(&mut self) -> usize {
        let id = self.rectangle_id_counter;
        self.rectangle_id_counter += 1;
        id
    }

    pub fn len(&self) -> usize {
        self.rectangle_stack.len()
    }

    pub fn create_surface(
        &mut self,
        update_context: &mut UpdateContext,
        label: String,
        id: usize,
        source: SurfaceSource,
        position: [u32; 3],
        size: [u32; 2],
    ) {
        let width;
        let height;
        let depth_or_array_layers;
        let sample_layout;
        let data;
        match &source {
            SurfaceSource::File { path } => {
                use image::io::Reader as ImageReader;
                let img = ImageReader::open(path.clone())
                    .unwrap()
                    .decode()
                    .unwrap()
                    .into_rgba8();
                width = img.dimensions().0;
                height = img.dimensions().1;
                depth_or_array_layers = 1;
                sample_layout = img.sample_layout();
                data = img.into_raw();
            }
            _ => panic!(),
        }

        let texture_descriptor = TextureDescriptor {
            label: label.clone() + " texture",
            source: TextureSource::Local,
            size: crate::wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: crate::wgpu::TextureDimension::D2,
            format: crate::wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: crate::wgpu::TextureUsage::SAMPLED | crate::wgpu::TextureUsage::COPY_DST,
        };
        let texture_id = update_context.add_resource_descriptor(texture_descriptor).unwrap();

        let texture_view_descriptor = TextureViewDescriptor {
            label: label + " texture view",
            texture: texture_id,
            format: crate::wgpu::TextureFormat::Rgba8UnormSrgb,
            dimension: crate::wgpu::TextureViewDimension::D2,
            aspect: crate::wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        };
        let texture_view_id = update_context
            .add_resource_descriptor(texture_view_descriptor)
            .unwrap();

        let resource_write = ResourceWrite::Texture(TextureWrite {
            texture: texture_id,
            mip_level: 0,
            origin: crate::wgpu::Origin3d::ZERO,
            data,
            layout: crate::wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(
                    sample_layout.width * sample_layout.channels as u32 * 1,
                ),
                rows_per_image: std::num::NonZeroU32::new(sample_layout.height),
            },
            size: crate::wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers,
            },
        });
        update_context.write_resource(&mut vec![resource_write]);

        let surface = RectangleInfo::new(texture_id, texture_view_id, source, position, size);
        let surface_data = surface.generate_data(self.rectangle_data_buffer.next_slot() as u32);
        self.rectangle_data_buffer
            .request(id, surface, surface_data);
        self.rectangle_stack.push(id);
    }

    pub fn resize_surface(&mut self, id: &usize, size: [u32; 2]) -> bool {
        let size = [size[0] as f32, size[1] as f32];
        let offset = field_offset::offset_of!(Rectangle => size);
        self.rectangle_data_buffer
            .pending_write_field(id, offset, size)
    }

    pub fn move_surface(&mut self, id: &usize, position: [u32; 3]) -> bool {
        let position = [position[0] as f32, position[1] as f32, position[2] as f32];
        let offset = field_offset::offset_of!(Rectangle => position);
        self.rectangle_data_buffer
            .pending_write_field(id, offset, position)
    }

    pub fn remove_surface(&mut self, id: &usize) -> bool {
        self.rectangle_data_buffer
            .release_pending(id)
            .map(|_associated_data| {
                let index = self
                    .rectangle_stack
                    .iter()
                    .position(|current_id| current_id == id)
                    .unwrap();
                self.rectangle_stack.swap_remove(index);
                Some(())
            })
            .is_some()
    }

    pub fn rectangle_views(&self) -> Vec<EntityId> {
        self.rectangle_stack
            .iter()
            .map(|id| {
                self.rectangle_data_buffer
                    .associated_data(id)
                    .unwrap()
                    .texture_view_id
            })
            .collect()
    }

    pub fn update(&mut self, update_context: &mut UpdateContext) -> Vec<Command> {
        self.rectangle_data_buffer.update(update_context)
    }
}
