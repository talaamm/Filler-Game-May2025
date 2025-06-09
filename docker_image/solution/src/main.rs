use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let (my_syms, opp_syms);
    let player_line = lines.next().unwrap().unwrap();
    let player_num = if player_line.contains("p1") {
        (my_syms, opp_syms) = (vec!['@', 'a'], vec!['$', 's']);
        1
    } else if player_line.contains("p2") {
        (my_syms, opp_syms) = (vec!['$', 's'], vec!['@', 'a']);
        2
    } else {
        panic!("Could not determine player number");
    };
    loop {
        // let mut line = String::new();
        let mut piece_line = String::new();
        let mut anfield = Vec::new();
        let mut piece = Vec::new();

        while let Some(Ok(l)) = lines.next() {
            if l.starts_with("Anfield") {
                line = l;
                let mut parts = line.split_whitespace();
                parts.next();
                let width: usize = parts.next().unwrap().parse().unwrap();
                let height: usize = parts.next().unwrap().trim_end_matches(':').parse().unwrap();
                lines.next();
                for _ in 0..height {
                    let row = lines.next().unwrap().unwrap();
                    let grid_row = row[4..4 + width].chars().collect::<Vec<char>>();
                    anfield.push(grid_row);
                }
            } else if l.starts_with("Piece") {
                piece_line = l;
                let mut parts = piece_line.split_whitespace();
                parts.next();
                let piece_width: usize = parts.next().unwrap().parse().unwrap();
                let piece_height: usize =
                    parts.next().unwrap().trim_end_matches(':').parse().unwrap();
                for _ in 0..piece_height {
                    let row = lines.next().unwrap().unwrap();
                    piece.push(row.chars().collect::<Vec<char>>());
                }
                break;
            }
        }
        if line.is_empty() || piece_line.is_empty() {
            break;
        }

        let valid_moves = find_valid_placements(&anfield, &piece, player_num, (my_syms.clone(), opp_syms.clone()));
        if valid_moves.is_empty() {
            println!("0 0");
            continue;
        }
        let opponent_cells = find_opponent_cells(&anfield, &opp_syms);
        let best_move = valid_moves.iter().min_by_key(|&&(y, x)| {
            opponent_cells
                .iter()
                .map(|&(oy, ox)| manhattan_distance((y, x), (oy, ox)))
                .min()
                .unwrap_or(usize::MAX)
        });
        if let Some(&(y, x)) = best_move {
            println!("{} {}", x, y);
        } else {
            println!("0 0");
        }
    }
}
fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}
fn find_opponent_cells(anfield: &Vec<Vec<char>>, opp_syms: &[char]) -> Vec<(usize, usize)> {
    let mut cells = Vec::new();
    for (y, row) in anfield.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if opp_syms.contains(&cell) {
                cells.push((y, x));
            }
        }
    }
    cells
}
fn find_valid_placements(
    anfield: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    player_num: usize,
    (my_syms, opp_syms): (Vec<char>, Vec<char>),
) -> Vec<(usize, usize)> {
    let height = anfield.len();
    let width = anfield[0].len();
    let piece_height = piece.len();
    let piece_width = piece[0].len();
    let mut valid = Vec::new();
    for y in 0..=height.saturating_sub(piece_height) {
        for x in 0..=width.saturating_sub(piece_width) {
            let mut overlap = 0;
            let mut valid_pos = true;
            for dy in 0..piece_height {
                for dx in 0..piece_width {
                    if piece[dy][dx] != '.' {
                        let cell = anfield[y + dy][x + dx];
                        if my_syms.contains(&cell) {
                            overlap += 1;
                        } else if opp_syms.contains(&cell) {
                            valid_pos = false;
                        }
                    }
                }
            }
            if valid_pos && overlap == 1 {
                valid.push((y, x));
            }
        }
    }
    valid
}
