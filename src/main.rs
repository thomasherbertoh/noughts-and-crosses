#[macro_use]
extern crate text_io;

fn insert_value(
    grid: &mut Vec<Vec<Option<bool>>>,
    x: usize,
    y: usize,
    val: bool,
) -> Result<(), String> {
    if x >= grid.len() || y >= grid[x].len() {
        return Err(format!("cell `({}, {})` does not exist", x, y).to_string());
    }
    match grid[x][y] {
        None => {
            grid[x][y] = Some(val);
            Ok(())
        }
        Some(cell) => Err(format!(
            "cell `({}, {})` already contains `{}`",
            x,
            y,
            || -> char {
                if cell {
                    'X'
                } else {
                    'O'
                }
            }()
        )
        .to_string()),
    }
}

fn print_grid(grid: &Vec<Vec<Option<bool>>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                None => print!("-"),
                Some(c) => {
                    if c {
                        print!("X");
                    } else {
                        print!("O");
                    }
                }
            }
            if j < grid[i].len() - 1 {
                print!(", ");
            }
        }
        println!("");
    }
    println!("");
}

fn check_win(grid: &Vec<Vec<Option<bool>>>) -> bool {
    /*
    win conditions:
        one or more of:
            horizontal line identical
            vertical line identical
            diagonal line identical
    */

    let winning_states = vec![
        vec![vec![true; 3], vec![false; 3], vec![false; 3]], // first row
        vec![vec![false; 3], vec![true; 3], vec![false; 3]], // second row
        vec![vec![false; 3], vec![false; 3], vec![true; 3]], // third row
        vec![vec![true, false, false]; 3],                   // first column
        vec![vec![false, true, false]; 3],                   // second column
        vec![vec![false, false, true]; 3],                   // third column
        vec![
            // left-right diagonal
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, true],
        ],
        vec![
            // right-left diagonal
            vec![false, false, true],
            vec![false, true, false],
            vec![true, false, false],
        ],
    ];

    for state in winning_states {
        let mut winning = 0;
        for i in 0..state.len() {
            for j in 0..state[i].len() {
                if !state[i][j] {
                    continue;
                }
                match grid[i][j] {
                    None => continue,
                    Some(_) => {}
                }
                if grid[i][j].unwrap() {
                    winning += 1;
                } else {
                    winning -= 1;
                }
            }
        }
        if winning == 3 || winning == -3 {
            return true;
        }
    }

    false
}

fn validate_input(inp: String) -> Result<Vec<char>, String> {
    let values = inp.trim().split_ascii_whitespace().collect::<Vec<&str>>();
    for value in &values {
        if value.len() > 1 {
            return Err(format!("{} is invalid", value));
        }
    }
    // check coords are valid
    if !values[0].chars().nth(0).unwrap().is_digit(3)
        || !values[1].chars().nth(0).unwrap().is_digit(3)
    {
        return Err("Coordinates must be in the range `[0, 2]`".to_string());
    } else {
        let chars = vec!['o', 'x'];
        if !chars
            .iter()
            .any(|c| c.eq_ignore_ascii_case(&values[2].chars().nth(0).unwrap()))
        {
            return Err(format!(
                "Character wasn't `x` or `o`, but `{}`",
                values[2].chars().nth(0).unwrap()
            ));
        }
    }
    Ok(values.iter().map(|v| v.chars().nth(0).unwrap()).collect())
}

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
