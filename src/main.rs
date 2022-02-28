fn main() {
    print!("\x1b[2J"); // clear screen
    print!("\x1b[0;0H"); // move cursor to top

    println!(" _________ ");
    println!("| o o o o |");
    println!("| o o o o  ");
    println!("| o o o o |");
    println!("| o o o o |");
    println!(" _________ ");

    // draw horizontal car
    print!("\x1b[2;3H"); // move cursor
    print!("\x1b[48;5;1m\x1b[38;5;196m-->\x1b[0m");

    // draw vertical car
    print!("\x1b[4;5H"); // move cursor
    println!("\x1b[48;5;1m\x1b[38;5;196m^\x1b[1B\x1b[1D|\x1b[1B\x1b[0m");

    print!("\x1b[10;10H\n"); // move cursor to bottom
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
 */
