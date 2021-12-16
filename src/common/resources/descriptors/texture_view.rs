use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::{DeviceId, TextureId};

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [TextureViewHandle][crate::common::resources::handles::TextureViewHandle]
*/
pub struct TextureViewDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub texture: TextureId,
    pub format: crate::wgpu::TextureFormat,
    pub dimension: crate::wgpu::TextureViewDimension,
    pub aspect: crate::wgpu::TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<std::num::NonZeroU32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<std::num::NonZeroU32>,
}
impl HaveDependencies for TextureViewDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
            .chain(std::iter::once(*self.texture.id_ref()))
            .collect()
    }
}
impl HaveDescriptor for TextureViewDescriptor {
    type D = Self;
    fn descriptor(&self) -> Self::D {
        self.clone()
    }
    fn descriptor_ref(&self) -> &Self::D {
        self
    }
    fn descriptor_mut(&mut self) -> &mut Self::D {
        self
    }
    fn state_type(&self) -> StateType {
        StateType::Stateless
    }
    fn needs_update(&self, _other: &Self::D) -> bool {
        true
    }
}
