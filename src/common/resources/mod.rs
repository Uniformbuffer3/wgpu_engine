pub mod descriptors;
pub use descriptors::*;

pub mod handles;
pub use handles::*;

pub mod builders;
pub use builders::*;

pub use crate::EntityId;
pub use crate::TaskId;

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
/**
A combination of owners, a descriptor and a handle.
*/
pub struct Resource {
    owners: Vec<TaskId>,
    current_descriptor: ResourceDescriptor,
    current_handle: Option<ResourceHandle>,
}
impl Resource {
    pub fn new(
        owners: Vec<TaskId>,
        descriptor: ResourceDescriptor,
        handle: Option<ResourceHandle>,
    ) -> Self {
        Self {
            owners,
            current_descriptor: descriptor,
            current_handle: handle,
        }
    }
}

impl HaveOwners for Resource {
    type O = TaskId;
    fn owners(&self) -> Vec<TaskId> {
        self.owners.clone()
    }
    fn owners_ref(&self) -> &Vec<TaskId> {
        &self.owners
    }
    fn owners_mut(&mut self) -> &mut Vec<TaskId> {
        &mut self.owners
    }
}
impl HaveDependencies for Resource {
    fn dependencies(&self) -> Vec<EntityId> {
        self.current_descriptor.dependencies()
    }
}
impl HaveDescriptor for Resource {
    type D = ResourceDescriptor;
    fn descriptor(&self) -> Self::D {
        self.current_descriptor.clone()
    }
    fn descriptor_ref(&self) -> &Self::D {
        &self.current_descriptor
    }
    fn descriptor_mut(&mut self) -> &mut Self::D {
        &mut self.current_descriptor
    }
    fn state_type(&self) -> StateType {
        self.current_descriptor.state_type()
    }
    fn needs_update(&self, other: &Self::D) -> bool {
        self.current_descriptor.needs_update(other)
    }
}

impl HaveHandle for Resource {
    type H = Option<ResourceHandle>;
    fn handle_ref(&self) -> &Self::H {
        &self.current_handle
    }
    fn handle_mut(&mut self) -> &mut Self::H {
        &mut self.current_handle
    }
}
impl HaveDescriptorAndHandle for Resource {}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.current_descriptor {
            ResourceDescriptor::Instance(descriptor) => {
                write!(f, "Instance `{}`", descriptor.label)
            }
            ResourceDescriptor::Device(descriptor) => {
                write!(f, "Device `{}`", descriptor.label)
            }
            ResourceDescriptor::Swapchain(descriptor) => {
                write!(f, "Swapchain `{}`", descriptor.label)
            }
            ResourceDescriptor::Buffer(descriptor) => {
                write!(f, "Buffer `{}`", descriptor.label)
            }
            ResourceDescriptor::Texture(descriptor) => {
                write!(f, "Texture  `{}`", descriptor.label)
            }
            ResourceDescriptor::TextureView(descriptor) => {
                write!(f, "TextureView `{}`", descriptor.label)
            }
            ResourceDescriptor::Sampler(descriptor) => {
                write!(f, "Sampler `{}`", descriptor.label)
            }
            ResourceDescriptor::ShaderModule(descriptor) => {
                write!(f, "ShaderModule `{}`", descriptor.label)
            }

            ResourceDescriptor::BindGroupLayout(descriptor) => {
                write!(f, "BindGroupLayout `{}`", descriptor.label)
            }
            ResourceDescriptor::BindGroup(descriptor) => {
                write!(f, "BindGroup `{}`", descriptor.label)
            }

            ResourceDescriptor::PipelineLayout(descriptor) => {
                write!(f, "PipelineLayout `{}`", descriptor.label)
            }
            ResourceDescriptor::RenderPipeline(descriptor) => {
                write!(f, "RenderPipeline `{}`", descriptor.label)
            }
            ResourceDescriptor::ComputePipeline(descriptor) => {
                write!(f, "ComputePipeline `{}`", descriptor.label)
            }
            ResourceDescriptor::CommandBuffer(descriptor) => {
                write!(f, "CommandBuffer `{}`", descriptor.label)
            }
        }
    }
}

macro_rules! make_resource_ids {
    ($($name: ident),*) => {
        paste::paste! {
            crate::common::make_id![$($name),*];

            #[derive(Clone,Copy,PartialEq)]
            pub enum ResourceId {
                $(
                    [<$name:camel>]([<$name:camel Id>]),
                )*
            }

            impl From<&ResourceIdMut<'_>> for ResourceId {
                fn from(id: &ResourceIdMut<'_>)->Self {
                    match id {
                        $(
                        ResourceIdMut::[<$name:camel>](id)=>Self::[<$name:camel>](**id),
                        )*
                    }
                }
            }

            $(
            impl From<[<$name:camel Id>]> for ResourceId {
                fn from(id: [<$name:camel Id>])->Self {
                    Self::[<$name:camel>](id)
                }
            }
            impl std::convert::TryInto<[<$name:camel Id>]> for ResourceId {
                type Error = ();
                fn try_into(self) -> Result<[<$name:camel Id>], Self::Error> {
                    if let Self::[<$name:camel>](id) = self {Ok(id)}
                    else{Err(())}
                }
            }
            )*

            impl Into<EntityId> for ResourceId {
                fn into(self)->EntityId {
                    match self {
                        $(
                        Self::[<$name:camel>](id)=>*id.id_ref(),
                        )*
                    }
                }
            }


            #[derive(Clone,Copy,PartialEq)]
            pub enum ResourceIdRef<'a> {
                $(
                    [<$name:camel>](&'a [<$name:camel Id>]),
                )*
            }

            $(
            impl<'a> From<&'a [<$name:camel Id>]> for ResourceIdRef<'a> {
                fn from(id: &'a [<$name:camel Id>])->Self {
                    Self::[<$name:camel>](id)
                }
            }
            impl<'a> std::convert::TryInto<&'a [<$name:camel Id>]> for ResourceIdRef<'a> {
                type Error = ();
                fn try_into(self) -> Result<&'a [<$name:camel Id>], Self::Error> {
                    if let Self::[<$name:camel>](id) = self {Ok(id)}
                    else{Err(())}
                }
            }
            )*

            impl<'a> Into<EntityId> for ResourceIdRef<'a> {
                fn into(self)->EntityId {
                    match self {
                        $(
                        Self::[<$name:camel>](id)=>*id.id_ref(),
                        )*
                    }
                }
            }

            #[derive(PartialEq)]
            pub enum ResourceIdMut<'a> {
                $(
                    [<$name:camel>](&'a mut [<$name:camel Id>]),
                )*
            }

            $(
            impl<'a> From<&'a mut [<$name:camel Id>]> for ResourceIdMut<'a> {
                fn from(id: &'a mut [<$name:camel Id>])->Self {
                    Self::[<$name:camel>](id)
                }
            }
            impl<'a> std::convert::TryInto<&'a mut [<$name:camel Id>]> for ResourceIdMut<'a> {
                type Error = ();
                fn try_into(self) -> Result<&'a mut [<$name:camel Id>], Self::Error> {
                    if let Self::[<$name:camel>](id) = self {Ok(id)}
                    else{Err(())}
                }
            }
            )*

            impl<'a> Into<EntityId> for ResourceIdMut<'a> {
                fn into(self)->EntityId {
                    match self {
                        $(
                        Self::[<$name:camel>](id)=>*id.id_ref(),
                        )*
                    }
                }
            }

            impl<'a> Deref for ResourceIdMut<'a> {
                type Target = EntityId;

                fn deref(&self) -> &Self::Target {
                    match self {
                        $(
                        Self::[<$name:camel>](id)=>id.id_ref(),
                        )*
                    }
                }
            }

            impl<'a> DerefMut for ResourceIdMut<'a> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    match self {
                        $(
                        Self::[<$name:camel>](id)=>id.id_mut(),
                        )*
                    }
                }
            }

        }
    }
}

make_resource_ids!(
    Instance,
    Device,
    Swapchain,
    Buffer,
    Texture,
    TextureView,
    Sampler,
    ShaderModule,
    BindGroupLayout,
    BindGroup,
    PipelineLayout,
    RenderPipeline,
    ComputePipeline,
    CommandBuffer
);

pub enum ResourceType {
    Instance,
    Device,
    Swapchain,
    Buffer,
    Texture,
    TextureView,
    Sampler,
    ShaderModule,
    BindGroupLayout,
    BindGroup,
    PipelineLayout,
    RenderPipeline,
    ComputePipeline,
    CommandBuffer,
}

/*
pub enum ResourceId {
    Instance(InstanceId),
    Device(DeviceId),
    Swapchain(SwapchainId),

    Buffer(BufferId),
    Texture(TextureId),
    TextureView(TextureViewId),
    Sampler(SamplerId),
    ShaderModule(ShaderModuleId),

    BindGroupLayout(BindGroupLayoutId),
    BindGroup(BindGroupId),

    PipelineLayout(PipelineLayoutId),
    RenderPipeline(RenderPipelineId),
    ComputePipeline(ComputePipelineId),
    CommandBuffer(CommandBufferId),
}*/
