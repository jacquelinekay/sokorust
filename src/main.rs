/*
 * */

use std::collections::HashSet;
use std::io;
use std::vec::Vec;
mod handle_vector;

type XCoord = usize;
type YCoord = usize;

type XDiff = i32;
type YDiff = i32;

fn check_underflow(u: usize, i: i32) {
    if i < 0 {
        assert!(u >= (i.abs() as usize));
    }
}

#[derive (Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: XCoord,
    y: YCoord,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x: x, y: y }
    }

    fn offset(mut self, o: Offset) -> Self {
        check_underflow(self.x, o.x);
        check_underflow(self.y, o.y);
        self.x = ((self.x as i32) + o.x) as usize;
        self.y = ((self.y as i32) + o.y) as usize;
        self
    }
}

#[derive (Copy, Clone, Debug)]
struct Offset {
    x: XDiff,
    y: YDiff,
}



const WIDTH: usize = 10;
const HEIGHT: usize = 10;


#[derive (Copy, Clone, Eq, PartialEq)]
enum Entity {
    Player,
    Wall,
    Crate,
    Floor,
}

struct Board {
    width: usize,
    height: usize,
    entities: Vec<Entity>,
}

impl Board {
    fn new(w: usize, h: usize) -> Self {
        let mut b = Board {
            width: w,
            height: h,
            entities: Vec::with_capacity(w * h),
        };
        for _ in 0..w * h {
            b.entities.push(Entity::Floor);
        }
        b
    }

    fn offset(&self, pos: Position, o: Offset) -> Position {
        let o = pos.offset(o);
        assert!(o.x < self.width && o.y < self.height);
        o
    }

    fn to_index(&self, p: Position) -> usize {
        match p {
            Position { x: x, y: y } => (x + y * self.width) as usize,
        }
    }

    fn swap_tile(&mut self, x0: Position, x1: Position) {
        let i0 = self.to_index(x0);
        let i1 = self.to_index(x1);
        self.entities.swap(i0, i1);
    }

    fn tile_at(&self, pos: Position) -> &Entity {
        &self.entities[self.to_index(pos)]
    }
}

struct State {
    board: Board,
    player_pos: Position,
    goals: HashSet<Position>,
    moves: usize,
    running: bool,
}

impl State {
    fn new() -> Self {
        State {
            board: Board::new(WIDTH, HEIGHT),
            player_pos: Position::new(0, 0),
            goals: [Position::new(1, 1)].iter().cloned().collect(),
            moves: 0,
            running: true,
        }
    }

    fn print(&self) {
        // x = i % 10
        // y = i / 10
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let p = Position::new(x, y);
                let entity = self.board.tile_at(p);
                if self.goals.contains(&p) {
                    match entity {
                        &Entity::Crate => print!("O"),
                        &Entity::Player => print!("@"),
                        _ => print!("g"),
                    }
                } else {
                    match entity {
                        &Entity::Player => print!("@"),
                        &Entity::Wall => print!("#"),
                        &Entity::Crate => print!("o"),
                        &Entity::Floor => print!(" "),
                    }
                }
            }
            print!("\n");
        }
    }

    fn load(x: &str) -> Option<State> {
        let mut state = State::new();
        for i in 0..(WIDTH * HEIGHT) {
            state.board.entities[i] = match x.chars().nth(i).unwrap() {
                '@' => {
                    state.player_pos = Position::new(i % WIDTH, i / HEIGHT);
                    Entity::Player
                }
                '#' => Entity::Wall,
                'o' => Entity::Crate,
                ' ' => Entity::Floor,
                _ => return Option::None,
            }
        }
        return Option::Some(state);
    }

    fn move_thing(&mut self, pos: Position, offset: Offset) -> bool {
        let next_pos = pos.offset(offset);
        match self.board.tile_at(next_pos) {
            &Entity::Floor => {
                let old_pos = pos;
                self.board.swap_tile(old_pos, next_pos);

                true
            }
            _ => false,
        }
    }

    fn move_player(&mut self, offset: Offset) {
        let old_pos = self.player_pos;
        let next_pos = old_pos.offset(offset);
        self.moves += 1;

        match *self.board.tile_at(next_pos) {
            Entity::Floor => {
                self.move_thing(old_pos, offset);
                self.player_pos = next_pos;
            }
            Entity::Crate => {
                let success = self.move_thing(next_pos, offset);
                if success {
                    self.move_player(offset);
                    return;
                }
            }
            Entity::Wall => (),
            Entity::Player => (),
        }
        if self.goals
               .iter()
               .all(|g| *self.board.tile_at(*g) == Entity::Crate) {
            self.running = false;
            println!("You won!! Your score: {}", self.moves);
        }
    }
}

fn main() {
    let mut state = State::load("##########\
                                 #        #\
                                 #        #\
                                 #        #\
                                 #   @  o #\
                                 #        #\
                                 #    o   #\
                                 #        #\
                                 #        #\
                                 ##########")
            .unwrap();

    while state.running {
        print!("{}[2J", 27 as char);
        state.print();
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        match input.as_str() {
            "w\n" => state.move_player(Offset { x: 0, y: -1 }),
            "a\n" => state.move_player(Offset { x: -1, y: 0 }),
            "s\n" => state.move_player(Offset { x: 0, y: 1 }),
            "d\n" => state.move_player(Offset { x: 1, y: 0 }),
            _ => (),
        };
    }
}
