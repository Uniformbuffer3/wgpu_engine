//! [EntityManager][EntityManager] related structures and enumerations.

use crate::common::*;

pub mod update_context;
pub use update_context::*;

pub mod dmg_entity_manager;
pub use dmg_entity_manager::*;

use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use petgraph::Direction;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Unique identifier for an entity inside the engine.
pub struct EntityId(usize);
impl EntityId {
    pub(crate) fn new(id: usize) -> Self {
        Self(id)
    }
    pub fn id(&self) -> usize {
        self.0
    }
}
impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.0)
    }
}
impl From<NodeIndex<usize>> for EntityId {
    fn from(index: NodeIndex<usize>) -> Self {
        Self(index.index())
    }
}
impl From<EntityId> for NodeIndex<usize> {
    fn from(id: EntityId) -> Self {
        Self::new(id.id())
    }
}
impl AsRef<EntityId> for EntityId {
    fn as_ref(&self) -> &EntityId {
        &self
    }
}

#[derive(Debug, Clone, Copy)]
/// Errors related to entity management.
pub enum EntityManagerError {
    MissingDependencies,
}

#[derive(Debug)]
/// Metadata related to a dependency between two entities.
pub struct Dependency;
impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug)]
/**
This struct store entities into a graph based on the declared dependencies.
*/
pub struct EntityManager<N: HaveDependencies> {
    dependency_graph: StableDiGraph<N, Dependency, usize>,
}
impl<N: HaveDependencies> EntityManager<N> {
    pub fn new() -> Self {
        let dependency_graph = StableDiGraph::default();
        Self { dependency_graph }
    }

    pub(crate) fn graph(&self) -> &StableDiGraph<N, Dependency, usize> {
        &self.dependency_graph
    }
    pub(crate) fn graph_mut(&mut self) -> &mut StableDiGraph<N, Dependency, usize> {
        &mut self.dependency_graph
    }
    /// Iterate over all the entities.
    pub(crate) fn entities(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.dependency_graph
            .node_indices()
            .map(|id| EntityId::new(id.index()))

        //self.dependency_graph.node_weights()
    }
    /// Get an entity.
    pub(crate) fn entity(&self, id: &EntityId) -> Option<&N> {
        self.graph().node_weight(NodeIndex::new(id.id()))
    }
    fn entity_mut(&mut self, id: &EntityId) -> Option<&mut N> {
        self.graph_mut().node_weight_mut(NodeIndex::new(id.id()))
    }

    /// Get the parents of an entity.
    pub(crate) fn entity_parents(&self, id: &EntityId) -> Vec<EntityId> {
        self.graph()
            .neighbors_directed((*id).into(), Direction::Incoming)
            .map(|index| EntityId::new(index.index()))
            .collect()
    }
    /// Add an entity to the graph.
    pub(crate) fn add_entity(
        &mut self,
        entity: impl Into<N>,
    ) -> Result<EntityId, EntityManagerError> {
        let entity = entity.into();

        if !self.check_dependencies(&entity) {
            return Err(EntityManagerError::MissingDependencies);
        }

        let dependencies = entity.dependencies();
        let id = EntityId::new(self.graph_mut().add_node(entity).index());
        dependencies.into_iter().for_each(|dep_id| {
            self.add_dependency(&dep_id, &id);
        });

        Ok(id)
    }
    /// Update an entity.
    pub(crate) fn update_entity<T>(
        &mut self,
        id: &EntityId,
        callback: impl FnOnce(&mut N) -> T,
    ) -> Option<T> {
        match self.entity_mut(id) {
            Some(entity) => {
                let current_dependencies: HashSet<_> = entity.dependencies().into_iter().collect();

                let result = callback(entity);
                let entity = &entity;

                let new_dependencies: HashSet<_> = entity.dependencies().into_iter().collect();

                //Removing no more dependencies
                current_dependencies
                    .difference(&new_dependencies)
                    .for_each(|dep_id| {
                        self.remove_dependency(dep_id, id);
                    });

                //Adding new dependencies
                new_dependencies
                    .difference(&current_dependencies)
                    .for_each(|dep_id| {
                        self.add_dependency(dep_id, id);
                    });

                Some(result)
            }
            None => None,
        }
    }

    /// Remove an entity from the graph.
    pub(crate) fn remove_entity(&mut self, id: &EntityId) -> Result<(), ()> {
        if self.graph_mut().remove_node((*id).into()).is_some() {
            Ok(())
        } else {
            Err(())
        }
    }
    /// Add a dependency between two entities.
    pub(crate) fn add_dependency(&mut self, entity1: &EntityId, entity2: &EntityId) {
        let node1 = NodeIndex::new(entity1.id());
        let node2 = NodeIndex::new(entity2.id());

        match (
            self.graph().contains_node(node1),
            self.graph().contains_node(node2),
            self.graph().find_edge(node1, node2).is_none(),
        ) {
            (true, true, true) => {
                self.graph_mut().add_edge(node1, node2, Dependency);
            }
            (true, true, false) => {
                log::info!(target: "EntityManager","Dependency {} -> {} already exists, skipping",entity1,entity2);
            }
            _ => (),
        }
    }
    /// Remove a dependency between two entities.
    pub(crate) fn remove_dependency(&mut self, entity1: &EntityId, entity2: &EntityId) -> bool {
        if let Some(edge_id) = self
            .graph()
            .find_edge(NodeIndex::new(entity1.id()), NodeIndex::new(entity2.id()))
        {
            self.graph_mut().remove_edge(edge_id);
            true
        } else {
            false
        }
    }

    /// List all the dependencies of an entity.
    pub(crate) fn dependencies(&mut self, entity: &EntityId) -> Vec<EntityId> {
        self.graph()
            .neighbors_directed((*entity).into(), Direction::Incoming)
            .map(|index| index.into())
            .collect()
    }
    pub fn check_dependencies(&self, descriptor: &N) -> bool {
        for dependency in descriptor.dependencies() {
            if self.entity(&dependency).is_none() {
                return false;
            }
        }
        true
    }
}

impl<N: HaveDependencies + std::fmt::Display> EntityManager<N> {
    pub(crate) fn print_graphviz(&self) {
        struct Node<'a, N: std::fmt::Display>(EntityId, &'a N);
        impl<'a, N: std::fmt::Display> std::fmt::Display for Node<'a, N> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", self.0, self.1)
            }
        }
        let graph = self.graph().filter_map(
            |id, entity| Some(Node(EntityId::new(id.index()), entity)),
            |_, dependency| Some(dependency),
        );
        log::info!(target: "EntityManager","\n{}",petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
    }
}

/*
pub struct Iter<'a, N>(Box<dyn Iterator<Item = &'a N>>);
impl<'a, N: 'a> Iter<'a, N> {
    pub fn new<I: 'static + Iterator<Item = &'a N>>(iter: I) -> Self {
        Self(Box::new(iter))
    }
}
impl<'a, N> Iterator for Iter<'a, N> {
    type Item = &'a N;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
*/
