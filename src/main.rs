/*
 * */

use std::io;
mod handle_vector;

#[derive (Copy, Clone)]
struct XCoord(usize);

#[derive (Copy, Clone)]
struct YCoord(usize);

#[derive (Copy, Clone)]
struct Position(XCoord, YCoord);

#[derive (Copy, Clone)]
struct XDiff(i32);

#[derive (Copy, Clone)]
struct YDiff(i32);

#[derive (Copy, Clone)]
struct Offset(XDiff, YDiff);

impl XCoord {
    fn offset(&self, x: XDiff) -> Self {
        XCoord(((self.0 as i32) + x.0) as usize)
    }
}

impl YCoord {
    fn offset(&self, y: YDiff) -> Self {
        YCoord(((self.0 as i32) + y.0) as usize)
    }
}

impl Position {
    fn offset(mut self, o: Offset) -> Self {
        self.0 = self.0.offset(o.0);
        self.1 = self.1.offset(o.1);
        self
    }
}



const WIDTH: usize = 10;
const HEIGHT: usize = 10;






#[derive (Copy, Clone)]
enum Entity {
    Player,
    Wall,
    Crate,
    Floor,
}

struct State {
    board: [Entity; WIDTH * HEIGHT],
    player_pos: Position,
}

impl State {
    fn new() -> Self {
        State {
            board: [Entity::Floor; WIDTH * HEIGHT], // TODO: idiomatic? (repeating size)
            player_pos: Position(XCoord(0), YCoord(0)),
        }
    }

    fn print(&self) {
        // x = i % 10
        // y = i / 10
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let entity = &self.board[x + y * WIDTH];
                match entity {
                    &Entity::Player => print!("@"),
                    &Entity::Wall => print!("#"),
                    &Entity::Crate => print!("o"),
                    &Entity::Floor => print!(" "),
                }
            }
            print!("\n");
        }
    }

    fn load(x: &str) -> Option<State> {
        let mut state = State::new();
        for i in 0..(WIDTH * HEIGHT) {
            state.board[i] = match x.chars().nth(i).unwrap() {
                '@' => {
                    state.player_pos = Position(XCoord(i % WIDTH), YCoord(i / HEIGHT));
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

    fn swap_tile(&mut self, x0: Position, x1: Position) {
        self.board.swap(Self::to_index(x0), Self::to_index(x1));
    }

    fn tile_at(&self, pos: Position) -> &Entity {
        &self.board[Self::to_index(pos)]
    }

    fn move_thing(&mut self, pos: Position, offset: Offset) -> bool {
        let next_pos = pos.offset(offset);
        match self.tile_at(next_pos) {
            &Entity::Floor => {
                let old_pos = pos;
                self.swap_tile(old_pos, next_pos);

                true
            }
            _ => false,
        }
    }

    fn move_player(&mut self, offset: Offset) {
        let old_pos = self.player_pos;
        let next_pos = old_pos.offset(offset);

        match *self.tile_at(next_pos) {
            Entity::Floor => {
                self.move_thing(old_pos, offset);
                self.player_pos = next_pos;
            }
            Entity::Crate => {
                let success = self.move_thing(next_pos, offset);
                if success {
                    self.move_player(offset);
                }
            }
            Entity::Wall => (),
            Entity::Player => (),
        }
    }

    fn to_index(p: Position) -> usize {
        match p {
            Position(XCoord(x), YCoord(y)) => (x + y * WIDTH) as usize,
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

    loop {
        print!("{}[2J", 27 as char);
        state.print();
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        match input.as_str() {
            "w\n" => state.move_player(Offset(XDiff(0), YDiff(-1))),
            "a\n" => state.move_player(Offset(XDiff(-1), YDiff(0))),
            "s\n" => state.move_player(Offset(XDiff(0), YDiff(1))),
            "d\n" => state.move_player(Offset(XDiff(1), YDiff(0))),
            _ => (),
        };
    }
}
