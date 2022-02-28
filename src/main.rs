mod game;
mod tui;

fn main() {
    let game = game::Game {
        cars: vec![
            game::Car::new(1, 196, false, (0, 0)),
            game::Car::new(22, 64, true, (2, 2)),
        ],
    };
    tui::draw(&game);
}
