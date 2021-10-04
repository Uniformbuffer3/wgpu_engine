use downcast_rs::{impl_downcast, Downcast};

pub mod descriptor;
pub use descriptor::*;

use crate::common::*;
pub use crate::entity_manager::UpdateContext;

make_id!(Task);

/// The task template contains the required and optional features and limit of the task.
/// It also contains the render and compute task to drive the command buffer logic.
pub trait TaskTrait: Downcast + Send + Sync {
    fn name(&self) -> String;

    fn update(&mut self) {}

    fn update_resources(&mut self, _update_context: &mut UpdateContext) {}
    fn command_buffers(&self) -> Vec<CommandBufferId> {
        Vec::new()
    }

    //fn features_and_limits()->(wgpu::Features,wgpu::Limits) {(wgpu::Features::default(),wgpu::Limits::default())}
}
impl_downcast!(TaskTrait);

pub type TaskHandle = Box<dyn TaskTrait + 'static>;

pub struct Task {
    descriptor: TaskDescriptor,
    handle: Option<TaskHandle>,
}

impl HaveDependencies for Task {
    fn dependencies(&self) -> Vec<EntityId> {
        self.descriptor.dependencies()
    }
}
impl HaveDescriptor for Task {
    type D = TaskDescriptor;
    fn descriptor(&self) -> Self::D {
        self.descriptor.clone()
    }
    fn descriptor_ref(&self) -> &Self::D {
        &self.descriptor
    }
    fn descriptor_mut(&mut self) -> &mut Self::D {
        &mut self.descriptor
    }
    fn needs_update(&self, other: &Self::D) -> bool {
        self.descriptor.needs_update(other)
    }
}

impl HaveHandle for Task {
    type H = Option<TaskHandle>;
    fn handle_ref(&self) -> &Self::H {
        &self.handle
    }
    fn handle_mut(&mut self) -> &mut Self::H {
        &mut self.handle
    }
}
impl HaveDescriptorAndHandle for Task {}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task `{}`", self.descriptor.name)
    }
}

impl From<(TaskDescriptor, Option<TaskHandle>)> for Task {
    fn from(descriptor: (TaskDescriptor, Option<TaskHandle>)) -> Self {
        Self {
            descriptor: descriptor.0,
            handle: descriptor.1,
        }
    }
}
