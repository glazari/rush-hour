use std::collections::HashSet;

pub struct Game {
    pub cars: Vec<Car>,
}

const V: Dir = Dir::V;
const H: Dir = Dir::H;

impl Game {
    fn invalid(&self) -> Option<Error> {
        // check duplicates
        let mut set: HashSet<Piece> = HashSet::new();
        for car in self.cars.iter() {
            if set.contains(&car.piece) {
                return Some(Error::DuplicatePiece);
            }
            set.insert(car.piece);
        }

        // check out of bounds
        for car in self.cars.iter() {
            if Game::car_out_of_bound(car) {
                return Some(Error::OutOfBounds);
            }
        }

        None
    }

    fn car_out_of_bound(car: &Car) -> bool {
        if car.position.0 > 5 || car.position.1 > 5 {
            return true;
        }

        match car.dir {
            Dir::H => {
                if car.position.1 + car.size() > 5 {
                    return true;
                }
            }
            Dir::V => {
                if car.position.0 + car.size() > 5 {
                    return true;
                }
            }
        }
        return false;
    }

    fn possible_moves(&self) -> Vec<(usize, Move)> {
        let mut out = vec![];
        let mut occupied: HashSet<(u8, u8)> = HashSet::new();

        // build up occupied set
        // for car in self.cars.iter()
        for car in self.cars.iter() {
            let (row, col) = car.position;
            for i in 0..car.size() {
                match car.dir {
                    Dir::V => occupied.insert((row + i, col)),
                    Dir::H => occupied.insert((row, col + i)),
                };
            }
        }

        //println!("occupied:\n{:?}", occupied);

        for (i, car) in self.cars.iter().enumerate() {
            let (row, col) = car.position;
            //println!("{}: {:?}", i, car);
            match car.dir {
                Dir::V => {
                    // Tile to the top (of full car) needs to be free
                    if row != 0 && !occupied.contains(&(row - 1, col)) {
                        out.push((i, Move::Up))
                    }

                    // tile to the bottom (of full car) needs to be free
                    let bottom_tile = row + car.size() - 1;
                    if bottom_tile != 5 && !occupied.contains(&(bottom_tile + 1, col)) {
                        out.push((i, Move::Down))
                    }
                }
                Dir::H => {
                    // tile to the Left (of full car) needs to be free
                    if col != 0 && !occupied.contains(&(row, col - 1)) {
                        out.push((i, Move::Left));
                    }

                    // Tile to the right (of full car) needs to be free
                    let right_side = col + car.size() - 1;
                    if right_side != 5 && !occupied.contains(&(row, right_side + 1)) {
                        out.push((i, Move::Right));
                    }
                }
            }
        }

        out
    }
}

// example games
impl Game {
    pub fn full_grid() -> Game {
        Game {
            cars: vec![
                Car::new(H, (0, 0), Piece::Bege),
                Car::new(V, (1, 0), Piece::Brown),
                Car::new(V, (1, 1), Piece::PukeGreen),
                Car::new(V, (0, 2), Piece::BlueTruck),
                Car::new(V, (0, 3), Piece::PurpleTruck),
                Car::new(V, (0, 4), Piece::GreenTruck),
                Car::new(V, (0, 5), Piece::YellowTruck),
                Car::new(V, (3, 0), Piece::Yellow),
                Car::new(V, (3, 1), Piece::Green),
                Car::new(V, (3, 2), Piece::Grey),
                Car::new(V, (3, 3), Piece::SeaBlue),
                Car::new(V, (3, 4), Piece::Orange),
                Car::new(V, (3, 5), Piece::Cyan),
                Car::new(H, (5, 0), Piece::Red),
                Car::new(H, (5, 2), Piece::Pink),
                Car::new(H, (5, 4), Piece::Purple),
            ],
        }
    }

    pub fn example_game() -> Game {
        Game {
            cars: vec![
                Car::new(V, (0, 0), Piece::PurpleTruck),
                Car::new(H, (0, 1), Piece::PukeGreen),
                Car::new(H, (2, 1), Piece::Red),
                Car::new(V, (4, 0), Piece::Orange),
                Car::new(H, (4, 1), Piece::Cyan),
                Car::new(H, (5, 2), Piece::GreenTruck),
                Car::new(V, (1, 3), Piece::BlueTruck),
                Car::new(V, (3, 5), Piece::YellowTruck),
            ],
        }
    }
}

#[derive(Debug)]
pub enum Dir {
    V, //vertical
    H, //horizontal
}

#[derive(Debug, PartialEq)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Piece {
    Bege,
    Brown,
    PukeGreen,
    BlueTruck,
    PurpleTruck,
    GreenTruck,
    YellowTruck,
    Yellow,
    Green,
    Grey,
    SeaBlue,
    Orange,
    Cyan,
    Red,
    Pink,
    Purple,
}

impl Piece {
    pub fn color_size(&self) -> (Color, u8) {
        match self {
            Piece::Bege => (Color::Bege, 2),
            Piece::Brown => (Color::Brown, 2),
            Piece::PukeGreen => (Color::PukeGreen, 2),
            Piece::BlueTruck => (Color::Blue, 3),
            Piece::PurpleTruck => (Color::LightPurple, 3),
            Piece::GreenTruck => (Color::BlueGreen, 3),
            Piece::YellowTruck => (Color::Yellow, 3),
            Piece::Yellow => (Color::LightYellow, 2),
            Piece::Green => (Color::Green, 2),
            Piece::Grey => (Color::Grey, 2),
            Piece::SeaBlue => (Color::SeaBlue, 2),
            Piece::Orange => (Color::Orange, 2),
            Piece::Cyan => (Color::Cyan, 2),
            Piece::Red => (Color::RedMain, 2),
            Piece::Pink => (Color::Pink, 2),
            Piece::Purple => (Color::Purple, 2),
        }
    }
}

#[derive(Debug)]
pub struct Car {
    pub piece: Piece,
    pub dir: Dir,
    pub position: (u8, u8),
}

impl Car {
    pub fn new(dir: Dir, position: (u8, u8), piece: Piece) -> Car {
        Car {
            piece,
            dir,
            position,
        }
    }

    pub fn win_red() -> Car {
        Car {
            piece: Piece::Red,
            dir: Dir::H,
            position: (2, 7),
        }
    }

    pub fn color(&self) -> Color {
        self.piece.color_size().0
    }

    pub fn size(&self) -> u8 {
        self.piece.color_size().1
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    RedMain,
    Green,
    PukeGreen,
    Pink,
    Purple,
    LightPurple,
    Grey,
    Yellow,
    LightYellow,
    Blue,
    SeaBlue,
    BlueGreen,
    Cyan,
    Brown,
    Orange,
    Bege,
}

#[derive(Debug, PartialEq)]
enum Error {
    DuplicatePiece,
    OutOfBounds,
    PieceOverlap,
}

#[cfg(test)]
mod test {
    use super::*;

    fn vec_compare(va: &Vec<(usize, Move)>, vb: &Vec<(usize, Move)>) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| a == b)
    }

    #[test]
    fn game_valid() {
        let game = Game {
            cars: vec![Car::new(H, (0, 0), Piece::Bege)],
        };

        assert_eq!(game.invalid(), None);
    }

    #[test]
    fn game_invalid_duplicate_piece() {
        let game = Game {
            cars: vec![
                Car::new(H, (0, 0), Piece::Bege),
                Car::new(H, (0, 0), Piece::Bege),
            ],
        };

        assert_eq!(game.invalid().unwrap(), Error::DuplicatePiece);
    }

    #[test]
    #[ignore]
    fn game_invalid_piece_overlap() {
        // horizontal rigth overlap
        let game = Game {
            cars: vec![
                Car::new(H, (0, 0), Piece::Bege),
                Car::new(H, (0, 1), Piece::Green),
            ],
        };

        assert_eq!(game.invalid().unwrap(), Error::PieceOverlap);

        // horizontal rigth overlap truck
        // horizontal left overlap
        // horizontal left overlap truck
        // vertical down overlap
        // vertical down overlap truck
        // vertical up overlap
        // vertical up overlap truck
    }

    #[test]
    fn game_invalid_out_of_bounds() {
        // invalid cars
        let invalid_cars = vec![
            Car::new(H, (6, 0), Piece::Bege),      // row index oob
            Car::new(H, (0, 6), Piece::Bege),      // col index oob
            Car::new(V, (5, 0), Piece::Bege),      // row + size oob
            Car::new(V, (4, 0), Piece::BlueTruck), // row + size oob
            Car::new(H, (0, 5), Piece::Bege),      // row + size oob
            Car::new(H, (0, 4), Piece::BlueTruck), // row + size oob
        ];

        for car in invalid_cars.iter() {
            println!("{:?}", car);
            assert!(Game::car_out_of_bound(car))
        }

        // invalid game
        let game = Game {
            cars: vec![Car::new(H, (6, 0), Piece::Bege)],
        };

        assert_eq!(game.invalid().unwrap(), Error::OutOfBounds);
    }

    #[test]
    fn possible_moves_horizontal() {
        let game = Game {
            cars: vec![Car::new(H, (0, 1), Piece::Bege)],
        };

        let expected: Vec<(usize, Move)> = vec![(0, Move::Left), (0, Move::Right)];
        let got = game.possible_moves();

        if !vec_compare(&got, &expected) {
            println!("expected:  {:?}", expected);
            println!("got:       {:?}", got);
            assert!(false);
        }
    }

    #[test]
    fn possible_moves_truck_blocks_car() {
        // vertival truck blocks horizontal car
        let game = Game {
            cars: vec![
                Car::new(H, (3, 1), Piece::Bege),
                Car::new(V, (1, 3), Piece::BlueTruck),
            ],
        };

        let expected: Vec<(usize, Move)> = vec![(0, Move::Left), (1, Move::Up), (1, Move::Down)];
        let got = game.possible_moves();

        if !vec_compare(&got, &expected) {
            println!("expected:  {:?}", expected);
            println!("got:       {:?}", got);
            assert!(false);
        }
    }
}
