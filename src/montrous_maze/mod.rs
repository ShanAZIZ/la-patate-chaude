use std::collections::{HashSet, VecDeque};

pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u32,
}

pub struct MonstrousMazeOutput {
    pub path: String,
}

struct Maze {
    grid: Vec<Vec<char>>,
    start_pos : (usize, usize),
    end_pos : (usize, usize)
}

pub fn find_path(input: &MonstrousMazeInput) {
    let maze = from_str_to_vec(&input.grid);
    let path = write_path(maze.grid, maze.start_pos, maze.end_pos, input.endurance as i32);
    match path {
        Some(x) => MonstrousMazeOutput{
            path: x,
        },
        None => MonstrousMazeOutput {
            path: "".to_string() // TODO: manage another way
        }
    };
}

fn from_str_to_vec(grid: &String) -> Maze {
    let rows: Vec<&str> = grid.split('\n').collect(); // Split the grid string into rows
    let mut grid: Vec<Vec<char>> = vec![vec![' '; rows[0].len()]; rows.len()]; // Initialize 2D array with empty spaces
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    for (i, row) in rows.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            if ch == 'Y' {
                start_pos = (i, j)
            }
            if ch == 'X' {
                end_pos = (i, j);
            }
            grid[i][j] = ch;
        }
    }
    Maze {
        grid,
        start_pos,
        end_pos
    }
}

fn write_path(maze: Vec<Vec<char>>, depart: (usize, usize), arrivee: (usize, usize), endurance: i32) -> Option<String> {
    let mut queue: VecDeque<(usize, usize, String, i32)> = VecDeque::new();
    let mut visites: Vec<Vec<bool>> = vec![vec![false; maze[0].len()]; maze.len()];

    queue.push_back((depart.0, depart.1, "".to_string(), endurance));
    visites[depart.0][depart.1] = true;

    while let Some((ligne, colonne, chemin, endurance_actuelle)) = queue.pop_front() {
        if (ligne, colonne) == arrivee {
            return Some(chemin);
        }

        let deplacements: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dl, dc) in deplacements.iter() {
            let (nouvelle_ligne, nouvelle_colonne) = (ligne as i32 + dl, colonne as i32 + dc);
            if nouvelle_ligne >= 0 && nouvelle_ligne < maze.len() as i32 && nouvelle_colonne >= 0 && nouvelle_colonne < maze[0].len() as i32 {
                let (nouvelle_ligne, nouvelle_colonne) = (nouvelle_ligne as usize, nouvelle_colonne as usize);
                if !visites[nouvelle_ligne][nouvelle_colonne] && maze[nouvelle_ligne][nouvelle_colonne] == ' ' || maze[nouvelle_ligne][nouvelle_colonne] == 'M' || maze[nouvelle_ligne][nouvelle_colonne] == 'X' {
                    let mut nouveau_chemin = chemin.clone();
                    match (dl, dc) {
                        (0, 1) => nouveau_chemin.push('>'),
                        (0, -1) => nouveau_chemin.push('<'),
                        (1, 0) => nouveau_chemin.push('v'),
                        (-1, 0) => nouveau_chemin.push('^'),
                        _ => unreachable!(),
                    }

                    let nouvelle_endurance = if maze[nouvelle_ligne][nouvelle_colonne] == 'M' {
                        endurance_actuelle - 1
                    } else {
                        endurance_actuelle
                    };

                    if nouvelle_endurance > 0 {
                        queue.push_back((nouvelle_ligne, nouvelle_colonne, nouveau_chemin, nouvelle_endurance));
                        visites[nouvelle_ligne][nouvelle_colonne] = true;
                    }
                }
            }
        }
    }

    None
}


fn dir(dx: i32, dy: i32) -> &'static str {
    match (dx, dy) {
        (0, 1) => ">",
        (0, -1) => "<",
        (1, 0) => "v",
        (-1, 0) => "^",
        _ => unreachable!(),
    }
}