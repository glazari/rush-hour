pub struct Game {
    pub cars: Vec<Car>,
}

const V: Dir = Dir::V;
const H: Dir = Dir::H;

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

#[derive(Debug)]
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
    piece: Piece,
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
