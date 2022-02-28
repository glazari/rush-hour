mod game;
mod tui;

// use game::Car;

fn main() {
    let game = game::Game::full_grid();
    tui::draw(&game);
}
