use eframe::Frame;
use egui::{warn_if_debug_build, CentralPanel, Context, Event, Grid, Key, Label, Sense, Visuals};

pub struct NoughtsAndCrosses {
    grid: Vec<Vec<Option<bool>>>,
}

impl Default for NoughtsAndCrosses {
    fn default() -> Self {
        Self {
            grid: vec![vec![None; 3]; 3],
        }
    }
}

impl eframe::App for NoughtsAndCrosses {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.set_visuals(Visuals::dark());
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("noughts and crosses :)");
            Grid::new("noughts_and_crosses_grid")
                .striped(true)
                .show(ui, |ui| {
                    for (i, row) in self.grid.clone().iter().enumerate() {
                        for (j, cell) in row.iter().enumerate() {
                            let text;
                            if let Some(c) = cell {
                                text = if *c { 'X' } else { 'O' };
                            } else {
                                text = '-';
                            };
                            if ui
                                .add(Label::new(text.to_string()).sense(Sense::click()))
                                .clicked()
                            {
                                println!("clicked cell at ({}, {})", i, j);
                                let events = ui.input().events.clone();
                                let mut user_move = None;
                                for event in events {
                                    match event {
                                        Event::Key {
                                            key: Key::X,
                                            pressed: true,
                                            modifiers: _,
                                        } => {
                                            println!("pressed x");
                                            user_move = Some(true);
                                        }
                                        Event::Key {
                                            key: Key::O,
                                            pressed: true,
                                            modifiers: _,
                                        } => {
                                            println!("pressed o");
                                            user_move = Some(false);
                                        }
                                        _ => {}
                                    }
                                }
                                match user_move {
                                    Some(um) => match insert_value(&mut self.grid, i, j, um) {
                                        Ok(_) => {
                                            if check_win(&self.grid) {
                                                println!("win");
                                                frame.quit();
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Error encountered: `({}, {}) => {}`: '{}'",
                                                i, j, true, e
                                            )
                                        }
                                    },
                                    None => {}
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
            warn_if_debug_build(ui);
        });
    }
}

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

fn _print_grid(grid: &Vec<Vec<Option<bool>>>) {
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

fn _validate_input(inp: String) -> Result<Vec<char>, String> {
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
