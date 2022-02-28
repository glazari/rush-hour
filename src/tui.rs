use crate::game::{self, Car, Game};

pub fn draw(game: &Game) {
    print!("\x1b[2J"); // clear screen
    print!("\x1b[0;0H"); // move cursor to top

    println!(" _____________ ");
    println!("| o o o o o o |");
    println!("| o o o o o o |");
    println!("| o o o o o o  ");
    println!("| o o o o o o |");
    println!("| o o o o o o |");
    println!("| o o o o o o |");
    println!(" _____________ ");

    for car in game.cars.iter() {
        draw_car(car);
    }
    print!("\x1b[10;10H\n"); // move cursor to bottom
    print!("\x1b[0m\n"); // reset
}

fn draw_car(car: &Car) {
    let (x, y) = (car.position.0 + 2, (car.position.1 * 2) + 3);
    print!("\x1b[{};{}H", x, y); // move cursor

    let color = Color::from_game(car.color);
    print!("\x1b[48;5;{}m\x1b[38;5;{}m", color.fg, color.bg); // set color

    if car.vertical {
        print!("^");
        print!("\x1b[1B\x1b[1D|"); // add | bellow
        if car.size == 3 {
            print!("\x1b[1B\x1b[1D|"); // add | bellow
        }
    } else {
        if car.size == 3 {
            print!("- - >");
        } else {
            print!("- >");
        }
    }
}

struct Color {
    fg: u8, // forground
    bg: u8, // background
}

impl Color {
    fn new(fg: u8, bg: u8) -> Color {
        Color { fg, bg }
    }

    fn from_game(c: game::Color) -> Color {
        match c {
            game::Color::RedMain => Color::new(1, 196),
            game::Color::Green => Color::new(22, 64),
            game::Color::PukeGreen => Color::new(64, 22),
            game::Color::Pink => Color::new(211, 163),
            game::Color::Purple => Color::new(55, 99),
            game::Color::LightPurple => Color::new(141, 93),
            game::Color::Grey => Color::new(241, 232),
            game::Color::Yellow => Color::new(11, 220),
            game::Color::LightYellow => Color::new(228, 214),
            game::Color::Blue => Color::new(12, 27),
            game::Color::SeaBlue => Color::new(24, 17),
            game::Color::BlueGreen => Color::new(31, 22),
            game::Color::Cyan => Color::new(49, 37),
            game::Color::Brown => Color::new(130, 214),
            game::Color::Orange => Color::new(202, 208),
            game::Color::Bege => Color::new(137, 94),
        }
    }
}

/*
 * ANSI codes
 *
 * RESET:           \x1b[0m
 * COLOR 256:       \x1b[38;5;{n}
 * BACKGROUND 256:  \x1b[48;5;{n}
 * MOVE:            \x1b[{n}{dir}
 *      dirs: A (UP), B (DOWN), C (RIGHT), D (LEFT)
 *
 * CLEAR SCREEN:    \x1b[2J
 * SET POSITION:    \x1b[{row};{column}H
 *
 *
 * Print all colors to explore
 *
 *   for i in 0..256 {
 *       print!("\x1b[48;5;{}m {} ", i, i);
 *       print!("\x1b[0m ");
 *   }
 *   println!("\x1b[0m");
 */
