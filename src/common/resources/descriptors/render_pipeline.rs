use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::{DeviceId, PipelineLayoutId, ShaderModuleId, SwapchainId, TextureViewId};

#[derive(Debug, Clone, PartialEq)]
pub struct VertexBufferLayout {
    pub array_stride: crate::wgpu::BufferAddress,
    pub step_mode: crate::wgpu::InputStepMode,
    pub attributes: Vec<crate::wgpu::VertexAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VertexState {
    pub module: ShaderModuleId, //Arc<crate::wgpu::ShaderModule>
    pub entry_point: String,
    pub buffers: Vec<VertexBufferLayout>,
}
impl HaveDependencies for VertexState {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.module.id_ref().clone()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorTarget {
    Swapchain(SwapchainId),
    TextureView(TextureViewId),
}
impl HaveDependencies for ColorTarget {
    fn dependencies(&self) -> Vec<EntityId> {
        match self {
            Self::Swapchain(id) => vec![id.id_ref().clone()],
            Self::TextureView(id) => vec![id.id_ref().clone()],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorTargetState {
    pub target: ColorTarget,
    pub blend: Option<crate::wgpu::BlendState>,
    pub write_mask: crate::wgpu::ColorWrite,
}
impl HaveDependencies for ColorTargetState {
    fn dependencies(&self) -> Vec<EntityId> {
        self.target.dependencies()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentState {
    pub module: ShaderModuleId, //Arc<crate::wgpu::ShaderModule>
    pub entry_point: String,
    pub targets: Vec<crate::wgpu::ColorTargetState>,
}
impl HaveDependencies for FragmentState {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.module.id_ref().clone()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DepthStencilState {
    pub id: TextureViewId,
    pub depth_write_enabled: bool,
    pub depth_compare: crate::wgpu::CompareFunction,
    pub stencil: crate::wgpu::StencilState,
    pub bias: crate::wgpu::DepthBiasState,
}

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [RenderPipelineHandle][crate::common::resources::handles::RenderPipelineHandle]
*/
pub struct RenderPipelineDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub layout: Option<PipelineLayoutId>, //Arc<crate::wgpu::PipelineLayout>
    pub vertex: VertexState,
    pub primitive: crate::wgpu::PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: crate::wgpu::MultisampleState,
    pub fragment: Option<FragmentState>,
}
impl HaveDependencies for RenderPipelineDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
            .chain(self.layout.map(|id| id.id_ref().clone()))
            .chain(self.vertex.dependencies())
            .chain(
                self.fragment
                    .iter()
                    .map(|fragment| fragment.dependencies())
                    .flatten(),
            )
            .collect()
    }
}
impl HaveDescriptor for RenderPipelineDescriptor {
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
