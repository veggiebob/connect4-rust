use std::cmp::max;
use std::fmt::Display;

/// pad a string on the left with spaces
pub fn pad_left(s: &String, length: usize) -> String {
    if s.len() < length {
        let spaces = " ".repeat(length - s.len());
        spaces + &s
    } else {
        s.clone()
    }
}

/// convert a matrix to a string
pub fn matrix_to_string<T: Display>(
    array: &Vec<Vec<T>>,
    headers: Option<&Vec<String>>,
    display_column_numbers: bool,
) -> String {
    let mut s = "".to_string();
    let ss = array
        .iter()
        .map(|row| row.iter().map(|t| format!("{}", t)).collect())
        .collect::<Vec<Vec<_>>>();
    let cols = ss[0].len();
    let mut max_col_width = (0..cols)
        .into_iter()
        .map(
            |col| max(ss.iter().map(|row| row[col].len()).max().unwrap_or(0) + 1,
            col.to_string().len() + 1)
        )
        .collect::<Vec<_>>();
    if let Some(headers) = headers {
        if headers.len() != cols {
            panic!("headers.len() != cols");
        }
        // update max col width
        for col in 0..cols {
            max_col_width[col] = max_col_width[col].max(headers[col].len() + 1);
        }
        // print the headers of each column aligned
        s += &format!(
            "{}\n",
            headers
                .iter()
                .zip(0..cols)
                .map(|(s, c)| pad_left(s, max_col_width[c]))
                .reduce(|x, y| x + &y)
                .unwrap_or("".to_string())
        );
    }

    let total_width: usize = max_col_width.iter().sum();

    let line = "-".repeat(total_width + 1) + "\n";
    s += &line;
    for row in ss.iter() {
        s += &format!(
            "{}\n",
            row.iter()
                .zip(0..cols)
                .map(|(s, c)| pad_left(s, max_col_width[c]))
                .reduce(|x, y| x + &y)
                .unwrap_or("".to_string())
        );
    }
    s += &line;
    if display_column_numbers {
        s += &format!(
            "{}\n",
            (0..cols)
                .into_iter()
                .map(|c| pad_left(&format!("{}", c), max_col_width[c]))
                .reduce(|x, y| x + &y)
                .unwrap_or("".to_string())
        );
    }
    return s;
}



