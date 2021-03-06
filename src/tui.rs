use crate::game::{self, Car, Dir, Game, Move};
use std::io::stdin;

pub fn clear() {
    print!("\x1b[2J"); // clear screen
}

pub fn draw(base: (u8, u8), game: &Game) {
    draw_base_board(base);

    for car in game.cars.iter() {
        draw_car(base, car);
    }
    print!("\x1b[{};{}H\n", base.0 + 7, base.1 + 15); // move cursor to bottom
    print!("\x1b[0m\n"); // reset
}

fn draw_base_board(base: (u8, u8)) {
    print!("\x1b[{};{}H", base.0, base.1); // move cursor to top
    print!(" _____________ \x1b[1B\x1b[15D");
    print!("| o o o o o o |\x1b[1B\x1b[15D");
    print!("| o o o o o o |\x1b[1B\x1b[15D");
    print!("| o o o o o o  \x1b[1B\x1b[15D");
    print!("| o o o o o o |\x1b[1B\x1b[15D");
    print!("| o o o o o o |\x1b[1B\x1b[15D");
    print!("| o o o o o o |\x1b[1B\x1b[15D");
    print!(" _____________ \x1b[1B\x1b[15D");
}

pub fn draw_win(base: (u8, u8), game: &Game) {
    draw_base_board(base);

    for car in game.cars.iter() {
        match car.piece {
            game::Piece::Red => draw_car(base, &Car::win_red()),
            _ => draw_car(base, car),
        }
    }
    print!("\x1b[{};{}H\n", base.0 + 7, base.1 + 15); // move cursor to bottom
    print!("\x1b[0m\n"); // reset
}

fn draw_car(base: (u8, u8), car: &Car) {
    let x = base.0 + car.position.0 + 1;
    let y = base.1 + (car.position.1 * 2) + 2;
    print!("\x1b[{};{}H", x, y); // move cursor

    let color = Color::from_game(car.color());
    print!("\x1b[48;5;{}m\x1b[38;5;{}m", color.fg, color.bg); // set color

    match car.dir {
        Dir::V => {
            print!("^");
            print!("\x1b[1B\x1b[1D|"); // add | bellow
            if car.size() == 3 {
                print!("\x1b[1B\x1b[1D|"); // add | bellow
            }
        }
        Dir::H => {
            if car.size() == 3 {
                print!("- - >");
            } else {
                print!("- >");
            }
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

pub fn animate_game(mut game: Game, moves: Vec<(usize, Move)>) {
    let origin = (3, 10);
    clear();

    draw(origin, &game);
    pause();
    for (i, m) in moves.iter() {
        game.apply_move(*i, *m);
        draw(origin, &game);
        pause();
    }
    draw_win(origin, &game);
}

fn pause() {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Use press enter");
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
