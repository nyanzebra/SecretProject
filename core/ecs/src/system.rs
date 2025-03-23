use std::{
    any::Any,
    collections::{BTreeMap, HashMap},
    sync::{LazyLock, Mutex},
};

use super::{Id, NULL_ID, ONE_ID};

pub static SYSTEM_GENERATOR: LazyLock<Mutex<SystemGenerator>> =
    LazyLock::new(|| Mutex::new(SystemGenerator::new()));

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct SystemId(Id);


pub enum Event {
    Start(SystemId),
    Pause(SystemId),
    NoOp,
    Stop(SystemId)
}

pub enum State {
    Running(Event),
    Finish

}

pub struct SystemRegistry {
    system_lookup: HashMap<SystemId, SystemIndex>,
    systems: Vec<Box<dyn Systems>>,
}

impl SystemRegistry {
    pub fn new() -> Self {
        Self {
            system_lookup: HashMap::new(),
            systems: Vec::new()
        }
    }

    pub fn register_system<S>(&mut self, system: impl System<>) -> SystemId
    where 
        S: System + 'static,
    {
        let system_id = S::id();

        let index = self.systems.len();
        // we don't allow registering the same system twice
        assert!(self.system_lookup.insert(system_id, index).is_none());

        self.systems
            .push(Box::new(SystemsImpl::<S>::new()) as _);

        system_id
    }
}

impl<F, I, R> System<I> for F where F: Fn(I) -> R {
    type Result = R;

    fn run(self:Box<Self>, input: I) -> Self::Result {
        (*self)(input)
    }
}

impl<F, I1, I2, R> System<(I1, I2)> for F where F: Fn(I1, I2) -> R {
    type Result = R;

    fn run(self:Box<Self>, input: (I1, I2)) -> Self::Result {
        (*self)(input)
    }
}


pub trait System<Input> {
    type Result;

    fn run(self:Box<Self>, input: Input) -> Self::Result;
}

pub(crate) trait Systems {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub struct SystemGenerator {
    map: BTreeMap<&'static str, SystemId>,
    current: Id,
}

type SystemIndex = usize;
type SystemsImpl<T> = Vec<Option<T>>;
impl<S> Systems for SystemsImpl<S>
where
    S: System + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

impl SystemGenerator {
    pub(crate) fn new() -> Self {
        Self {
            map: Default::default(),
            current: NULL_ID,
        }
    }

    pub fn generate(&mut self, name: &'static str) -> SystemId {
        if let Some(id) = self.map.get(name) {
            return *id;
        }
        self.current = self.current + ONE_ID;
        let id = SystemId(self.current);
        self.map.insert(name, id);
        id
    }
}