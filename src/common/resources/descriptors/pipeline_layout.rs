use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::{BindGroupLayoutId, DeviceId};

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [PipelineLayoutHandle][crate::common::resources::handles::PipelineLayoutHandle]
*/
pub struct PipelineLayoutDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub bind_group_layouts: Vec<BindGroupLayoutId>, //Arc<crate::wgpu::BindGroupLayout>
    pub push_constant_ranges: Vec<crate::wgpu::PushConstantRange>,
}
impl HaveDependencies for PipelineLayoutDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
            .chain(self.bind_group_layouts.iter().map(|id| id.id_ref().clone()))
            .collect()
    }
}
impl HaveDescriptor for PipelineLayoutDescriptor {
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
