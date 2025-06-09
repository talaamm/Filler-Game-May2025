use std::io::{self, BufRead}; /*self refers to the io module itself. By importing self, you can use io::stdin() and other items directly from the io module.
                              BufRead is a trait in the std::io module that provides methods for buffered reading, such as .lines(). By importing BufRead, you can call methods like .lines() on types that implement this trait (e.g., BufReader, or the result of stdin().lock()).
                              Summary:

                              self lets you use io::stdin().
                              BufRead lets you use .lines() and other buffered reading methods. */
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines(); /* stdin refers to the standard input stream (usually your keyboard).
                                          .lock() locks the standard input for safe, buffered reading (important in multithreaded contexts).
                                          .lines() creates an iterator that yields each line from the input as a Result<String, std::io::Error>.
                                          let mut lines declares a mutable variable named lines that will hold this iterator. The mut keyword means you can advance the iterator (e.g., with lines.next()).
                                          In summary:
                                          This line prepares to read lines from standard input, one at a time, in a way that handles errors and allows you to process each line as you go. */

    let player_line = lines.next().unwrap().unwrap(); /* Here is an example of the first input that the game_engine will send to player 1:

                                                      $$$ exec p1 : [robots/bender]
                                                      Anfield 20 15:
                                                          01234567890123456789
                                                      000 ....................
                                                      001 ....................
                                                      002 .........@..........
                                                      003 ....................
                                                      004 ....................
                                                      005 ....................
                                                      006 ....................
                                                      007 ....................
                                                      008 ....................
                                                      009 ....................
                                                      010 ....................
                                                      011 ....................
                                                      012 .........$..........
                                                      013 ....................
                                                      014 ....................
                                                      Piece 4 1:
                                                      .OO. */
    let player_num = if player_line.contains("p1") {
        1
    } else if player_line.contains("p2") {
        2
    } else {
        panic!("Could not determine player number");
    };

    loop {
        let mut line = String::new();
        while let Some(Ok(l)) = lines.next() {
            if l.starts_with("Anfield") {
                // Anfield 20 15:
                line = l;
                break;
            }
        }
        if line.is_empty() {
            break;
        }
        let mut parts = line.split_whitespace(); // { "Anfiled", "20" , "15:" }
        parts.next(); // skip "Anfield"
        let width: usize = parts.next().unwrap().parse().unwrap(); // 20 (represents only the dots in a row--> .........@..........)
        let height: usize = parts.next().unwrap().trim_end_matches(':').parse().unwrap(); // 15

        lines.next(); // skip the line with the grid index ("01234567890123456789")

        let mut anfield = Vec::new();
        for _ in 0..height {
            // _ to ignore the value of the loop variable
            // Read the next 'height' lines for the Anfield grid
            let row = lines.next().unwrap().unwrap(); // ex: 002 .........@..........
            let grid_row = row[4..4 + width].chars().collect::<Vec<char>>(); // start from index 4 until width+4 (to skip the index part)
            anfield.push(grid_row);
        }

        let mut piece_line = String::new();
        while let Some(Ok(l)) = lines.next() {
            // Some(OK(l))because lines.nexts() returns an Option<Result<String, std::io::Error>>
            if l.starts_with("Piece") {
                piece_line = l; // Piece 4 1:
                break;
            }
        }
        if piece_line.is_empty() {
            break;
        }

        let mut parts = piece_line.split_whitespace(); // { "Piece", "4", "1:" }
        parts.next(); // skip "Piece"

        let piece_width: usize = parts.next().unwrap().parse().unwrap(); // 4
        let piece_height: usize = parts.next().unwrap().trim_end_matches(':').parse().unwrap(); // 1

        let mut piece = Vec::new();
        for _ in 0..piece_height {
            // iterate 'piece_height' times to read the piece lines
            let row = lines.next().unwrap().unwrap(); //  .OO.
            piece.push(row.chars().collect::<Vec<char>>());
        }

        let valid_moves = find_valid_placements(&anfield, &piece, player_num); // get all valid moves

        let (my_syms, opp_syms) = if player_num == 1 {
            (vec!['@', 'a'], vec!['$', 's'])
        } else {
            (vec!['$', 's'], vec!['@', 'a'])
        };

        if valid_moves.is_empty() {
            println!("0 0");
            continue;
        }

        // For each valid move, calculate the minimum Manhattan distance to any opponent cell
        // The move with the smallest such distance is considered the "best" move
        let opponent_cells = find_opponent_cells(&anfield, &opp_syms);
        let best_move = valid_moves.iter().min_by_key(|&&(y, x)| {
            opponent_cells
                .iter()
                // For each opponent cell, calculate the Manhattan distance to this move (y, x)
                .map(|&(oy, ox)| manhattan_distance((y, x), (oy, ox)))
                // Take the minimum distance to any opponent cell for this move
                .min()
                // If there are no opponent cells, use usize::MAX as a fallback
                /* inner min(): For each valid move (y, x), you want to know:
                What is the closest opponent cell to this move?
                So, you calculate the Manhattan distance from (y, x) to every opponent cell, and .min() gives you the smallest distance (i.e., the closest opponent cell).
                This gives you a single number: the minimum distance from this move to any opponent. */
                .unwrap_or(usize::MAX)
                /*If there are no opponent cells on the board, .min() will return None.
                .unwrap_or(usize::MAX) means:
                If there is a minimum value, use it.
                If not (i.e., there are no opponent cells), use usize::MAX (a very large number), so this move will not be chosen over others that have a real minimum distance. */
        }); // min_by_key is used to pick the move with the smallest (minimum) Manhattan distance to any opponent cell.
            // The first finds the best (closest) opponent for each move.
            // The second finds the best move overall.

        /*If two or more moves have the same minimum key (e.g., both (4,7) and (4,11) have a Manhattan distance of 7), min_by_key will return the first one it encounters in the iterator. */
        if let Some(&(y, x)) = best_move {
            println!("{} {}", x, y);
        } else {
            println!("0 0");
        }
    }
}

// Calculates the Manhattan distance between two points (a and b) on the grid.
// Manhattan distance is the sum of the absolute differences of their coordinates.
fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1) // or (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn find_opponent_cells(anfield: &Vec<Vec<char>>, opp_syms: &[char]) -> Vec<(usize, usize)> {
    let mut cells = Vec::new();
    for (y, row) in anfield.iter().enumerate() {
        // .enumerate() gives you both the index (y) and the row itself.
        for (x, &cell) in row.iter().enumerate() {
            if opp_syms.contains(&cell) {
                cells.push((y, x));
            }
        }
    }
    cells
}
/*For each possible top-left position (y, x) on the Anfield where the piece could fit:
Try to overlay the piece.
Count how many cells of the piece overlap with your territory (must be exactly 1).
Ensure no piece cell overlaps with the opponent.
Ensure all piece cells are inside the Anfield.
If all conditions are met, add (y, x) to the list of valid placements.*/
fn find_valid_placements(
    anfield: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    player_num: usize,
) -> Vec<(usize, usize)> {
    let height = anfield.len();
    let width = anfield[0].len();

    let piece_height = piece.len();
    let piece_width = piece[0].len();

    let (my_syms, opp_syms) = if player_num == 1 {
        (vec!['@', 'a'], vec!['$', 's'])
    } else {
        (vec!['$', 's'], vec!['@', 'a'])
    };

    let mut valid = Vec::new();

    for y in 0..=height.saturating_sub(piece_height) {
        for x in 0..=width.saturating_sub(piece_width) {
            /*This iterates over all possible top-left positions (y, x) where the piece can fully fit inside the board.
            saturating_sub prevents underflow if the piece is larger than the board (gracefully results in 0). */
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
