/*
 * */
use std::io;
use std::mem;

const width: usize = 10;
const height: usize = 10;

#[derive (Copy, Clone)]
enum Entity {
    Player,
    Wall,
    Crate,
    Floor,
}

struct State {
    board: [Entity; width * height],
    player_pos: (usize, usize),
}

fn apply_offset(x: usize, offset: i32) -> usize {
    ((x as i32) + offset) as usize // TODO: idiomatic?
}

fn apply_offset_to_vector((x, y): (usize, usize), (ox, oy): (i32, i32)) -> (usize, usize) {
    (apply_offset(x, ox), apply_offset(y, oy))
}

impl State {
    fn new() -> Self {
        State {
            board: [Entity::Floor; width * height], // TODO: idiomatic? (repeating size)
            player_pos: (0, 0),
        }
    }

    fn print(&self) {
        // x = i % 10
        // y = i / 10
        for y in 0..height {
            for x in 0..width {
                let entity = &self.board[x + y * width];
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
        for i in 0..(width * height) {
            state.board[i] = match x.chars().nth(i).unwrap() {
                '@' => {
                    state.player_pos = (i % width, i / height);
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

    fn swap_tile(&mut self, x0: (usize, usize), x1: (usize, usize)) {
        self.board.swap(Self::to_index(x0), Self::to_index(x1));
    }

    fn tile_at(&self, pos: (usize, usize)) -> &Entity {
        &self.board[Self::to_index(pos)]
    }

    fn move_thing(&mut self, pos: (usize, usize), offset: (i32, i32)) -> bool {
        let next_pos = apply_offset_to_vector(pos, offset);
        match self.tile_at(next_pos) {
            &Entity::Floor => {
                let old_pos = pos;
                self.swap_tile(old_pos, next_pos);

                true
            }
            _ => false,
        }
    }

    fn move_player(&mut self, offset: (i32, i32)) {
        let old_pos = self.player_pos;
        let next_pos = apply_offset_to_vector(old_pos, offset);

        match *self.tile_at(next_pos) {
            Entity::Floor => {
                self.move_thing(old_pos, offset);
                self.player_pos = next_pos;
            }
            Entity::Crate => {
                let success = self.move_thing(next_pos, offset);
                if (success) {
                    self.move_player(offset);
                }
            }
            Entity::Wall => (),
            Entity::Player => (),
        }
    }

    fn to_index(p: (usize, usize)) -> usize {
        match p {
            (x, y) => (x + y * width) as usize,
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
            "w\n" => state.move_player((0, -1)),
            "a\n" => state.move_player((-1, 0)),
            "s\n" => state.move_player((0, 1)),
            "d\n" => state.move_player((1, 0)),
            _ => (),
        };
    }
}
