extern crate nalgebra as na;
extern crate specs;

use specs::{System, VecStorage, Fetch, ReadStorage, WriteStorage};
use ecs::DeltaTime;

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position(pub na::Vector2<f32>);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Rotation(pub f32);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Velocity(pub na::Vector2<f32>);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct AngularMomentum(pub f32);

pub struct Move;
impl<'a> System<'a> for Move {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (delta, vel, mut pos): Self::SystemData) {
        use specs::Join;

        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0 += vel.0 * delta;
        }
    }
}

pub struct Rotate;
impl<'a> System<'a> for Rotate {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, AngularMomentum>,
        WriteStorage<'a, Rotation>,
    );

    fn run(&mut self, (delta, amo, mut rot): Self::SystemData) {
        use specs::Join;

        let delta = delta.0;

        for (amo, rot) in (&amo, &mut rot).join() {
            rot.0 += amo.0 * delta;
        }
    }
}
