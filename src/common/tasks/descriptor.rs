use crate::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskDescriptor {
    pub name: String,
    pub broken: bool,
    pub dependencies: Vec<TaskId>,
}

impl TaskDescriptor {
    pub(crate) fn new(name: String, dependencies: Vec<TaskId>) -> Self {
        let broken = false;
        Self {
            name,
            broken,
            dependencies,
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn set_broken(&mut self, value: bool) {
        self.broken = value;
    }
    pub(crate) fn broken(&self) -> bool {
        self.broken
    }
}
impl HaveDependencies for TaskDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        self.dependencies.iter().map(|dep| *dep.id_ref()).collect()
    }
}
impl HaveDescriptor for TaskDescriptor {
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
    fn needs_update(&self, _other: &Self) -> bool {
        false
    }
}
impl std::fmt::Display for TaskDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
