mod game;
mod tui;

use game::{Car, Move, Piece, H, V};

fn main() {
    tui::clear();

    let game = game::Game::full_grid();
    tui::draw((3, 10), &game);

    let game = game::Game::example_game();
    tui::draw((12, 10), &game);

    tui::draw_win((21, 10), &game);

    let game = game::Game::new(vec![
        Car::new(H, (2, 3), Piece::Red),
        Car::new(V, (2, 5), Piece::Yellow),
    ]);

    let moves = vec![(1, Move::Down), (0, Move::Right)];
    tui::animate_game(game, moves);
}
