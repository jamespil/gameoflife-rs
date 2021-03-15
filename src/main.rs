use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::cmp;



// Experimental game of life simulator by james
// no tutorials used

type Matrix = HashMap<Cell, State>;

type Cell = (u8, u8);

// #[derive(PartialEq)]
// enum STATE {
//     DEAD,
//     P1,
//     P2
// }

#[derive(PartialEq)]
enum State {
    DEAD,
    ALIVE,
}

// impl State {
//     fn symbol(&self) -> char {
//         match self {
//             Cell::DEAD => ' ',
//             Cell::P1 => 'X',
//             Cell::P2 => 'O'
//         }
//     }
// }

impl State {
    fn symbol(&self) -> char {
        match self {
            State::DEAD => ' ',
            State::ALIVE => '*',
        }
    }
}

fn main() {
    let mut matrix: Matrix = HashMap::new();

    for y in 0..32 {
        for x in 0..32 {
            matrix.insert((x, y), State::DEAD);
        }
    }

    // for now, insert starting cells here

    matrix.insert((14, 12), State::ALIVE);
    matrix.insert((15, 12), State::ALIVE);
    matrix.insert((16, 12), State::ALIVE);

    matrix.insert((14, 13), State::ALIVE);
    matrix.insert((16, 13), State::ALIVE);

    matrix.insert((14, 14), State::ALIVE);
    matrix.insert((15, 14), State::ALIVE);
    matrix.insert((16, 14), State::ALIVE);

    matrix.insert((14, 15), State::ALIVE);
    matrix.insert((15, 15), State::ALIVE);
    matrix.insert((16, 15), State::ALIVE);

    matrix.insert((14, 16), State::ALIVE);
    matrix.insert((15, 16), State::ALIVE);
    matrix.insert((16, 16), State::ALIVE);

    matrix.insert((14, 17), State::ALIVE);
    matrix.insert((15, 17), State::ALIVE);
    matrix.insert((16, 17), State::ALIVE);

    matrix.insert((14, 18), State::ALIVE);
    matrix.insert((16, 18), State::ALIVE);

    matrix.insert((14, 19), State::ALIVE);
    matrix.insert((15, 19), State::ALIVE);
    matrix.insert((16, 19), State::ALIVE);

    // to here

    for _ in 0..128 {
        tick(&mut matrix);
        thread::sleep(Duration::from_millis(200));
    }
}

fn clamp<T: Ord>(n: T, min: T, max: T) -> T {
    cmp::max(cmp::min(n, max), min)
}

fn get_alive_neighbors(matrix: &mut Matrix, cell: Cell) -> u8 {
    let mut alive = 0;

    for y in -1..2 {
        for x in -1..2 {
            if (x, y) != (0, 0) && matrix[&(clamp(cell.0 as i8 + x, 0, 31) as u8, clamp(cell.1 as i8 + y, 0, 31) as u8)] == State::ALIVE {
                alive += 1;
            }
        }
    }

    alive
}

fn tick(matrix: &mut Matrix) {
    let mut matrix_new: Matrix = HashMap::new();
    let mut string = "\x1B[2J\x1B[1;1H".to_owned();

    for y in 0..32 {
        for x in 0..32 {
            let state = match matrix[&(x, y)] {
                State::DEAD => {
                    if get_alive_neighbors(matrix, (x, y)) == 3 {
                        State::ALIVE
                    } else {
                        State::DEAD
                    }
                },
                State::ALIVE => {
                    let neighbors = get_alive_neighbors(matrix, (x, y));
                    if neighbors < 2 {
                        State::DEAD
                    } else if neighbors == 2 || neighbors == 3 {
                        State::ALIVE
                    } else if neighbors > 3 {
                        State::DEAD
                    } else {
                        panic!("neighbors??");
                    }
                }
            };

            matrix_new.insert((x, y), state);

            if x == 0 {
                string.push('|');
            }
            string.push(matrix[&(x, y)].symbol());
            string.push(' ');
            if x == 31 {
                string.push_str("|\n");
            }
        }
    }

    *matrix = matrix_new;
    print!("{}", string);
}
