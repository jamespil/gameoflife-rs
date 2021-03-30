use std::collections::HashMap;
use std::thread;
use std::time::Duration;



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

    matrix.insert((31, 12), State::ALIVE);
    matrix.insert((31, 13), State::ALIVE);

    // to here

    for _ in 0..128 {
        tick(&mut matrix);
        thread::sleep(Duration::from_millis(200));
    }
}

fn in_bounds<T: Ord>(n: T, min: T, max: T) -> bool {
    n <= max && n >= min
}

fn get_alive_neighbors(matrix: &mut Matrix, cell: Cell) -> u8 {
    let mut alive = 0;

    for y in -1..2 {
        for x in -1..2 {
            let neighbor_x = (cell.0 as i8 + x) as u8;
            let neighbor_y = (cell.1 as i8 + y) as u8;
            if in_bounds(neighbor_x, 0, 31) && in_bounds(neighbor_y, 0, 31)
            && (x, y) != (0, 0) && matrix[&(neighbor_x, neighbor_y)] == State::ALIVE {
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