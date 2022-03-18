use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Game {
    pub cars: Vec<Car>,
}

pub const V: Dir = Dir::V;
pub const H: Dir = Dir::H;

impl Game {
    fn is_finnished(&self) -> bool {
        for car in self.cars.iter() {
            if car.piece == Piece::Red && car.position == (2, 4) {
                return true;
            }
        }
        return false;
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        for car in self.cars.iter() {
            s.push_str(&car.to_string());
            s.push_str(",");
        }
        return s;
    }

    pub fn solve(&self) -> Option<Vec<(usize, Move)>> {
        self._solve_bfs()
    }

    fn _solve_bfs(&self) -> Option<Vec<(usize, Move)>> {
        let mut queue: VecDeque<(Game, Vec<(usize, Move)>)> =
            VecDeque::from([(self.clone(), vec![])]);
        let mut seen = HashSet::new();

        while queue.len() > 0 {
            let (game, moves) = queue.pop_front().expect("not empty");

            if game.is_finnished() {
                return Some(moves);
            }

            for (i, m) in game.possible_moves().iter() {
                let mut next_game = game.clone();
                let mut next_moves = moves.clone();
                next_game.apply_move(*i, *m);
                let gamestr = next_game.to_string();
                if seen.contains(&gamestr) {
                    continue;
                }
                seen.insert(gamestr);
                next_moves.push((*i, *m));
                queue.push_back((next_game, next_moves));
            }
        }

        None
    }

    #[allow(dead_code)]
    fn reverse_move(&mut self, i: usize, m: Move) {
        let m = match m {
            Move::Right => Move::Left,
            Move::Left => Move::Right,
            Move::Up => Move::Down,
            Move::Down => Move::Up,
        };
        self.apply_move(i, m);
    }

    #[allow(dead_code)]
    fn pause() {
        use std::io::stdin;
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Use press enter");
    }

    #[allow(dead_code)]
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

    pub fn apply_move(&mut self, i: usize, m: Move) {
        let car = self.cars[i];
        self.cars[i] = car.moved(m);
    }
}

// example games
impl Game {
    pub fn new(cars: Vec<Car>) -> Game {
        Game { cars }
    }
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

    pub fn hard_example_game() -> Game {
        Game {
            cars: vec![
                Car::new(V, (0, 0), Piece::YellowTruck),
                Car::new(H, (0, 1), Piece::Green),
                Car::new(V, (1, 1), Piece::Cyan),
                Car::new(V, (1, 2), Piece::Pink),
                Car::new(H, (3, 0), Piece::PurpleTruck),
                Car::new(H, (5, 0), Piece::Bege),
                Car::new(V, (4, 2), Piece::PukeGreen),
                Car::new(H, (2, 3), Piece::Red),
                Car::new(V, (3, 3), Piece::Purple),
                Car::new(H, (5, 3), Piece::Yellow),
                Car::new(V, (0, 4), Piece::Orange),
                Car::new(H, (4, 4), Piece::Grey),
                Car::new(V, (1, 5), Piece::PurpleTruck),
            ],
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Dir {
    V, //vertical
    H, //horizontal
}

impl Dir {
    fn to_string(&self) -> String {
        match self {
            Dir::V => "V".to_string(),
            Dir::H => "H".to_string(),
        }
    }

    fn from_string(s: &str) -> Dir {
        match s {
            "V" => Dir::V,
            "H" => Dir::H,
            _ => panic!("unkown direction"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub fn to_string(&self) -> String {
        match self {
            Piece::Bege => "H".to_string(),
            Piece::Brown => "J".to_string(),
            Piece::PukeGreen => "K".to_string(),
            Piece::BlueTruck => "Q".to_string(),
            Piece::PurpleTruck => "P".to_string(),
            Piece::GreenTruck => "R".to_string(),
            Piece::YellowTruck => "O".to_string(),
            Piece::Yellow => "I".to_string(),
            Piece::Green => "F".to_string(),
            Piece::Grey => "G".to_string(),
            Piece::SeaBlue => "A".to_string(),
            Piece::Orange => "B".to_string(),
            Piece::Cyan => "C".to_string(),
            Piece::Red => "X".to_string(),
            Piece::Pink => "D".to_string(),
            Piece::Purple => "E".to_string(),
        }
    }
    pub fn from_string(s: &str) -> Piece {
        match s {
            "H" => Piece::Bege,
            "J" => Piece::Brown,
            "K" => Piece::PukeGreen,
            "Q" => Piece::BlueTruck,
            "P" => Piece::PurpleTruck,
            "R" => Piece::GreenTruck,
            "O" => Piece::YellowTruck,
            "I" => Piece::Yellow,
            "F" => Piece::Green,
            "G" => Piece::Grey,
            "A" => Piece::SeaBlue,
            "B" => Piece::Orange,
            "C" => Piece::Cyan,
            "X" => Piece::Red,
            "D" => Piece::Pink,
            "E" => Piece::Purple,
            _ => panic!("unknown string"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

    fn moved(&self, m: Move) -> Car {
        let (row, col) = self.position;

        let pos = match m {
            Move::Up => (row - 1, col),
            Move::Down => (row + 1, col),
            Move::Left => (row, col - 1),
            Move::Right => (row, col + 1),
        };

        Car {
            piece: self.piece,
            dir: self.dir,
            position: pos,
        }
    }
    pub fn color(&self) -> Color {
        self.piece.color_size().0
    }

    pub fn size(&self) -> u8 {
        self.piece.color_size().1
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.piece.to_string(),
            self.dir.to_string(),
            position_to_string(self.position),
        )
    }

    pub fn from_string(s: &str) -> Car {
        let parts: Vec<&str> = s.split(' ').collect();

        if parts.len() < 3 {
            panic!("car needs 3 parts");
        }

        Car {
            piece: Piece::from_string(parts[0]),
            dir: Dir::from_string(parts[1]),
            position: position_from_string(parts[2]),
        }
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

fn position_to_string(p: (u8, u8)) -> String {
    format!("({}:{})", p.0, p.1)
}

fn position_from_string(s: &str) -> (u8, u8) {
    let patterns: &[_] = &['(', ')'];
    let nums: Result<Vec<u8>, _> = s
        .trim_matches(patterns)
        .split(':')
        .map(|c: &str| c.parse())
        .collect();
    let nums = nums.expect("perfectly parsable");

    if nums.len() < 2 {
        panic!("position needs 2 values");
    }
    (nums[0], nums[1])
}

#[cfg(test)]
mod test {
    use super::*;

    fn vec_compare<T: PartialEq>(va: &Vec<T>, vb: &Vec<T>) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| a == b)
    }

    #[test]
    fn car_to_string_test() {
        let c = Car::new(Dir::H, (3, 4), Piece::Red);

        assert_eq!("X H (3:4)", c.to_string());
        assert_eq!(c, Car::from_string(&c.to_string()));
    }

    #[test]
    fn position_to_string_test() {
        let p: (u8, u8) = (2, 3);

        assert_eq!("(2:3)", position_to_string(p));
        assert_eq!(p, position_from_string(&position_to_string(p)));
    }

    #[test]
    fn dir_to_string() {
        let d = Dir::H;
        assert_eq!(d, Dir::from_string(&d.to_string()));

        let d = Dir::V;
        assert_eq!(d, Dir::from_string(&d.to_string()));
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

    #[test]
    fn move_car() {
        let before = vec![
            Car::new(H, (0, 1), Piece::Bege),
            Car::new(H, (1, 1), Piece::Green),
            Car::new(V, (3, 1), Piece::Red),
            Car::new(V, (3, 2), Piece::Grey),
        ];
        let after = vec![
            Car::new(H, (0, 2), Piece::Bege),
            Car::new(H, (1, 0), Piece::Green),
            Car::new(V, (4, 1), Piece::Red),
            Car::new(V, (2, 2), Piece::Grey),
        ];

        let mut game = Game { cars: before };
        game.apply_move(0, Move::Right);
        game.apply_move(1, Move::Left);
        game.apply_move(2, Move::Down);
        game.apply_move(3, Move::Up);

        let got = game.cars;

        if !vec_compare(&got, &after) {
            println!("expected:  {:?}", after);
            println!("got:       {:?}", got);
            assert!(false);
        }
    }

    #[test]
    fn solve_game() {
        let solvable = vec![
            Car::new(H, (2, 3), Piece::Red),
            Car::new(V, (2, 5), Piece::Grey),
        ];

        let game = Game { cars: solvable };

        let got = game.solve().expect("solution");
        let expected = vec![(1, Move::Down), (0, Move::Right)];

        if !vec_compare(&got, &expected) {
            println!("expected:  {:?}", expected);
            println!("got:       {:?}", got);
            assert!(false);
        }
    }

    #[test]
    fn solve_example_game() {
        let game = Game::example_game();

        let got = game.solve().expect("solution");
        let expected = vec![
            (3, Move::Up),
            (5, Move::Left),
            (5, Move::Left),
            (6, Move::Down),
            (6, Move::Down),
            (2, Move::Right),
            (2, Move::Right),
            (2, Move::Right),
        ];

        if !vec_compare(&got, &expected) {
            println!("expected:  {:?}", expected);
            println!("got:       {:?}", got);
            assert!(false);
        }
    }

    #[test]
    fn solve_hard_example_game() {
        let game = Game::hard_example_game();

        let got = game.solve().expect("solution");
        let expected = vec![
            (9, Move::Right),
            (8, Move::Down),
            (4, Move::Right),
            (0, Move::Down),
            (0, Move::Down),
            (1, Move::Left),
            (3, Move::Up),
            (4, Move::Right),
            (2, Move::Down),
            (2, Move::Down),
            (7, Move::Left),
            (7, Move::Left),
            (10, Move::Down),
            (12, Move::Up),
            (4, Move::Right),
            (6, Move::Up),
            (5, Move::Right),
            (0, Move::Down),
            (7, Move::Left),
            (3, Move::Down),
            (1, Move::Right),
            (1, Move::Right),
            (1, Move::Right),
            (3, Move::Up),
            (7, Move::Right),
            (0, Move::Up),
            (0, Move::Up),
            (0, Move::Up),
            (5, Move::Left),
            (6, Move::Down),
            (4, Move::Left),
            (7, Move::Right),
            (2, Move::Up),
            (2, Move::Up),
            (2, Move::Up),
            (4, Move::Left),
            (4, Move::Left),
            (7, Move::Left),
            (8, Move::Up),
            (8, Move::Up),
            (8, Move::Up),
            (4, Move::Right),
            (0, Move::Down),
            (0, Move::Down),
            (4, Move::Right),
            (9, Move::Left),
            (11, Move::Left),
            (12, Move::Down),
            (1, Move::Right),
            (8, Move::Up),
            (7, Move::Right),
            (2, Move::Down),
            (2, Move::Down),
            (2, Move::Down),
            (7, Move::Left),
            (8, Move::Down),
            (1, Move::Left),
            (12, Move::Up),
            (4, Move::Right),
            (6, Move::Up),
            (5, Move::Right),
            (0, Move::Down),
            (7, Move::Left),
            (3, Move::Down),
            (1, Move::Left),
            (1, Move::Left),
            (1, Move::Left),
            (3, Move::Up),
            (7, Move::Right),
            (0, Move::Up),
            (5, Move::Left),
            (6, Move::Down),
            (4, Move::Left),
            (8, Move::Up),
            (7, Move::Right),
            (10, Move::Up),
            (7, Move::Right),
            (12, Move::Down),
            (12, Move::Down),
            (12, Move::Down),
            (7, Move::Right),
        ];

        if !vec_compare(&got, &expected) {
            println!("expected:  {:?}", expected);
            println!("got:       {:?}", got);
            assert!(false);
        }
    }
}
