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
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0), (3, 4)), 7);
        assert_eq!(manhattan_distance((2, 5), (2, 5)), 0);
        assert_eq!(manhattan_distance((1, 2), (4, 6)), 7);
    }
    #[test]
    fn test_manhattan_distance_edge_cases() {
        assert_eq!(manhattan_distance((0, 0), (0, 0)), 0); // Same point
        assert_eq!(manhattan_distance((0, 5), (0, 10)), 5); // Same row
        assert_eq!(manhattan_distance((7, 3), (2, 3)), 5); // Same column
        assert_eq!(manhattan_distance((1000, 1000), (0, 0)), 2000); // Large coordinates
    }
    #[test]
    fn test_find_valid_placements_simple() {
        let anfield = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '@', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        let piece = vec![vec!['O', 'O'], vec!['.', 'O']];
        let valid = find_valid_placements(&anfield, &piece, 1);
        assert!(valid.contains(&(0, 0)));
        assert!(valid.contains(&(1, 0)));
        assert!(valid.contains(&(1, 1)));
        assert_eq!(valid.len(), 3);
    }
    #[test]
    fn test_find_valid_placements_when_2_overlap() {
        let anfield = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '@', '.', '@'],
            vec!['.', '.', '.', '.'],
        ];
        let piece = vec![vec!['O', 'O', 'O', 'O'], vec!['O', 'O', 'O', 'O']];
        let valid = find_valid_placements(&anfield, &piece, 1);
        assert!(valid.is_empty());
    }

    #[test]
    fn test_find_valid_placements_no_overlap_with_player() {
        let anfield = vec![
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ];
        let piece = vec![vec!['O']];
        let valid = find_valid_placements(&anfield, &piece, 1);
        assert!(valid.is_empty());
    }

    #[test]
    fn test_find_valid_placements_overlap_with_opponent() {
        let anfield = vec![
            vec!['.', '$', '.'],
            vec!['.', '@', '.'],
            vec!['.', '.', '.'],
        ];
        let piece = vec![vec!['O'], vec!['O']];
        let valid = find_valid_placements(&anfield, &piece, 1);
        assert!(!valid.contains(&(2, 1)));
        assert_eq!(valid.len(), 1);
    }
    #[test]
    fn test_find_opponent_cells() {
        let anfield = vec![
            vec!['.', '$', '.', '.', '.'],
            vec!['.', '.', 's', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        let opp_syms = vec!['$', 's'];
        let cells = find_opponent_cells(&anfield, &opp_syms);
        assert!(cells.contains(&(0, 1)));
        assert!(cells.contains(&(1, 2)));
        assert_eq!(cells.len(), 2);
    }
    #[test]
    fn test_find_opponent_cells_empty_board() {
        let anfield = vec![vec!['.', '.', '.'], vec!['.', '.', '.']];
        let opp_syms = vec!['$', 's'];
        let cells = find_opponent_cells(&anfield, &opp_syms);
        assert!(cells.is_empty());
    }
    #[test]
    fn test_find_opponent_cells_all_opponents() {
        let anfield = vec![vec!['$', 's'], vec!['$', 's']];
        let opp_syms = vec!['$', 's'];
        let cells = find_opponent_cells(&anfield, &opp_syms);
        assert_eq!(cells.len(), 4);
        assert!(cells.contains(&(0, 0)));
        assert!(cells.contains(&(0, 1)));
        assert!(cells.contains(&(1, 0)));
        assert!(cells.contains(&(1, 1)));
    }
}
