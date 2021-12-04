pub mod util {
    pub fn transpose_strs(lines: Vec<String>) -> Vec<String> {
        let mut cols: Vec<String> = Vec::new();

        if lines.len() > 0 {
            let len = lines[0].len();

            for _ in 0..len {
                cols.push(String::new())
            }

            for l in lines {
                for i in 0..len {
                    let col = &mut cols[i];
                    col.push(l.chars().nth(i).unwrap());
                }
            }
        }

        cols
    }
}