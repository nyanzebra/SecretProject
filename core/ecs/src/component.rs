use std::{
    any::Any,
    collections::BTreeMap,
    sync::{LazyLock, Mutex},
};

use super::{Id, NULL_ID, ONE_ID};

pub static COMPONENT_GENERATOR: LazyLock<Mutex<ComponentGenerator>> =
    LazyLock::new(|| Mutex::new(ComponentGenerator::new()));

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ComponentId(Id);

pub trait Component {
    fn id() -> ComponentId;
}

pub(crate) trait Components {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub struct ComponentGenerator {
    map: BTreeMap<&'static str, ComponentId>,
    current: Id,
}

impl ComponentGenerator {
    pub(crate) fn new() -> Self {
        Self {
            map: Default::default(),
            current: NULL_ID,
        }
    }

    pub fn generate(&mut self, name: &'static str) -> ComponentId {
        if let Some(id) = self.map.get(name) {
            return *id;
        }
        self.current = self.current + ONE_ID;
        let id = ComponentId(self.current);
        self.map.insert(name, id);
        id
    }
}
