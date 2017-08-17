extern crate ggez;
extern crate nalgebra as na;

extern crate specs;
#[macro_use] extern crate specs_derive;

use ggez::{conf, GameResult, Context, graphics, timer};
use ggez::event::*;

use specs::{World, Dispatcher, DispatcherBuilder};

use std::time::Duration;

pub mod scene;
pub mod ecs;

pub use scene::{SceneSwitch, Scene, SceneStack};



struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {

        let mut world = specs::World::new();
        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder = ecs::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = ecs::control::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = ecs::graphics::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = ecs::movement::init_world(&mut world, dispatcher_builder);


        let dispatcher = dispatcher_builder.build();

        world.create_entity()
            .with(ecs::movement::Position(na::Vector2::new(320.0, 240.0)))
            .with(ecs::movement::Velocity(na::Vector2::new(0.0, 0.0)))
            .with(ecs::control::Controllable)
            .with(ecs::movement::Rotation(0.3))
            .with(ecs::movement::AngularMomentum(-0.2))
            .with(ecs::graphics::Sprite(graphics::Image::new(ctx, "/duck_target_brown.png")?))
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
            let mut delta = self.world.write_resource::<ecs::DeltaTime>();
            *delta = ecs::DeltaTime(dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9);
        }

        self.dispatcher.dispatch(&mut self.world.res);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;

        graphics::clear(ctx);

        let entities = self.world.entities();
        let positions = self.world.read::<ecs::movement::Position>();
        let rotations = self.world.read::<ecs::movement::Rotation>();
        let sprites = self.world.read::<ecs::graphics::Sprite>();

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
        let mut keyboard = self.world.write_resource::<ecs::control::Keyboard>();
        keyboard.0.insert(keycode, true);
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut keyboard = self.world.write_resource::<ecs::control::Keyboard>();
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
