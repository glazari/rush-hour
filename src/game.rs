pub struct Game {
    pub cars: Vec<Car>,
}

impl Game {
    pub fn full_grid() -> Game {
        Game {
            cars: vec![
                Car::new(Color::Bege, false, (0, 0)),
                Car::new(Color::Brown, true, (1, 0)),
                Car::new(Color::PukeGreen, true, (1, 1)),
                Car::new(Color::Blue, true, (0, 2)),
                Car::new(Color::LightPurple, true, (0, 3)),
                Car::new(Color::BlueGreen, true, (0, 4)),
                Car::new(Color::Yellow, true, (0, 5)),
                Car::new(Color::LightYellow, true, (3, 0)),
                Car::new(Color::Green, true, (3, 1)),
                Car::new(Color::Grey, true, (3, 2)),
                Car::new(Color::SeaBlue, true, (3, 3)),
                Car::new(Color::Orange, true, (3, 4)),
                Car::new(Color::Cyan, true, (3, 5)),
                Car::new(Color::RedMain, false, (5, 0)),
                Car::new(Color::Pink, false, (5, 2)),
                Car::new(Color::Purple, false, (5, 4)),
            ],
        }
    }
}

pub struct Car {
    pub color: Color,
    pub vertical: bool,
    pub position: (u8, u8),
}

impl Car {
    pub fn new(color: Color, vertical: bool, position: (u8, u8)) -> Car {
        Car {
            color,
            vertical,
            position,
        }
    }
}

#[derive(Clone, Copy)]
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
