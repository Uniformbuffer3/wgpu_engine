use crate::common::*;
use crate::engine::batch::Batch;

use crate::EntityManager;
use crate::Task;
use petgraph::visit::Topo;

pub struct TaskManager(EntityManager<Task>);
impl TaskManager {
    pub fn new() -> Self {
        Self(EntityManager::new())
    }
    pub(crate) fn add_task(&mut self, task: impl Into<Task>) -> Result<TaskId, ()> {
        match self.0.add_entity(task.into()) {
            Ok(id) => Ok(TaskId::new(id)),
            Err(_) => Err(()),
        }
    }
    pub(crate) fn update_task_handle(&mut self, id: &TaskId, handle: TaskHandle) -> bool {
        self.0
            .update_entity(id.id_ref(), |entity| *entity.handle_mut() = Some(handle))
            .is_some()
    }

    pub(crate) fn task_descriptor_ref(&self, id: &TaskId) -> Option<&TaskDescriptor> {
        self.0.entity(id.id_ref()).map(|task| task.descriptor_ref())
    }
    pub(crate) fn task_handle_ref(&self, id: &TaskId) -> Option<&TaskHandle> {
        match self.0.entity(id.id_ref()) {
            Some(task) => task.handle_ref().as_ref(),
            None => None,
        }
    }
    pub fn task_handle_mut(&mut self, id: &TaskId, callback: impl FnOnce(&mut TaskHandle)) -> bool {
        self.0
            .update_entity(id.id_ref(), |task| {
                callback(task.handle_mut().as_mut().unwrap())
            })
            .is_some()
    }

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
    pub(crate) fn commit_tasks(&mut self, batch: &mut Batch) {
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
    /*
        pub(crate) fn commit_tasks(&mut self,resource_manager: &mut EntityManager<Resource>, events: &[ResourceEvent]) -> bool {
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
    /*
            let task_map: HashMap<_, _> = tasks
                .iter_mut()
                .map(|task| (*task.as_ref(), RwLock::new(task)))
                .collect();
            let task_map = Arc::new(task_map);
    */
            let mut syncs = HashMap::new();
            tokio_scoped::scoped(&self.tokio_handle().clone()).scope(|scope|{
                let task_manager = Arc::new(RwLock::new(self));

                for (entity,dependencies) in entity_path {
                    let (sender,receiver) = tokio::sync::watch::channel(false);
                    syncs.insert(entity, receiver);

                    let receivers: Vec<_> = dependencies.into_iter().filter_map(|id|{
                        syncs.get(&id).cloned()
                    }).collect();

                    let task_manager = task_manager.clone();

                    scope.spawn(async move{
                        for mut receiver in receivers {
                            match receiver.changed().await {
                                Ok(_)=>(),
                                Err(_)=>{
                                    log::error!(target: "ResourceManager","Skipping Resource {} update: a dependency has failed to build",entity);
                                    return;
                                }
                            };
                        }
                        /*Execute task start*/
                        let mut task_manager = task_manager.write().await;

                        let mut task = if let Some(task) = task_manager.entity_mut(&entity){task}
                        else{return;};

                        task.handle_mut().update();

                        log::info!(target: "Engine","Updating task resources {}",entity);
                        let device = task.descriptor().device;
                        let mut update_context = UpdateContext::new(device, resource_manager);
                        task.handle_mut().update_resources(&mut update_context,&events);
                        let resource_writes = update_context.into_resource_writes();

                        log::info!(target: "Engine","Damaging task {} command buffers",entity);
    /*
                        task.handle_ref().command_buffers()
                            .into_iter()
                            .for_each(|id| resource_manager.damage_entity(id));
    */

                        resource_writes
                            .into_iter()
                            .map(move |resource_write| (device, resource_write));



                        /*Execute task end*/
                        sender.send(true).unwrap();
                    });
                }

            });

            true
        }
        */
}
