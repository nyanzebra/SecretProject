use std::{any::Any, collections::HashMap};

use super::{
    component::{Component, ComponentId, Components},
    entity::{Entity, EntityGenerator},
    Id,
};

type ComponentIndex = usize;
type ComponentsImpl<T> = Vec<Option<T>>;

impl<C> Components for ComponentsImpl<C>
where
    C: Component + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub struct World {
    // generators
    entity_generator: EntityGenerator,
    // ecs management
    component_lookup: HashMap<ComponentId, ComponentIndex>,
    components: Vec<Box<dyn Components>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_generator: EntityGenerator::new(),
            component_lookup: HashMap::new(),
            components: Vec::new(),
        }
    }

    pub fn generate_entity(&mut self) -> Entity {
        let entity = self.entity_generator.generate();

        entity
    }

    pub fn register_component<C>(&mut self) -> ComponentId
    where
        C: Component + 'static,
    {
        let component_id = C::id();

        let index = self.components.len();
        // we don't allow registering the same component twice
        assert!(self.component_lookup.insert(component_id, index).is_none());

        self.components
            .push(Box::new(ComponentsImpl::<C>::new()) as _);

        component_id
    }

    pub fn register_component_on_entity<C>(&mut self, entity: Entity, initial_state: Option<C>)
    where
        C: Component + 'static,
    {
        let index = *self
            .component_lookup
            .get(&C::id())
            .expect("component must be generated before registering entities onto it");
        let components = self.components[index].as_mut_any().downcast_mut::<ComponentsImpl<C>>().expect("tried to register an entity with a component id that does not match the component type");

        let id: Id = entity.into();
        let index: usize = id.into();
        while components.len() <= index {
            components.push(None);
        }
        components[index] = initial_state;
    }

    pub fn get_component_on_entity<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component + 'static,
    {
        let index = *self
            .component_lookup
            .get(&C::id())
            .expect("component must be generated before registering entities onto it");
        let components = self.components[index]
            .as_any()
            .downcast_ref::<ComponentsImpl<C>>()
            .expect(
                "tried to get an entity with a component id that does not match the component type",
            );

        let id: Id = entity.into();
        let index: usize = id.into();
        components[index].as_ref()
    }
}
