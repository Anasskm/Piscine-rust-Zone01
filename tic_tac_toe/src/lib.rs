pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    table.iter().any(|value| value.iter().all(|&r| r == player))
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let n = table.len();
    (0..n).any(|col| (0..n).all(|row| table[row][col] == player))
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let n = table.len();
    if (0..n).all(|i| table[i][i] == player) {
        return true;
    }

    // Check anti-diagonal
    if (0..n).all(|i| table[i][n - i - 1] == player) {
        return true;
    }
    false
}

pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if vertical("X", &table) || horizontal("X", &table) || diagonals("X", &table) {
        return "player X won".to_string();
    } else if vertical("O", &table) || horizontal("O", &table) || diagonals("O", &table) {
        return "player O won".to_string();
    }
    return "tie".to_string();
}
