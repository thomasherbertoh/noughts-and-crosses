mod noughts_and_crosses;

use eframe::NativeOptions;
use noughts_and_crosses::NoughtsAndCrosses;

fn main() {
    let app = NoughtsAndCrosses::default();
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Noughts and Crosses",
        native_options,
        Box::new(|_| Box::new(app)),
    );
}

/*
fn main() {
    let mut grid: Vec<Vec<Option<bool>>> = vec![vec![None; 3]; 3];
    println!("To make a move, enter a string `x y v` where `x` and `y` correspond to an `(x, y)` pair in the following grid, and `v` is either an `X` or an `O`");
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("({}, {})", i, j);
            if j < grid[i].len() - 1 {
                print!(", ");
            }
        }
        println!();
    }

    let mut won = false;
    while !won {
        let line: String = read!("{}\n");

        let coord_x;
        let coord_y;
        let user_move;

        match validate_input(line) {
            Ok(vec) => {
                coord_x = vec[0].to_digit(3).unwrap();
                coord_y = vec[1].to_digit(3).unwrap();
                user_move = if vec[2].eq_ignore_ascii_case(&'x') {
                    1
                } else {
                    0
                };
            }
            Err(e) => {
                eprintln!(
                    "Encountered the following error when validating input: '{}'",
                    e
                );
                continue;
            }
        }
        // input is valid
        println!(
            "coord_x = {}, coord_y = {}, user_move = {}",
            coord_x, coord_y, user_move
        );
        match insert_value(
            &mut grid,
            coord_x as usize,
            coord_y as usize,
            user_move == 1,
        ) {
            Ok(_) => {
                if check_win(&grid) {
                    println!("win");
                    won = true;
                }
                print_grid(&grid);
            }
            Err(e) => eprintln!(
                "Encountered the following error when performing the move `{}, {}, {}`: '{}'",
                coord_x, coord_y, user_move, e
            ),
        }
    }
}
*/
