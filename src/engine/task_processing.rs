use crate::{
    common::*,
    engine::batch::Batch,
    engine::resource_manager::ResourceManager,
    engine::task_manager::TaskManager,
    entity_manager::UpdateContext,
    tasks::{TaskDescriptor, TaskTrait},
};

impl super::WGpuEngine {
    /**
    Create a task in the TaskManager.
    */
    pub fn create_task<
        T: 'static + TaskTrait,
        C: Fn(TaskId, &tokio::runtime::Handle, &mut UpdateContext) -> T,
    >(
        &mut self,
        name: String,
        features_and_limits: (crate::wgpu::Features, crate::wgpu::Limits),
        callback: C,
    ) -> Option<TaskId> {
        create_task(
            &mut self.task_manager,
            &mut self.resource_manager,
            self.runtime.handle(),
            name,
            vec![self.engine_task],
            features_and_limits,
            callback,
        )
    }

    /**
    Get and cast the mutable task handle.
    */
    pub fn task_handle_cast_mut<T: TaskTrait, K>(
        &mut self,
        id: &TaskId,
        callback: impl FnOnce(&mut T) -> K,
    ) -> Option<K> {
        self.task_manager.task_handle_cast_mut(id, callback)
    }

    /**
    Dispatch all the tasks and elaborate all the pending operations.
    */
    pub fn dispatch_tasks(&mut self) {
        log::info!(target: "Engine","Dispatching tasks");

        let mut batch = Batch::new(&mut self.resource_manager);
        self.task_manager.commit_tasks(&mut batch);

        batch.resource_manager_mut().commit_resources();
        batch.submit();

        log::info!(target: "Engine","Dispatch completed\n");
    }
}

pub(crate) fn create_task<
    T: 'static + TaskTrait,
    C: Fn(TaskId, &tokio::runtime::Handle, &mut UpdateContext) -> T,
>(
    task_manager: &mut TaskManager,
    resource_manager: &mut ResourceManager,
    tokio: &tokio::runtime::Handle,
    name: String,
    dependencies: Vec<TaskId>,
    _features_and_limits: impl Into<(crate::wgpu::Features, crate::wgpu::Limits)>,
    callback: C,
) -> Option<TaskId> {
    let descriptor = TaskDescriptor::new(name, dependencies);

    match task_manager.add_task((descriptor, None)) {
        Ok(id) => {
            let mut events = Vec::new();
            let mut update_context = UpdateContext::new(id, resource_manager, &mut events);
            let handle: TaskHandle = Box::new(callback(id, tokio, &mut update_context));

            task_manager.update_task_handle(&id, handle);
            Some(id)
        }
        Err(err) => {
            log::error!(target: "Engine","Failed to create task: {:#?}",err);
            None
        }
    }
}
