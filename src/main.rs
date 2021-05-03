use rltk::{Rltk, RGB};
use specs::prelude::*;

//Modules
pub mod components;
pub mod map;
pub mod player;
pub mod rect;
pub mod state;
pub mod visibility_system;
use components::{Position, Renderable, Viewshed};
use map::*;
use player::*;
use rect::*;
use state::*;
use visibility_system::*;

fn main() -> rltk::BError {
    //builds the window
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    //instanciate our State struct with our World
    let mut gs = State { ecs: World::new() };

    //we register our components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    //add the map which is a vector of tyles returned by this functions new_map()
    let map = Map::new_map_rooms_and_corridors();
    gs.ecs.insert(map.tiles);
    let (player_x, player_y) = map.rooms[0].center();
    //creates the palyer with the components Position and Renderable
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
        })
        .build();
    //method chaining and build patterns in rust hmm

    rltk::main_loop(context, gs) //the context (window) and the game state (logic)
}

//comon thing in game programming to refer to each iteration as a tick
//
