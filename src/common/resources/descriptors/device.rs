//! Device related structures and enumerations.

use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::InstanceId;

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [DeviceHandle][crate::common::resources::handles::DeviceHandle]
*/
pub struct DeviceDescriptor {
    pub label: String,
    pub instance: InstanceId,
    pub backend: crate::wgpu::BackendBit,
    pub pci_id: usize,
    pub features: crate::wgpu::Features,
    pub limits: crate::wgpu::Limits,
}
impl HaveDependencies for DeviceDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.instance.id_ref().clone()]
    }
}
impl HaveDescriptor for DeviceDescriptor {
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
