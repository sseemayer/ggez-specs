extern crate ggez;
extern crate nalgebra as na;

extern crate specs;
#[macro_use] extern crate specs_derive;

use ggez::{conf, GameResult, Context, graphics, timer};
use ggez::event::*;

use specs::{System, VecStorage, HashMapStorage, Fetch, ReadStorage, WriteStorage, World, Dispatcher, DispatcherBuilder};

use std::time::Duration;
use std::collections::HashMap;

#[derive(Component, Debug)]
#[component(VecStorage)]
struct Position(na::Vector2<f32>);

#[derive(Component, Debug)]
#[component(VecStorage)]
struct Rotation(f32);

#[derive(Component, Debug)]
#[component(VecStorage)]
struct Velocity(na::Vector2<f32>);

#[derive(Component, Debug)]
#[component(VecStorage)]
struct AngularMomentum(f32);

#[derive(Component, Debug)]
#[component(VecStorage)]
struct Sprite(graphics::Image);

#[derive(Component, Debug)]
#[component(HashMapStorage)]
struct Controllable;

struct DeltaTime(f32);

struct Keyboard(HashMap<Keycode, bool>);
impl Keyboard {
    fn is_pressed(&self, btn: Keycode) -> bool {
        match self.0.get(&btn) {
            Some(s) => *s,
            None => false
        }
    }
}

struct Move;
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

struct Rotate;
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

struct Control;
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

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {

        let mut world = specs::World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Rotation>();
        world.register::<AngularMomentum>();
        world.register::<Controllable>();
        world.register::<Sprite>();

        world.add_resource(DeltaTime(0.05));
        world.add_resource(Keyboard(HashMap::new()));

        world.create_entity()
            .with(Position(na::Vector2::new(320.0, 240.0)))
            .with(Velocity(na::Vector2::new(0.0, 0.0)))
            .with(Controllable)
            .with(Rotation(0.3))
            .with(AngularMomentum(-0.2))
            .with(Sprite(graphics::Image::new(ctx, "/duck_target_brown.png")?))
            .build();

        let dispatcher = DispatcherBuilder::new()
            .add(Control, "Control", &[])
            .add(Move, "Move", &[])
            .add(Rotate, "Rotate", &[])
            .build();

        Ok(MainState {
            world: world,
            dispatcher: dispatcher,
        })
    }
}

impl<'a, 'b> EventHandler for MainState<'a, 'b> {

    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {

        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9);
        }

        self.dispatcher.dispatch(&mut self.world.res);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;

        graphics::clear(ctx);

        let entities = self.world.entities();
        let positions = self.world.read::<Position>();
        let rotations = self.world.read::<Rotation>();
        let sprites = self.world.read::<Sprite>();

        for (ent, pos, spr) in (&*entities, &positions, &sprites).join() {
            let rot = match rotations.get(ent) {
                Some(r) => r.0,
                None => 0.0
            };

            graphics::draw(ctx, &spr.0, graphics::Point::new(pos.0[0], pos.0[1]), rot)?;
        }


        graphics::present(ctx);
        timer::sleep(Duration::from_secs(0));
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut keyboard = self.world.write_resource::<Keyboard>();
        keyboard.0.insert(keycode, true);
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut keyboard = self.world.write_resource::<Keyboard>();
        keyboard.0.insert(keycode, false);
    }

}


fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "ggez+specs".to_string();
    c.window_width = 640;
    c.window_height = 480;

    let mut ctx = Context::load_from_conf("ggez-specs", "ggez", c).unwrap();

    match MainState::new(&mut ctx) {
        Err(e) => {
            println!("Could not load game!\nError: {}", e);
        }
        Ok(ref mut game) => {
            graphics::set_background_color(&mut ctx, (0, 0, 0, 255).into());
            let result = run(&mut ctx, game);
            if let Err(e) = result {
                println!("Error running game: {}", e);
            } else {
                println!("Clean exit");
            }
        }
    }
}
