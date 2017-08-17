pub mod control;
pub mod graphics;
pub mod movement;

pub struct DeltaTime(pub f32);

use specs::{World, DispatcherBuilder};

pub fn init_world<'a, 'b>(world: &mut World, dispatcher_builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(DeltaTime(0.05));
    dispatcher_builder
}
