use std::fmt::Display;

pub fn pad_left(s: &String, length: usize) -> String {
    if s.len() < length {
        let spaces = " ".repeat(length - s.len());
        spaces + &s
    } else {
        s.clone()
    }
}

pub fn matrix_to_string<T: Display>(array: &Vec<Vec<T>>, headers: Option<&Vec<String>>) -> String {
    let mut s = "".to_string();
    let ss = array.iter()
        .map(|row| row.iter()
            .map(|t| format!("{}", t)).collect()).collect::<Vec<Vec<_>>>();
    let cols = ss[0].len();
    let max_col_width = (0..cols).into_iter()
        .map(|col| ss.iter().map(|row|
            row[col].len()).max().unwrap_or(0) + 1)
        .collect::<Vec<_>>();
    let total_width: usize = max_col_width.iter().sum();
    if let Some(_) = headers {
        panic!("can't support headers right now");
    }
    let line = "-".repeat(total_width + 1) + "\n";
    s += &line;
    for row in ss.iter() {
        s += &format!("{}\n", row.iter().zip(0..cols)
            .map(|(s, c)| pad_left(s, max_col_width[c]))
            .reduce(|x, y| x + &y).unwrap_or("".to_string())
        );
    }
    s += &line;
    return s;
}
