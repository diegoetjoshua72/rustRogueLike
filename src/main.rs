
use rltk::{Rltk,GameState,RGB,VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max,min};
use specs_derive::Component;



#[derive(PartialEq,Copy,Clone)]//look more into these copy makes it so that it is not moved by default clone adds .clone() paritialEq adds == comparisons and how does it do that
enum TileType {
    Wall, Floor
}
//make a lava type that would be cool and a water one as well 

pub fn xy_idx(x:i32, y:i32) -> usize { //this maps the 2d coordinates to the 1d vector array
    (y as usize * 80) + x as usize
}
//usize is the equivalent to size_t which is whatever the basic size type used for the platform is 

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x,49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79,y)] = TileType::Wall;
    }
    
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1,79);
        let y = rng.roll_dice(1,49);
        let idx = xy_idx(x,y);
        if idx != xy_idx(40, 25){
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn draw_map(map: &[TileType], ctx:&mut Rltk){
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile{
            TileType::Floor => {
                ctx.set(x,y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('.'))
            }          

            TileType::Wall => {
                ctx.set(x,y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('#'))
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

}

///////////////////////////////

#[derive(Component,Debug)]
struct Player{}

fn try_move_player(delta_x:i32, delta_y:i32, ecs:&mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y +delta_y));
    }
}

fn player_input(gs:&mut State, ctx: &mut Rltk){
    match ctx.key {
        None => {}

        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

////////////////////////////////////////////////////////////
#[derive(Component)] //derive macro
struct Position{
    x:i32,
    y:i32,
}


#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}



//STATE
struct State{
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();

        player_input(self, ctx);
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

    }
}

impl State {
    // fn run_systems(&mut self) {
    //     let mut lw = LeftWalkerSys{};
    //     lw.run_now(&self.ecs);
    //     self.ecs.maintain();
    // }
}
//////////////////////////

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs: World::new(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.insert(new_map());
    gs.ecs
        .create_entity()
        .with(Position {x: 40, y: 25})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
        //method chaining and build patterns in rust hmm

    rltk::main_loop(context,gs) //the context (window) and the game state (logic)
}

//comon thing in game programming to refer to each iteration as a tick 
//

