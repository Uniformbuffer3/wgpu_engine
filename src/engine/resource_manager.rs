use crate::common::*;
use crate::entity_manager::DMGEntityManager;

use petgraph::visit::Topo;

use std::collections::HashSet;
use std::convert::TryInto;
use std::sync::Arc;

macro_rules! make_resource_functions {
    ($name: ident) => {
        paste::paste! {
            pub(crate) fn [<$name:snake s>](&self)->impl Iterator<Item = [<$name:camel Id>]> + '_{
                self.inner.entities().filter_map(move |id|{
                    let id = [<$name:camel Id>]::new(id);
                    if self.[<$name:snake _descriptor_ref>](&id).is_some(){Some(id)}
                    else{None}
                })
            }

            pub(crate) fn [<$name:snake _descriptor_ref>](&self, id: &[<$name:camel Id>]) -> Option<&[<$name:camel Descriptor>]> {
                match self.entity_descriptor_ref(id.id_ref()) {
                    Some(ResourceDescriptor::[<$name:camel>](descriptor)) => Some(descriptor),
                    _ => None,
                }
            }
            pub(crate) fn [<$name:snake _handle_ref>](&self, id: &[<$name:camel Id>]) -> Option<&[<$name:camel Handle>]> {
                match self.entity_handle_ref(id.id_ref()) {
                    Some(Some(ResourceHandle::[<$name:camel>](handle))) => Some(handle),
                    _ => None,
                }
            }
            /*
            pub fn [<add_ $name:snake _descriptor>](
                &mut self,
                device: Option<&DeviceId>,
                descriptor: impl Into<[<$name:camel Descriptor>]>,
            ) -> Result<[<$name:camel Id>], ()> {
                self.add_resource_descriptor(device,descriptor.into()).map(|id|[<$name:camel Id>]::new(id))
            }
            */
            pub fn [<add_ $name:snake>](
                &mut self,
                task: TaskId,
                descriptor: impl Into<[<$name:camel Descriptor>]>,
                handle: impl Into<Option<[<$name:camel Handle>]>>,
            ) -> Result<[<$name:camel Id>], ()> {
                self.add_resource(task,descriptor.into(),handle.into().map(|handle|handle.into())).map(|id|[<$name:camel Id>]::new(id.try_into().unwrap()))
            }

            pub(crate) fn [<update_ $name:snake _descriptor>](
                &mut self,
                task: &TaskId,
                id: &mut [<$name:camel Id>],
                descriptor: impl Into<[<$name:camel Descriptor>]>,
            ) -> bool {
                let id: ResourceIdMut = id.into();
                self.update_resource_descriptor(task,id,descriptor.into())
            }
            /*
            pub(crate) fn [<update_ $name:snake _descriptor_mut>]<T>(
                &mut self,
                id: &mut [<$name:camel Id>],
                callback: impl FnOnce(&mut [<$name:camel Descriptor>])->T,
            ) -> Option<T> {
                self.update_resource_descriptor_mut(id.id_mut(),|resource_descriptor|{
                    if let ResourceDescriptor::[<$name:camel>](descriptor) = resource_descriptor {
                        Some(callback(descriptor))
                    }else{None}
                }).flatten()
            }
            */
            pub fn [<remove_ $name:snake>](&mut self, task: &TaskId, id: &[<$name:camel Id>]) -> Result<(), ()> {
                self.remove_resource(task, &id.clone().into())
            }
        }
    };
}

#[derive(Debug)]
pub struct ResourceManager {
    tokio: tokio::runtime::Handle,
    inner: DMGEntityManager<Resource>,

    instances: HashSet<InstanceId>,
    devices: HashSet<DeviceId>,
    swapchains: HashSet<SwapchainId>,

    buffers: HashSet<BufferId>,
    textures: HashSet<TextureId>,
    texture_views: HashSet<TextureViewId>,
    samplers: HashSet<SamplerId>,
    shader_modules: HashSet<ShaderModuleId>,

    bind_group_layouts: HashSet<BindGroupLayoutId>,
    bind_groups: HashSet<BindGroupId>,

    pipeline_layouts: HashSet<PipelineLayoutId>,
    render_pipelines: HashSet<RenderPipelineId>,
    compute_pipelines: HashSet<ComputePipelineId>,
    command_buffers: HashSet<CommandBufferId>,
}
impl ResourceManager {
    pub fn new(tokio: tokio::runtime::Handle) -> Self {
        let inner = DMGEntityManager::new();

        let instances = HashSet::new();
        let devices = HashSet::new();
        let swapchains = HashSet::new();

        let buffers = HashSet::new();
        let textures = HashSet::new();
        let texture_views = HashSet::new();
        let samplers = HashSet::new();
        let shader_modules = HashSet::new();

        let bind_group_layouts = HashSet::new();
        let bind_groups = HashSet::new();

        let pipeline_layouts = HashSet::new();
        let render_pipelines = HashSet::new();
        let compute_pipelines = HashSet::new();
        let command_buffers = HashSet::new();

        Self {
            inner,
            tokio,
            instances,
            devices,
            swapchains,

            buffers,
            textures,
            texture_views,
            samplers,
            shader_modules,

            bind_group_layouts,
            bind_groups,

            pipeline_layouts,
            render_pipelines,
            compute_pipelines,
            command_buffers,
        }
    }

    pub fn entity_device(&self, id: &EntityId) -> Option<&DeviceHandle> {
        let parents = self.inner.entity_parents(id);
        match parents.get(0) {
            Some(parent_id) => {
                if let Some(device) = self.device_handle_ref(&DeviceId::new(*parent_id)) {
                    Some(device)
                } else {
                    self.entity_device(parent_id)
                }
            }
            None => None,
        }
    }

    pub fn entity_device_id(&self, id: impl AsRef<EntityId>) -> Option<DeviceId> {
        let parents = self.inner.entity_parents(id.as_ref());
        match parents.get(0) {
            Some(parent_id) => {
                let device_id = DeviceId::new(*parent_id);
                if let Some(_) = self.device_handle_ref(&device_id) {
                    Some(device_id)
                } else {
                    self.entity_device_id(parent_id)
                }
            }
            None => None,
        }
    }

    fn take_resource(&mut self, id: &EntityId) -> Option<ResourceHandle> {
        self.inner.take_entity_handle(id)
    }

    fn search_compatible(
        &self,
        id: Option<&ResourceId>,
        descriptor: &ResourceDescriptor,
    ) -> Option<ResourceId> {
        if descriptor.state_type() == StateType::Statefull {
            return None;
        }

        match descriptor {
            ResourceDescriptor::Instance(descriptor) => self
                .instances
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.instance_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::Device(descriptor) => self
                .devices
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.device_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::Swapchain(descriptor) => self
                .swapchains
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.swapchain_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),

            ResourceDescriptor::Buffer(descriptor) => self
                .buffers
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.buffer_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::Texture(descriptor) => self
                .textures
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.texture_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::TextureView(descriptor) => self
                .texture_views
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.texture_view_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::Sampler(descriptor) => self
                .samplers
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.sampler_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::ShaderModule(descriptor) => self
                .shader_modules
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.shader_module_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),

            ResourceDescriptor::BindGroupLayout(descriptor) => self
                .bind_group_layouts
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.bind_group_layout_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::BindGroup(descriptor) => self
                .bind_groups
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.bind_group_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),

            ResourceDescriptor::PipelineLayout(descriptor) => self
                .pipeline_layouts
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.pipeline_layout_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::RenderPipeline(descriptor) => self
                .render_pipelines
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.render_pipeline_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::ComputePipeline(descriptor) => self
                .compute_pipelines
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.compute_pipeline_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
            ResourceDescriptor::CommandBuffer(descriptor) => self
                .command_buffers
                .iter()
                .find(|current_id| {
                    if let Some(id) = id {
                        if &ResourceId::from(**current_id) == id {
                            return false;
                        }
                    }
                    self.command_buffer_descriptor_ref(current_id).unwrap() == descriptor
                })
                .cloned()
                .map(|current_id| current_id.into()),
        }
    }

    pub fn add_resource(
        &mut self,
        task: TaskId,
        descriptor: impl Into<ResourceDescriptor>,
        handle: impl Into<Option<ResourceHandle>>,
    ) -> Result<ResourceId, ()> {
        let descriptor = descriptor.into();
        let handle = handle.into();
        let damaged = handle.is_none();

        if descriptor.state_type() == StateType::Stateless {
            if let Some(id) = self.search_compatible(None, &descriptor) {
                self.inner.add_entity_owner(&id.into(), task);
                return Ok(id);
            }
        }

        let resource = Resource::new(vec![task], descriptor.clone(), handle);
        match self.inner.add_entity(resource) {
            Ok(id) => {
                if damaged {
                    self.inner.damage_entity(id);
                }
                let id = self.add_inner(&descriptor, id);
                Ok(id)
            }
            Err(_err) => Err(()),
        }
    }
    pub fn add_resource_descriptor(
        &mut self,
        task: TaskId,
        descriptor: impl Into<ResourceDescriptor>,
    ) -> Result<ResourceId, ()> {
        self.add_resource(task, descriptor, None)
    }

    pub fn update_resource_descriptor<'a>(
        &mut self,
        task: &TaskId,
        id: impl Into<ResourceIdMut<'a>>,
        descriptor: impl Into<ResourceDescriptor>,
    ) -> bool {
        let mut id = id.into();
        let descriptor = descriptor.into();

        if descriptor.state_type() == StateType::Stateless {
            if let Some(compatible_id) = self.search_compatible(Some(&(&id).into()), &descriptor) {
                //TODO Check if actually works
                self.inner.remove_entity_owner(&id.clone().into(), task);
                self.inner
                    .add_entity_owner(&compatible_id.clone().into(), task.clone());
                *id = compatible_id.into();
                return true;
            }
        }
        self.inner
            .update_entity_descriptor(&id.into(), |entity_descriptor| {
                *entity_descriptor = descriptor;
            })
            .is_some()
    }

    pub(crate) fn update_resource_handle(
        &mut self,
        id: &EntityId,
        resource: ResourceHandle,
    ) -> bool {
        self.inner.update_entity_handle(id, Some(resource))
    }

    pub fn remove_resource(&mut self, task: &TaskId, id: &ResourceId) -> Result<(), ()> {
        let owners_count = self.inner.remove_entity_owner(&id.clone().into(), task);

        match owners_count {
            Some(0) => self.inner.remove_entity(&id.clone().into()).map(|v| {
                self.remove_inner(id);
                v
            }),
            Some(_) => Ok(()),
            None => Err(()),
        }
    }

    pub(crate) fn resource_descriptor(&self, id: &ResourceId) -> Option<&ResourceDescriptor> {
        self.inner.entity_descriptor_ref(&id.clone().into())
    }

    pub(crate) fn take_command_buffer(
        &mut self,
        id: &CommandBufferId,
    ) -> Option<crate::wgpu::CommandBuffer> {
        match self.inner.take_entity_handle(id.id_ref()) {
            Some(ResourceHandle::CommandBuffer(handle)) => match Arc::try_unwrap(handle) {
                Ok(unwrapped_command_buffer) => Some(unwrapped_command_buffer),
                Err(arc_command_buffer) => {
                    self.update_resource_handle(
                        id.id_ref(),
                        ResourceHandle::CommandBuffer(arc_command_buffer),
                    );
                    None
                }
            },
            _ => None,
        }
    }

    fn add_inner(&mut self, descriptor: &ResourceDescriptor, id: EntityId) -> ResourceId {
        match descriptor {
            ResourceDescriptor::Instance(_) => {
                let id = InstanceId::new(id);
                self.instances.insert(id);
                id.into()
            }
            ResourceDescriptor::Device(_) => {
                let id = DeviceId::new(id);
                self.devices.insert(id);
                id.into()
            }
            ResourceDescriptor::Swapchain(_) => {
                let id = SwapchainId::new(id);
                self.swapchains.insert(id);
                id.into()
            }

            ResourceDescriptor::Buffer(_) => {
                let id = BufferId::new(id);
                self.buffers.insert(id);
                id.into()
            }
            ResourceDescriptor::Texture(_) => {
                let id = TextureId::new(id);
                self.textures.insert(id);
                id.into()
            }
            ResourceDescriptor::TextureView(_) => {
                let id = TextureViewId::new(id);
                self.texture_views.insert(id);
                id.into()
            }
            ResourceDescriptor::Sampler(_) => {
                let id = SamplerId::new(id);
                self.samplers.insert(id);
                id.into()
            }
            ResourceDescriptor::ShaderModule(_) => {
                let id = ShaderModuleId::new(id);
                self.shader_modules.insert(id);
                id.into()
            }

            ResourceDescriptor::BindGroupLayout(_) => {
                let id = BindGroupLayoutId::new(id);
                self.bind_group_layouts.insert(id);
                id.into()
            }
            ResourceDescriptor::BindGroup(_) => {
                let id = BindGroupId::new(id);
                self.bind_groups.insert(id);
                id.into()
            }

            ResourceDescriptor::PipelineLayout(_) => {
                let id = PipelineLayoutId::new(id);
                self.pipeline_layouts.insert(id);
                id.into()
            }
            ResourceDescriptor::RenderPipeline(_) => {
                let id = RenderPipelineId::new(id);
                self.render_pipelines.insert(id);
                id.into()
            }
            ResourceDescriptor::ComputePipeline(_) => {
                let id = ComputePipelineId::new(id);
                self.compute_pipelines.insert(id);
                id.into()
            }
            ResourceDescriptor::CommandBuffer(_) => {
                let id = CommandBufferId::new(id);
                self.command_buffers.insert(id);
                id.into()
            }
        }
    }

    fn remove_inner(&mut self, id: &ResourceId) {
        match id {
            ResourceId::Instance(id) => {
                self.instances.remove(&id);
            }
            ResourceId::Device(id) => {
                self.devices.remove(&id);
            }
            ResourceId::Swapchain(id) => {
                self.swapchains.remove(&id);
            }

            ResourceId::Buffer(id) => {
                self.buffers.remove(&id);
            }
            ResourceId::Texture(id) => {
                self.textures.remove(&id);
            }
            ResourceId::TextureView(id) => {
                self.texture_views.remove(&id);
            }
            ResourceId::Sampler(id) => {
                self.samplers.remove(&id);
            }
            ResourceId::ShaderModule(id) => {
                self.shader_modules.remove(&id);
            }

            ResourceId::BindGroupLayout(id) => {
                self.bind_group_layouts.remove(&id);
            }
            ResourceId::BindGroup(id) => {
                self.bind_groups.remove(&id);
            }

            ResourceId::PipelineLayout(id) => {
                self.pipeline_layouts.remove(&id);
            }
            ResourceId::RenderPipeline(id) => {
                self.render_pipelines.remove(&id);
            }
            ResourceId::ComputePipeline(id) => {
                self.compute_pipelines.remove(&id);
            }
            ResourceId::CommandBuffer(id) => {
                self.command_buffers.remove(&id);
            }
        }
    }

    make_resource_functions!(Instance);
    make_resource_functions!(Device);
    make_resource_functions!(Swapchain);
    make_resource_functions!(Buffer);
    make_resource_functions!(Texture);
    make_resource_functions!(TextureView);
    make_resource_functions!(Sampler);
    make_resource_functions!(ShaderModule);
    make_resource_functions!(BindGroupLayout);
    make_resource_functions!(BindGroup);
    make_resource_functions!(PipelineLayout);
    make_resource_functions!(RenderPipeline);
    make_resource_functions!(ComputePipeline);
    make_resource_functions!(CommandBuffer);

    pub(crate) fn commit_resources(&mut self) -> bool {
        log::info!(target: "Engine","Committing resources updates");
        self.print_graphviz();

        let mut entity_path = Vec::new();

        let mut visitor = Topo::new(self.graph());
        while let Some(nx) = visitor.next(self.graph()) {
            let id: EntityId = nx.into();
            if self.is_damaged(&id) {
                let dependencies: Vec<EntityId> = self
                    .graph()
                    .neighbors_directed(nx, petgraph::Direction::Incoming)
                    .map(|index| index.into())
                    .collect();
                entity_path.push((id, dependencies));
            }
        }

        #[cfg(multithreading)]
        return self.commit_resources_mt(entity_path);

        #[cfg(not(multithreading))]
        return self.commit_resources_st(entity_path);
    }

    #[cfg(multithreading)]
    pub(crate) fn commit_resources_mt(
        &mut self,
        entity_path: impl IntoIterator<Item = (EntityId, Vec<EntityId>)>,
    ) -> bool {
        use std::collections::HashMap;
        use tokio::sync::RwLock;

        let mut syncs = HashMap::new();
        tokio_scoped::scoped(&self.tokio.clone()).scope(|scope|{
            let resource_manager = Arc::new(RwLock::new(self));

            for (entity,dependencies) in entity_path {
                let (sender,receiver) = tokio::sync::watch::channel(false);
                syncs.insert(entity, receiver);

                let receivers: Vec<_> = dependencies.into_iter().filter_map(|id|{
                    syncs.get(&id).cloned()
                }).collect();

                let resource_manager = resource_manager.clone();
                scope.spawn(async move{
                    for mut receiver in receivers {
                        let success = match receiver.changed().await {
                            Ok(_)=>*receiver.borrow(),
                            Err(_)=>false
                        };

                        if !success {
                            log::error!(target: "EntityManager","Skipping {} update: a dependency has failed to build",entity);
                        }
                    }
                    /*Execute task start*/
                    log::info!(target: "EntityManager","Updating {}",entity);
                    let builder = {
                        let resource_manager = resource_manager.read().await;

                        match resource_manager.entity_descriptor_ref(&entity) {
                            Some(descriptor)=>{
                                match ResourceBuilder::new(&resource_manager,entity,descriptor){
                                    Ok(builder)=>Some(builder),
                                    Err(_)=>None
                                }
                            },
                            _=>None
                        }
                    };

                    if let Some(builder) = builder {
                        let entity_handle = builder.build();

                        {
                            let mut resource_manager = resource_manager.write().await;
                            resource_manager.update_resource_handle(&entity,entity_handle);
                            log::info!(target: "EntityManager","{} updated",entity);
                        }

                        /*Execute task stop*/
                        sender.send(true).unwrap();
                    }
                    else{
                        /*Execute task stop*/
                        log::error!(target: "EntityManager","{} failed to update",entity);
                        sender.send(false).unwrap();
                    }
                });
            }

        });

        true
    }

    #[cfg(not(multithreading))]
    pub(crate) fn commit_resources_st(
        &mut self,
        entity_path: impl IntoIterator<Item = (EntityId, Vec<EntityId>)>,
    ) -> bool {
        for (entity, _dependencies) in entity_path {
            /*Execute task start*/
            log::info!(target: "EntityManager","Updating {}",entity);
            let builder = {
                match self.entity_descriptor_ref(&entity) {
                    Some(descriptor) => match ResourceBuilder::new(&self, entity, descriptor) {
                        Ok(builder) => Some(builder),
                        Err(_) => None,
                    },
                    _ => None,
                }
            };

            if let Some(builder) = builder {
                let entity_handle = builder.build();

                {
                    self.update_resource_handle(&entity, entity_handle);
                    log::info!(target: "EntityManager","{} updated",entity);
                }

                /*Execute task stop*/
            } else {
                /*Execute task stop*/
                log::error!(target: "EntityManager","{} failed to update",entity);
            }
        }

        true
    }
}

impl std::ops::Deref for ResourceManager {
    type Target = DMGEntityManager<Resource>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
