// this is not my best work (oops)

fn search(lookup: &[char], table: &[Vec<char>], row_idx: usize, col_idx: usize, direction: Option<(isize, isize)>) -> i32 {
    let mut queries = 0;
    if let Some((row, col)) = direction {
        let new_row = row_idx as isize + row;
        let new_col = col_idx as isize + col;
        if (new_row < table.len() as isize && new_col < table[0].len() as isize
            && new_row >= 0 && new_col >= 0) && table[new_row as usize][new_col as usize] == lookup[0] {
            if lookup.len() == 1 { queries += 1; }
            else { queries += search(&lookup[1..], table, new_row as usize, new_col as usize, Some((row, col))); }
        }
        return queries;
    }
    
    // down
    if row_idx > 0 && table[row_idx - 1][col_idx] == lookup[0] { 
        if lookup.len() == 1 { queries += 1; }
        else { queries += search(&lookup[1..], table, row_idx - 1, col_idx, Some((-1, 0))); }
    } 
    // up
    if row_idx < table.len() - 1 && table[row_idx + 1][col_idx] == lookup[0] {
        if lookup.len() == 1 { queries += 1; }
        else { queries += search(&lookup[1..], table, row_idx + 1, col_idx, Some((1, 0))); }
    } 
    // left
    if col_idx > 0 && table[row_idx][col_idx - 1] == lookup[0] {
        if lookup.len() == 1 { queries += 1; }
        else { queries += search(&lookup[1..], table, row_idx, col_idx - 1, Some((0, -1))); }
    } 
    // right
    if col_idx < table[row_idx].len() - 1 && table[row_idx][col_idx + 1] == lookup[0] {
        if lookup.len() == 1 { queries += 1; }
        else { queries += search(&lookup[1..], table, row_idx, col_idx + 1, Some((0, 1))); }
    }
    // up left
    if row_idx > 0 && col_idx > 0 && table[row_idx - 1][col_idx - 1] == lookup[0] {
        if lookup.len() == 1 { queries += 1; }
        else { queries += search(&lookup[1..], table, row_idx - 1, col_idx - 1, Some((-1, -1))); }
    } 
    // up right
    if row_idx > 0 && col_idx < table[row_idx].len() - 2 && table[row_idx - 1][col_idx + 1] == lookup[0] {
        if lookup.len() == 1 { queries += 1; }
        else { queries += search(&lookup[1..], table, row_idx - 1, col_idx + 1, Some((-1, 1))); }
    } 
    // down left
    if row_idx < table.len() - 2 && col_idx > 0 && table[row_idx + 1][col_idx - 1] == lookup[0] {
        if lookup.len() == 1 { queries += 1; }
        else { queries += search(&lookup[1..], table, row_idx + 1, col_idx - 1, Some((1, -1))); }
    } 
    // down right
    if row_idx < table.len() - 2 && col_idx < table[row_idx].len() - 2 && table[row_idx + 1][col_idx + 1] == lookup[0] {
        if lookup.len() == 1 { queries += 1; } 
        else { queries += search(&lookup[1..], table, row_idx + 1, col_idx + 1, Some((1, 1))); }
    }

    queries
}

pub fn day4(input: String) -> i32 {
    let table: Vec<Vec<char>> = input.split("\n").filter(|&line| !line.is_empty()).map(|line| line.chars().filter(|c| *c != ' ' && c.is_ascii_alphabetic()).collect()).collect();
    let mut matches = 0;
    table.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if *col == 'X' {
                let ch = search(&['M', 'A', 'S'], &table, row_idx, col_idx, None);
                matches += ch;
            }
        });
    });
    matches
}