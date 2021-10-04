use crate::common::*;
use crate::entity_manager::EntityManager;

use petgraph::visit::Bfs;

use std::collections::HashSet;
use std::ops::Deref;

#[derive(Debug)]
pub struct DMGEntityManager<N: HaveDescriptorAndHandle>(EntityManager<N>, HashSet<EntityId>);
impl<N: HaveDescriptorAndHandle> DMGEntityManager<N> {
    pub fn new() -> Self {
        Self(EntityManager::new(), HashSet::new())
    }
}
impl<D: HaveDescriptor + HaveDescriptor<D = D>, H, N: HaveDescriptorAndHandle<D = D, H = H>>
    DMGEntityManager<N>
{
    pub(crate) fn add_entity(&mut self, entity: impl Into<N>) -> Result<EntityId, ()> {
        let entity = entity.into();
        match self.0.add_entity(entity) {
            Ok(id) => Ok(id),
            Err(_err) => Err(()),
        }
    }

    pub(crate) fn update_entity_descriptor<T>(
        &mut self,
        id: &EntityId,
        callback: impl FnOnce(&mut D) -> T,
    ) -> Option<T> {
        let result = self.0.update_entity(id, |entity| {
            let current_descriptor = entity.descriptor();

            let result = callback(entity.descriptor_mut());

            let new_descriptor = entity.descriptor();
            (result, current_descriptor.needs_update(&new_descriptor))
        });

        match result {
            Some((value, needs_update)) => {
                if needs_update {
                    self.damage_entity(*id);
                }
                Some(value)
            }
            None => None,
        }
    }

    pub(crate) fn update_entity_handle(&mut self, id: &EntityId, handle: H) -> bool {
        if self
            .0
            .update_entity(id, |entity| *entity.handle_mut() = handle)
            .is_some()
        {
            self.fix_entity(id);
            true
        } else {
            false
        }
    }

    pub(crate) fn entity_descriptor_ref(&self, id: &EntityId) -> Option<&D> {
        self.0.entity(id).map(|entity| entity.descriptor_ref())
    }
    pub(crate) fn entity_handle_ref(&self, id: &EntityId) -> Option<&H> {
        self.0.entity(id).map(|entity| entity.handle_ref())
    }

    pub(crate) fn damage_entity(&mut self, id: EntityId) {
        if !self.is_damaged(&id) {
            let mut bfs = Bfs::new(self.graph(), id.into());
            while let Some(node) = bfs.next(self.graph()) {
                let id: EntityId = node.into();
                log::info!(target: "EntityManager","{} damaged",id);
                self.1.insert(id);
            }
        } else {
            log::info!(target: "EntityManager","{} already damaged, skipping",id);
        }
    }
    pub(crate) fn fix_entity(&mut self, id: &EntityId) {
        self.1.remove(id);
    }
    pub(crate) fn is_damaged(&self, id: &EntityId) -> bool {
        self.1.contains(id)
    }

    #[inline]
    pub(crate) fn add_dependency(&mut self, entity1: &EntityId, entity2: &EntityId) {
        self.0.add_dependency(entity1, entity2)
    }
    #[inline]
    pub(crate) fn remove_entity(&mut self, id: &EntityId) -> Result<(), ()> {
        self.0.remove_entity(id)
    }

    #[inline]
    pub(crate) fn entities(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.0.entities()
    }
}
impl<
        D: HaveDescriptor + HaveDescriptor<D = D>,
        H,
        N: HaveDescriptorAndHandle<D = D, H = Option<H>>,
    > DMGEntityManager<N>
{
    pub(crate) fn take_entity_handle(&mut self, id: &EntityId) -> Option<H> {
        let handle = self
            .0
            .update_entity(id, |entity| entity.handle_mut().take())
            .flatten();
        if handle.is_some() {
            self.damage_entity(*id);
        }
        handle
    }
}

impl<O: PartialEq ,N: HaveDescriptorAndHandle + HaveOwners<O=O>> DMGEntityManager<N> {
    pub fn entity_owners(&mut self, id: &EntityId)->Option<Vec<O>>{
        self.0.entity(id).map(|entity|entity.owners())
    }
    pub fn add_entity_owner(&mut self, id: &EntityId, new_owner: O){
        self.0.entity_mut(id).map(|entity| {
            if entity.owners_ref().iter().position(|current_owner|current_owner == &new_owner).is_none(){
                entity.owners_mut().push(new_owner);
            }
        });
    }
    pub fn remove_entity_owner(&mut self, id: &EntityId, new_owner: O){
        self.0.entity_mut(id).map(|entity| {
            if let Some(index) = entity.owners_ref().iter().position(|current_owner|current_owner == &new_owner){
                entity.owners_mut().remove(index);
            }
        });
    }
}

impl<N: HaveDescriptorAndHandle> Deref for DMGEntityManager<N> {
    type Target = EntityManager<N>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
