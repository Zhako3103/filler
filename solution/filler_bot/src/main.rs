use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut player = 0;
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut height = 0;
    let mut width = 0;

    while let Some(result) = lines.next() {
        let line = match result {
            Ok(l) => l.trim().to_string(),
            Err(_) => break,
        };

        if line.is_empty() {
            continue;
        }

        if line.contains("exec") {
            if line.contains("p1") {
                player = 1;
            } else {
                player = 2;
            }
        } else if line.starts_with("Anfield") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                width = parts[1].trim_end_matches(':').parse().unwrap_or(0);
                height = parts[2].trim_end_matches(':').parse().unwrap_or(0);
            }

            board = Vec::with_capacity(height);
            for _ in 0..height {
                if let Some(Ok(row_line)) = lines.next() {
                    let chars: Vec<char> = row_line.chars().skip(4).collect();
                    board.push(chars);
                }
            }
        } else if line.starts_with("Piece") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let p_height: usize = parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);

            let mut piece = Vec::with_capacity(p_height);
            for _ in 0..p_height {
                if let Some(Ok(p_line)) = lines.next() {
                    piece.push(p_line.chars().collect::<Vec<char>>());
                }
            }

            if width == 0 || height == 0 || piece.is_empty() {
                println!("0 0");
                io::stdout().flush().unwrap();
                continue;
            }

            let move_coord = find_best_move(&board, height, width, &piece, player);
            println!("{} {}", move_coord.0, move_coord.1);
            io::stdout().flush().unwrap();
        }
    }
}

fn find_best_move(
    board: &[Vec<char>],
    height: usize,
    width: usize,
    piece: &[Vec<char>],
    player: i32,
) -> (isize, isize) {
    let (my_char, my_lower, opp_char, opp_lower) = if player == 1 {
        ('@', 'a', '$', 's')
    } else {
        ('$', 's', '@', 'a')
    };

    let p_height = piece.len() as isize;
    let p_width = if !piece.is_empty() { piece[0].len() as isize } else { 0 };

    if p_width == 0 || p_height == 0 {
        return (0, 0);
    }

    let mut valid_moves = Vec::new();

    for y in -p_height..height as isize {
        for x in -p_width..width as isize {
            if is_valid(
                board, height, width, piece, x, y, my_char, my_lower, opp_char,
                opp_lower,
            ) {
                valid_moves.push((x, y));
            }
        }
    }

    if valid_moves.is_empty() {
        return (0, 0);
    }

    let mut opp_coords = Vec::new();
    for ry in 0..height {
        if ry < board.len() {
            for rx in 0..width {
                if rx < board[ry].len() {
                    let c = board[ry][rx];
                    if c == opp_char || c == opp_lower {
                        opp_coords.push((rx as isize, ry as isize));
                    }
                }
            }
        }
    }

    if opp_coords.is_empty() {
        return valid_moves[0];
    }

    let mut best_move = valid_moves[0];
    let mut min_dist: isize = isize::MAX;

    for &(mx, my) in &valid_moves {
        let mut placement_min: isize = isize::MAX;
        for (py, row) in piece.iter().enumerate() {
            for (px, &cell) in row.iter().enumerate() {
                if cell != '.' {
                    let bx = mx + px as isize;
                    let by = my + py as isize;
                    for &(ox, oy) in &opp_coords {
                        let dist = (bx - ox).abs() + (by - oy).abs();
                        if dist < placement_min {
                            placement_min = dist;
                        }
                    }
                }
            }
        }
        if placement_min < min_dist {
            min_dist = placement_min;
            best_move = (mx, my);
        }
    }

    best_move
}

fn is_valid(
    board: &[Vec<char>],
    height: usize,
    width: usize,
    piece: &[Vec<char>],
    start_x: isize,
    start_y: isize,
    my_char: char,
    my_lower: char,
    opp_char: char,
    opp_lower: char,
) -> bool {
    let mut overlaps = 0;

    for (py, row) in piece.iter().enumerate() {
        for (px, &cell) in row.iter().enumerate() {
            if cell == '.' {
                continue;
            }
            let bx = start_x + px as isize;
            let by = start_y + py as isize;

            if bx < 0 || bx >= width as isize || by < 0 || by >= height as isize {
                return false;
            }

            let by_u = by as usize;
            let bx_u = bx as usize;

            if by_u >= board.len() || bx_u >= board[by_u].len() {
                return false;
            }

            let board_cell = board[by_u][bx_u];
            if board_cell == opp_char || board_cell == opp_lower {
                return false;
            }
            if board_cell == my_char || board_cell == my_lower {
                overlaps += 1;
            }
        }
    }

    overlaps == 1
}