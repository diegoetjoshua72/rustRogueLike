use super::player_input;
use super::VisibilitySystem; //this is possible because i use visiblity_system::* in main.rs
use super::{draw_tile_vector, TileType};
use super::{Position, Renderable};
use rltk::{GameState, Rltk};
use specs::prelude::*;

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_tile_vector(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
