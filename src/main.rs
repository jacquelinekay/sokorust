/*
 * */
use std::io;

const width = 100;

#[derive (Copy, Clone)]
enum Entity {
    Player,
    Wall,
    Crate,
    Floor,
}

struct State {
    board: [Entity; 100],
    player_pos: (i32, i32),
}

impl State {
    fn new() -> Self {
        State {
            board: [Entity::Floor; 100],
            player_pos: (-1, -1),
        }
    }

    fn print(&self) {
        // x = i % 10
        // y = i / 10
        for y in 0..10 {
            for x in 0..10 {
                let entity = &self.board[x + y * 10];
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
        for i in 0..100 {
            state.board[i] = match x.chars().nth(i).unwrap() {
                '@' => {
                    state.player_pos = ((i % 10) as i32, (i / 10) as i32);
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

    fn move_player(&mut self, offset: (i32, i32)) {
        self.board[Self::to_index(self.player_pos)] = Entity::Floor;
        self.player_pos = match (self.player_pos, offset) {
            ((x, y), (x2, y2)) => (x + x2, y + y2),
        };
        self.board[Self::to_index(self.player_pos)] = Entity::Player;
    }

    fn to_index(p: (i32, i32)) -> usize {
        match p {
            (x, y) => (x + y * 10) as usize,
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
                                 #        #\
                                 #        #\
                                 #        #\
                                 ##########")
            .unwrap();

    loop {
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
