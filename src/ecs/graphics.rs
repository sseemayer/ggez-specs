extern crate ggez;
extern crate specs;

use ggez::graphics;
use specs::{VecStorage, World, DispatcherBuilder};

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Sprite(pub graphics::Image);

pub fn init_world<'a, 'b>(world: &mut World, dispatcher_builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    world.register::<Sprite>();

    dispatcher_builder
}
