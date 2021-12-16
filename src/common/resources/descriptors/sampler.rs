use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::DeviceId;

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [SamplerHandle][crate::common::resources::handles::SamplerHandle]
*/
pub struct SamplerDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub address_mode_u: crate::wgpu::AddressMode,
    pub address_mode_v: crate::wgpu::AddressMode,
    pub address_mode_w: crate::wgpu::AddressMode,
    pub mag_filter: crate::wgpu::FilterMode,
    pub min_filter: crate::wgpu::FilterMode,
    pub mipmap_filter: crate::wgpu::FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<crate::wgpu::CompareFunction>,
    pub anisotropy_clamp: Option<std::num::NonZeroU8>,
    pub border_color: Option<crate::wgpu::SamplerBorderColor>,
}
impl HaveDependencies for SamplerDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}
impl HaveDescriptor for SamplerDescriptor {
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
