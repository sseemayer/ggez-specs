use std::collections::HashMap;
use ggez::event::Keycode;

use specs::{System, HashMapStorage, Fetch, ReadStorage, WriteStorage};

use ecs::movement::{Velocity};

pub struct Keyboard(pub HashMap<Keycode, bool>);
impl Keyboard {
    pub fn new() -> Self {
        Keyboard(HashMap::new())
    }

    pub fn is_pressed(&self, btn: Keycode) -> bool {
        match self.0.get(&btn) {
            Some(s) => *s,
            None => false
        }
    }
}

#[derive(Component, Debug)]
#[component(HashMapStorage)]
pub struct Controllable;

pub struct Control;
impl<'a> System<'a> for Control {
    type SystemData = (
        Fetch<'a, Keyboard>,
        ReadStorage<'a, Controllable>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (kbd, cntrl, mut vel): Self::SystemData) {
        use specs::Join;

        let kbd = &*kbd;

        let mut xvel = 0.0;
        let mut yvel = 0.0;

        if kbd.is_pressed(Keycode::W) { yvel -= 300.0; }
        if kbd.is_pressed(Keycode::S) { yvel += 300.0; }
        if kbd.is_pressed(Keycode::A) { xvel -= 300.0; }
        if kbd.is_pressed(Keycode::D) { xvel += 300.0; }

        for (_cntrl, vel) in (&cntrl, &mut vel).join() {
            vel.0[0] = xvel;
            vel.0[1] = yvel;
        }
    }
}
