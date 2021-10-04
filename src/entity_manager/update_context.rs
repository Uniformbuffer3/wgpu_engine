use crate::common::*;
use crate::engine::resource_manager::ResourceManager;

macro_rules! make_update_context_functions {
    ($($name: ident),*) => {
        paste::paste! {
            $(
                pub fn [<$name:snake s>](&self)->impl Iterator<Item = [<$name:camel Id>]> + '_{
                    self.resource_manager.[<$name:snake s>]()
                }
                pub fn [<$name:snake _descriptor_ref>](&self, id: &[<$name:camel Id>]) -> Option<&[<$name:camel Descriptor>]> {
                    match self.resource_manager.entity_descriptor_ref(id.id_ref()) {
                        Some(ResourceDescriptor::[<$name:camel>](descriptor)) => Some(descriptor),
                        _ => None,
                    }
                }
                pub(crate) fn [<$name:snake _handle_ref>](&self, id: &[<$name:camel Id>]) -> Option<&[<$name:camel Handle>]> {
                    match self.resource_manager.entity_handle_ref(id.id_ref()) {
                        Some(Some(ResourceHandle::[<$name:camel>](handle))) => Some(handle),
                        _ => None,
                    }
                }
                pub fn [<add_ $name:snake _descriptor>](
                    &mut self,
                    descriptor: impl Into<[<$name:camel Descriptor>]>,
                ) -> Result<[<$name:camel Id>], ()> {
                    self.[<add_ $name:snake>](descriptor,None)
                }

                pub(crate) fn [<add_ $name:snake>](
                    &mut self,
                    descriptor: impl Into<[<$name:camel Descriptor>]>,
                    handle: impl Into<Option<[<$name:camel Handle>]>>
                ) -> Result<[<$name:camel Id>], ()> {
                    self.resource_manager.[<add_ $name:snake>](
                        self.task,
                        descriptor.into(),handle.into().map(|handle|handle.into()),
                    )
                }

                pub fn [<update_ $name:snake _descriptor>](
                    &mut self,
                    id: &mut [<$name:camel Id>],
                    descriptor: impl Into<[<$name:camel Descriptor>]>,
                ) -> bool {
                    self.resource_manager.[<update_ $name:snake _descriptor>](id,descriptor)
                }
                /*
                pub fn [<update_ $name:snake _descriptor_mut>]<T>(
                    &mut self,
                    id: &mut [<$name:camel Id>],
                    callback: impl FnOnce(&mut [<$name:camel Descriptor>])->T,
                ) -> Option<T> {
                    self.resource_manager.[<update_ $name:snake _descriptor_mut>](id,|descriptor|callback(descriptor))
                }
                */
                pub fn [<remove_ $name:snake>](&mut self, id: &[<$name:camel Id>]) -> Result<(), ()> {
                    self.resource_manager.[<remove_ $name:snake>](id)
                }
            )*
        }
    };
}

pub struct UpdateContext<'a> {
    task: TaskId,
    resource_manager: &'a mut ResourceManager,
    resource_writes: Vec<ResourceWrite>,
    events: &'a mut Vec<ResourceEvent>,
}
impl<'a> UpdateContext<'a> {
    pub fn new(
        task: TaskId,
        resource_manager: &'a mut ResourceManager,
        events: &'a mut Vec<ResourceEvent>,
    ) -> Self {
        Self {
            task,
            resource_manager,
            resource_writes: Vec::new(),
            events,
        }
    }

    pub fn is_damaged(&self, id: &EntityId)->bool {
        self.resource_manager.is_damaged(id)
    }

    pub fn entity_device_id(&self, id: impl AsRef<EntityId>) -> Option<DeviceId> {
        self.resource_manager.entity_device_id(id)
    }

    make_update_context_functions!(
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

    pub fn write_resource(&mut self, writes: &mut Vec<ResourceWrite>) {
        self.resource_writes.append(writes);
    }
    pub fn events(&self) -> &Vec<ResourceEvent> {
        self.events
    }
    pub(crate) fn push_event(&mut self, event: ResourceEvent) {

        if let Some(true) = self.events.last().map(|last|last==&event){}
        else {
            self.events.push(event);
        }

        //if self.events.iter().find(|current_event|*current_event==&event).is_none(){}
    }

    pub(crate) fn into_resource_writes(self) -> Vec<ResourceWrite> {
        self.resource_writes
    }
}
