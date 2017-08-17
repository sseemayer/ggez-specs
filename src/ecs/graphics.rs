extern crate ggez;
extern crate specs;

use ggez::graphics;
use specs::{VecStorage};

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Sprite(pub graphics::Image);
