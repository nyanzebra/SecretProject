use ecs::world::{self, World};
use ecs_macros::Component;

#[derive(Component, Debug)]
struct Test(i32);

fn main() {
    let mut world = World::new();
    let entity = world.generate_entity();
    let _component_id = world.register_component::<Test>();
    world.register_component_on_entity::<Test>(entity, Some(Test(-1)));
    if let Some(comp) = world.get_component_on_entity::<Test>(entity) {
        println!("Component: {:?}", comp);
    }
}
