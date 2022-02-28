pub struct Game {
    pub cars: Vec<Car>,
}

impl Game {}

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
