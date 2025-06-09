// use std::fs::OpenOptions;
// use std::io::Write;
// use std::io::{self, BufRead};

// fn main() {
//     let stdin = io::stdin();
//     let mut lines = stdin.lock().lines();

//     // Read the player info line
//     let player_line = lines.next().unwrap().unwrap();
//     let player_num = if player_line.contains("p1") {
//         1
//     } else if player_line.contains("p2") {
//         2
//     } else {
//         panic!("Could not determine player number");
//     };

//     loop {
//         // Read until we find the Anfield line
//         let mut line = String::new();
//         while let Some(Ok(l)) = lines.next() {
//             if l.starts_with("Anfield") {
//                 line = l;
//                 break;
//             }
//         }
//         if line.is_empty() {
//             break;
//         }

//         // Parse Anfield size
//         // Example: "Anfield 20 15:"
//         let mut parts = line.split_whitespace();
//         parts.next(); // Skip "Anfield"
//         let width: usize = parts.next().unwrap().parse().unwrap();
//         let height: usize = parts.next().unwrap().trim_end_matches(':').parse().unwrap();

//         // Skip the header line (column numbers)
//         lines.next();

//         // Parse Anfield grid
//         let mut anfield = Vec::new();
//         for _ in 0..height {
//             let row = lines.next().unwrap().unwrap();
//             // Each row starts with a 3-digit number and a space, then the grid
//             let grid_row = row[4..4 + width].chars().collect::<Vec<char>>();
//             anfield.push(grid_row);
//         }

//         // Read until we find the Piece line (  Piece 4 1:   )
//         let mut piece_line = String::new();
//         while let Some(Ok(l)) = lines.next() {
//             if l.starts_with("Piece") {
//                 piece_line = l;
//                 break;
//             }
//         }
//         if piece_line.is_empty() {
//             break;
//         }

//         // Parse piece size
//         // Example: "Piece 4 1:"
//         let mut parts = piece_line.split_whitespace();
//         parts.next(); // Skip "Piece"
//         let piece_width: usize = parts.next().unwrap().parse().unwrap();
//         let piece_height: usize = parts.next().unwrap().trim_end_matches(':').parse().unwrap();

//         // Parse piece shape
//         let mut piece = Vec::new();
//         for _ in 0..piece_height {
//             let row = lines.next().unwrap().unwrap();
//             piece.push(row.chars().collect::<Vec<char>>());
//         }

//         // finds all possible placements of the piece on the Anfield
//         let valid_moves = find_valid_placements(&anfield, &piece, player_num);
//         if let Some(&(y, x)) = valid_moves.first() {
//             println!("{} {}", x, y);

//             // Write debug info to a file
//             let mut file = OpenOptions::new()
//                 .create(true)
//                 .append(true)
//                 .open("debug_output.txt")
//                 .unwrap();
//             writeln!(file, "{} {} \nDEBUG: {:#?}\n", x, y, valid_moves).unwrap();
//         } else {
//             println!("0 0");
//         }
      
//     }
// }

// /*For each possible top-left position (y, x) on the Anfield where the piece could fit:
// Try to overlay the piece.
// Count how many cells of the piece overlap with your territory (must be exactly 1).
// Ensure no piece cell overlaps with the opponent.
// Ensure all piece cells are inside the Anfield.
// If all conditions are met, add (y, x) to the list of valid placements.*/

// fn find_valid_placements(
//     anfield: &Vec<Vec<char>>,
//     piece: &Vec<Vec<char>>,
//     player_num: usize,
// ) -> Vec<(usize, usize)> {
//     let height = anfield.len();
//     let width = anfield[0].len();
//     let piece_height = piece.len();
//     let piece_width = piece[0].len();

//     // Player and opponent symbols
//     let (my_syms, opp_syms) = if player_num == 1 {
//         (vec!['@', 'a'], vec!['$', 's'])
//     } else {
//         (vec!['$', 's'], vec!['@', 'a'])
//     };

//     let mut valid = Vec::new();

//     for y in 0..=height.saturating_sub(piece_height) {
//         for x in 0..=width.saturating_sub(piece_width) {
//             let mut overlap = 0;
//             let mut valid_pos = true;
//             for dy in 0..piece_height {
//                 for dx in 0..piece_width {
//                     if piece[dy][dx] != '.' {
//                         let cell = anfield[y + dy][x + dx];
//                         if my_syms.contains(&cell) {
//                             overlap += 1;
//                         } else if opp_syms.contains(&cell) {
//                             valid_pos = false;
//                         }
//                     }
//                 }
//             }
//             if valid_pos && overlap == 1 {
//                 valid.push((y, x));
//             }
//         }
//     }
//     valid
// }
