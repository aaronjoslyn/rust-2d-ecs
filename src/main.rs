use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use hecs::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    dx: f32,
}

struct MainState {
    world: World,
}

impl MainState {
    fn new(world: World) -> GameResult<MainState> {
        let s = MainState { world };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let query = self.world.query_mut::<(&Velocity, &mut Position)>();
        for (_, (velocity, position)) in query {
            position.x = position.x % 800.0 + velocity.dx;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let mut query = self.world.query::<&Position>();
        for (_, position) in &mut query {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                100.0,
                2.0,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &circle, (Vec2::new(position.x, 380.0),))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mut world = World::new();
    world.spawn((Position { x: 0.0 }, Velocity { dx: 1.0 }));
    let cb = ggez::ContextBuilder::new("simple_with_hecs", "");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(world)?;
    event::run(ctx, event_loop, state)
}
