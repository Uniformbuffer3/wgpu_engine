//! [TaskManager][TaskManager] related structures, enumerations and macros.

use crate::common::*;
use crate::engine::batch::Batch;

use crate::EntityManager;
use crate::Task;
use petgraph::visit::Topo;

/**
TaskManager is a specialization of EntityManager and an major subsystem of WGpuEngine.
It is responsible to manage the task creation, destruction and manipulation.
*/
pub struct TaskManager(EntityManager<Task>);
impl TaskManager {
    pub fn new() -> Self {
        Self(EntityManager::new())
    }
    /**
    Add a new task to the manager.
    */
    pub(crate) fn add_task(&mut self, task: impl Into<Task>) -> Result<TaskId, ()> {
        match self.0.add_entity(task.into()) {
            Ok(id) => Ok(TaskId::new(id)),
            Err(_) => Err(()),
        }
    }

    /**
    Update the handle of a task.
    */
    pub(crate) fn update_task_handle(&mut self, id: &TaskId, handle: TaskHandle) -> bool {
        self.0
            .update_entity(id.id_ref(), |entity| *entity.handle_mut() = Some(handle))
            .is_some()
    }

    /**
    Get the task descriptor reference.
    */
    pub(crate) fn task_descriptor_ref(&self, id: &TaskId) -> Option<&TaskDescriptor> {
        self.0.entity(id.id_ref()).map(|task| task.descriptor_ref())
    }

    /**
    Get the task handle reference.
    */
    pub(crate) fn task_handle_ref(&self, id: &TaskId) -> Option<&TaskHandle> {
        match self.0.entity(id.id_ref()) {
            Some(task) => task.handle_ref().as_ref(),
            None => None,
        }
    }

    /**
    Get the mutable task handle reference.
    */
    pub fn task_handle_mut(&mut self, id: &TaskId, callback: impl FnOnce(&mut TaskHandle)) -> bool {
        self.0
            .update_entity(id.id_ref(), |task| {
                callback(task.handle_mut().as_mut().unwrap())
            })
            .is_some()
    }

    /**
    Get and cast the task handle reference.
    */
    pub fn task_handle_cast_ref<T: TaskTrait, K>(
        &self,
        id: &TaskId,
        callback: impl FnOnce(&T) -> K,
    ) -> Option<K> {
        self.0.print_graphviz();
        self.0
            .entity(id.id_ref())
            .map(|task| {
                task.handle_ref()
                    .as_ref()
                    .unwrap()
                    .downcast_ref::<T>()
                    .map(callback)
            })
            .flatten()
    }

    /**
    Get and cast the mutable task handle reference.
    */
    pub fn task_handle_cast_mut<T: TaskTrait, K>(
        &mut self,
        id: &TaskId,
        callback: impl FnOnce(&mut T) -> K,
    ) -> Option<K> {
        self.0
            .update_entity(id.id_ref(), |task| {
                task.handle_mut()
                    .as_mut()
                    .unwrap()
                    .downcast_mut::<T>()
                    .map(callback)
            })
            .flatten()
    }

    /**
    Commit the pending updates of the tasks.
    */
    pub(crate) fn commit_tasks(&mut self, batch: &mut Batch) {
        log::info!(target: "Engine","Committing tasks updates");
        self.0.print_graphviz();

        let mut events = Vec::new();

        let mut visitor = Topo::new(self.0.graph());
        while let Some(nx) = visitor.next(self.0.graph()) {
            let id: TaskId = TaskId::new(nx.into());
            self.task_handle_mut(&id, |task| {
                //task.update();

                log::info!(target: "Engine","Updating task resources {}",id);
                let mut update_context =
                    UpdateContext::new(id, batch.resource_manager_mut(), &mut events);
                task.update_resources(&mut update_context);

                let resource_writes = update_context.into_resource_writes();
                batch.add_resource_writes(resource_writes);

                task.command_buffers().into_iter().for_each(|id| {
                    batch.add_command_buffer(id);
                });
            });
        }
    }
}
