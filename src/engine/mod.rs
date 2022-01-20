use crate::common::*;

mod batch;
mod engine_task;
mod surface_processing;
mod task_processing;

pub mod task_manager;
pub use task_manager::TaskManager;

pub mod resource_manager;
pub use resource_manager::ResourceManager;

#[derive(Debug, Clone, Copy)]
/// Possible engine errors.
pub enum WGpuEngineError {
    InitializationFailed,
}

/**
The main entry point of the engine.
*/
pub struct WGpuEngine {
    runtime: tokio::runtime::Runtime,
    task_manager: TaskManager,
    resource_manager: ResourceManager,
    engine_task: TaskId,

    tasks: Vec<Box<dyn TaskTrait + Sync + Send>>,
}

impl WGpuEngine {
    pub fn new(requirements: impl Into<Requirements>) -> Result<Self, WGpuEngineError> {
        let requirements = requirements.into();

        let runtime = tokio::runtime::Runtime::new().unwrap();

        let mut task_manager = TaskManager::new();
        let mut resource_manager = ResourceManager::new(runtime.handle().clone());

        let engine_task = task_processing::create_task(
            &mut task_manager,
            &mut resource_manager,
            runtime.handle(),
            String::from("EngineTask"),
            Vec::new(),
            requirements.clone(),
            |id, tokio, update_context| {
                engine_task::EngineTask::new(
                    id,
                    tokio.clone(),
                    requirements.clone(),
                    update_context,
                )
            },
        )
        .expect("Failed to initialize engine task");

        let tasks = Vec::new();
        Ok(Self {
            runtime,
            task_manager,
            resource_manager,
            engine_task,
            tasks,
        })
    }

    #[cfg(feature = "pal")]
    /**
    Retrieve the WGpuContext to allow the integration with PAL.
    */
    pub fn wgpu_context(&self) -> pal::definitions::WgpuContext {
        use crate::engine::engine_task::EngineTask;
        let (instance, devices) = self
            .task_manager
            .task_handle_cast_ref(&self.engine_task, |engine_task: &EngineTask| {
                (
                    engine_task.instance().clone(),
                    engine_task.devices().clone(),
                )
            })
            .unwrap();
        pal::definitions::WgpuContext {
            instance: self
                .resource_manager
                .instance_handle_ref(&instance)
                .unwrap()
                .clone(),
            devices: devices
                .iter()
                .map(|id| self.resource_manager.device_handle_ref(id).unwrap().clone())
                .collect(),
        }
    }
}
