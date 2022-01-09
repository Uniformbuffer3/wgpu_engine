//! Common structures and enumerations needed for the whole library.

pub mod resources;
pub use resources::*;

pub mod tasks;
pub use tasks::*;

pub mod requirements;
pub use requirements::*;

pub mod events;
pub use events::*;

macro_rules! make_id {
    [$($name: ident),*] => {
        paste::paste! {
            $(
                #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
                #[doc = "Id of [" [<$name:camel Handle>] "][crate::common::" [<$name:camel Handle>]  "]."]
                pub struct [<$name:camel Id>](EntityId);
                impl [<$name:camel Id>] {
                    pub(crate) fn new(entity_id: EntityId) -> Self {
                        Self(entity_id)
                    }
                    pub fn id(&self) -> EntityId {
                        self.0
                    }
                    pub fn id_ref(&self) -> &EntityId {
                        &self.0
                    }
                    pub fn id_mut(&mut self) -> &mut EntityId {
                        &mut self.0
                    }
                }
                impl std::fmt::Display for [<$name:camel Id>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}{}", std::stringify!([<$name:camel>]), self.0)
                    }
                }
                impl AsRef<EntityId> for [<$name:camel Id>] {
                    fn as_ref(&self) -> &EntityId {
                        &self.0
                    }
                }
            )*
        }
    };
}

pub(crate) use make_id;

/// Offset of a 2D element.
pub struct Offset2D {
    pub x: u32,
    pub y: u32,
}

impl From<(u32, u32)> for Offset2D {
    fn from(tuple: (u32, u32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

/// Extension of a 2D element.
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

impl From<(u32, u32)> for Extent2D {
    fn from(tuple: (u32, u32)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

/// Area of a 2D element.
pub struct Area2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

impl From<(Offset2D, Extent2D)> for Area2D {
    fn from(tuple: (Offset2D, Extent2D)) -> Self {
        Self {
            offset: tuple.0,
            extent: tuple.1,
        }
    }
}

impl From<((u32, u32), Extent2D)> for Area2D {
    fn from(tuple: ((u32, u32), Extent2D)) -> Self {
        Self {
            offset: tuple.0.into(),
            extent: tuple.1,
        }
    }
}

impl From<(Offset2D, (u32, u32))> for Area2D {
    fn from(tuple: (Offset2D, (u32, u32))) -> Self {
        Self {
            offset: tuple.0,
            extent: tuple.1.into(),
        }
    }
}

impl From<(u32, u32, u32, u32)> for Area2D {
    fn from(tuple: (u32, u32, u32, u32)) -> Self {
        Self {
            offset: (tuple.0, tuple.1).into(),
            extent: (tuple.2, tuple.3).into(),
        }
    }
}
