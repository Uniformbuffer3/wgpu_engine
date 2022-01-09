//! ShaderModule related structures and enumerations.

use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::DeviceId;

#[derive(Debug, Clone, PartialEq)]
pub enum ShaderSource {
    SpirV(Vec<u32>),
    Wgsl(String),
}

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [ShaderModuleHandle][crate::common::resources::handles::ShaderModuleHandle]
*/
pub struct ShaderModuleDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub source: ShaderSource,
    pub flags: crate::wgpu::ShaderFlags,
}
impl HaveDependencies for ShaderModuleDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}
impl HaveDescriptor for ShaderModuleDescriptor {
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
