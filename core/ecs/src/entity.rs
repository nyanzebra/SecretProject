use super::{Id, NULL_ID, ONE_ID};

pub(crate) const NULL_ENTITY_ID: Id = NULL_ID;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Entity(Id);

impl Entity {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
}

impl From<Entity> for Id {
    fn from(value: Entity) -> Self {
        value.0
    }
}

/// All entities in a game world must be unique.
/// Currently the backing `Id` is actually a `u128`
/// which means that even if generate a million new
/// entities every second we would still have enough free
/// ids to last past the heat death of universe.
///
/// We can check how true this all is later...
pub(crate) struct EntityGenerator {
    current: Id,
}

impl EntityGenerator {
    pub(crate) fn new() -> Self {
        Self { current: NULL_ID }
    }

    pub(crate) fn generate(&mut self) -> Entity {
        self.current = self.current + ONE_ID;
        Entity::new(self.current)
    }
}
