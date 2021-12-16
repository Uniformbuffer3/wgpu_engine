use crate::BufferDescriptor;
use crate::BufferId;
use crate::BufferToBufferCopy;
use crate::BufferWrite;
use crate::Command;
use crate::CommandBufferDescriptor;
use crate::CommandBufferId;
use crate::DeviceId;
use crate::ResourceWrite;
use crate::UpdateContext;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct BufferManager<D: bytemuck::Pod + Sized, A> {
    label: String,
    phantom: PhantomData<D>,
    device: DeviceId,
    buffer: BufferId,
    descriptor: BufferDescriptor,
    need_rebuild: bool,

    id_map: HashMap<usize, (usize, A)>,

    command_buffer: CommandBufferId,
    pending_copies: Vec<Command>,
    pending_writes: Vec<BufferWrite>,

    support_buffer: BufferId,
}
impl<D: bytemuck::Pod + Sized, A: std::fmt::Debug> BufferManager<D, A> {
    pub fn new(
        update_context: &mut UpdateContext,
        label: String,
        device: DeviceId,
        capacity: usize,
        usages: crate::wgpu::BufferUsage,
    ) -> Self {
        let descriptor = BufferDescriptor {
            label: label.clone() + " buffer",
            device,
            size: (capacity * std::mem::size_of::<D>()) as u64,
            usage: crate::wgpu::BufferUsage::COPY_SRC | crate::wgpu::BufferUsage::COPY_DST | usages,
        };

        let buffer = update_context
            .add_buffer_descriptor(descriptor.clone())
            .unwrap();

        let support_buffer_descriptor = BufferDescriptor {
            label: label.clone() + " support buffer",
            device,
            size: std::mem::size_of::<D>() as u64,
            usage: crate::wgpu::BufferUsage::COPY_SRC | crate::wgpu::BufferUsage::COPY_DST,
        };

        let support_buffer = update_context
            .add_buffer_descriptor(support_buffer_descriptor.clone())
            .unwrap();

        let command_buffer = update_context
            .add_command_buffer_descriptor(CommandBufferDescriptor {
                label: label.clone() + " command buffer",
                device,
                commands: Vec::new(),
            })
            .unwrap();

        let phantom = PhantomData;
        let need_rebuild = false;
        let id_map = HashMap::new();

        let pending_copies = Vec::new();
        let pending_writes = Vec::new();
        Self {
            label,
            phantom,
            device,
            buffer,
            descriptor,
            need_rebuild,
            id_map,
            command_buffer,
            pending_copies,
            pending_writes,

            support_buffer,
        }
    }
    pub fn id(&self) -> &BufferId {
        &self.buffer
    }
    pub fn len(&self) -> usize {
        self.id_map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.id_map.is_empty()
    }
    pub fn size(&self) -> usize {
        self.id_map.len() * std::mem::size_of::<D>()
    }
    pub fn capacity(&self) -> usize {
        self.descriptor.size as usize / std::mem::size_of::<D>()
    }
    pub fn next_slot(&self) -> usize {
        self.len()
    }

    pub fn request(&mut self, id: usize, auxiliary_data: A, data: D) {
        let slot_id = self.id_map.len();
        if slot_id < self.capacity() {
            self.id_map.insert(id, (slot_id, auxiliary_data));
            assert!(self.pending_write_struct(&id, data));
        } else {
            self.extend();
            self.request(id, auxiliary_data, data);
        }
    }

    pub fn release_pending(&mut self, buffer_index: &usize) -> Option<A> {
        let removed_slot = if let Some((id, _)) = self.id_map.get(buffer_index) {
            *id
        } else {
            log::error!(target: "Buffer Manager","release_pending: buffer_index {} does not exists",buffer_index);
            return None;
        };

        let last_slot = self.id_map.len() - 1;
        if removed_slot == last_slot {
            self.id_map
                .remove(buffer_index)
                .map(|(_, associated_data)| associated_data)
        } else {
            let removed_element = match self.id_map.remove(buffer_index) {
                Some(removed_element) => removed_element,
                None => return None,
            };

            //Last
            self.id_map
                .iter_mut()
                .find(|(_id, value)| value.0 == last_slot)
                .map(|(_id, value)| {
                    value.0 = removed_element.0;
                    //println!("Last: {:#?}",(id,value));
                });
            //println!("Last key: {}",last_key);

            //println!("Removed element: {:#?}",removed_element);
            /*
            let last = self.id_map.remove(&last_key).unwrap();
            println!("Last: {:#?}",last);
            self.id_map.insert(*buffer_index, last);
            */

            /*
                        let command = Command::BufferToBuffer(BufferToBufferCopy {
                            src_buffer: self.buffer,
                            src_offset: (last_slot * std::mem::size_of::<D>()) as u64,
                            dst_buffer: self.buffer,
                            dst_offset: (removed_slot * std::mem::size_of::<D>()) as u64,
                            size: std::mem::size_of::<D>() as u64,
                        });
                        self.pending_copies.push(command);
            */

            let mut commands = vec![
                Command::BufferToBuffer(BufferToBufferCopy {
                    src_buffer: self.buffer,
                    src_offset: (last_slot * std::mem::size_of::<D>()) as u64,
                    dst_buffer: self.support_buffer,
                    dst_offset: 0,
                    size: std::mem::size_of::<D>() as u64,
                }),
                Command::BufferToBuffer(BufferToBufferCopy {
                    src_buffer: self.support_buffer,
                    src_offset: 0,
                    dst_buffer: self.buffer,
                    dst_offset: (removed_slot * std::mem::size_of::<D>()) as u64,
                    size: std::mem::size_of::<D>() as u64,
                }),
            ];
            self.pending_copies.append(&mut commands);

            //println!("Associated data from middle is some: true");
            Some(removed_element.1)
        }
    }

    pub fn pending_write_struct(&mut self, buffer_index: &usize, data: D) -> bool {
        self.pending_write(buffer_index, move || {
            (0, bytemuck::bytes_of(&data).to_vec())
        })
    }

    pub fn pending_write_field<U: bytemuck::Pod + Sized>(
        &mut self,
        buffer_index: &usize,
        offset: field_offset::FieldOffset<D, U>,
        field: U,
    ) -> bool {
        self.pending_write(buffer_index, move || {
            (
                offset.get_byte_offset(),
                bytemuck::bytes_of(&field).to_vec(),
            )
        })
    }

    fn pending_write(
        &mut self,
        buffer_index: &usize,
        callback: impl Fn() -> (usize, Vec<u8>),
    ) -> bool {
        let slot = if let Some((id, _)) = self.id_map.get(buffer_index) {
            *id
        } else {
            log::error!(target: "Buffer Manager","Failed write buffer: index {} does not exists",buffer_index);
            return false;
        };

        let (offset, data) = callback();
        if offset + data.len() <= std::mem::size_of::<D>() {
            let write = BufferWrite {
                buffer: self.buffer,
                offset: (slot * std::mem::size_of::<D>() + offset) as u64,
                data,
            };
            self.pending_writes.push(write);
            true
        } else {
            log::error!(target: "Buffer Manager","Failed write buffer: offset {} + size {} greater then the slot size {}",offset,data.len(),std::mem::size_of::<D>());
            false
        }
    }

    fn extend(&mut self) {
        let new_capacity = self.capacity() + 32;
        self.descriptor.size = (new_capacity * std::mem::size_of::<D>()) as u64;
        self.need_rebuild = true;
    }

    pub fn update(&mut self, update_context: &mut UpdateContext) -> Vec<Command> {
        if self.need_rebuild {
            update_context.update_buffer_descriptor(&mut self.buffer, self.descriptor.clone());
            self.need_rebuild = false;
        }

        let mut writes: Vec<_> = self
            .pending_writes
            .drain(..)
            .map(ResourceWrite::Buffer)
            .collect();
        update_context.write_resource(&mut writes);

        self.pending_copies.drain(..).collect()
    }

    pub fn associated_data(&self, buffer_index: &usize) -> Option<&A> {
        self.id_map
            .get(buffer_index)
            .map(|(_, associated_data)| associated_data)
    }
    pub fn associated_data_mut(&mut self, buffer_index: &usize) -> Option<&mut A> {
        self.id_map
            .get_mut(buffer_index)
            .map(|(_, associated_data)| associated_data)
    }

    pub fn data_slot(&self, buffer_index: &usize) -> Option<usize> {
        self.id_map.get(buffer_index).map(|(slot, _)| *slot)
    }
}
use std::collections::hash_map::Iter;
impl<'a, D: bytemuck::Pod + Sized, A> IntoIterator for &'a BufferManager<D, A> {
    type Item = (&'a usize, &'a (usize, A));
    type IntoIter = Iter<'a, usize, (usize, A)>;
    fn into_iter(self) -> Self::IntoIter {
        self.id_map.iter()
    }
}
