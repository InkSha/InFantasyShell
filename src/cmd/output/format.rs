use unicode_width::UnicodeWidthStr;

use crate::cmd::terminal::Terminal;

pub fn list_output<T: ToString>(items: Vec<T>) -> String {
    items
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn rows_output<T: ToString>(items: Vec<T>) -> String {
    if items.is_empty() {
        return String::new();
    }

    let (term_width, _) = Terminal::get_size();
    let mut max_len = 0;
    let mut strs: Vec<String> = items
        .into_iter()
        .map(|i| {
            let item = i.to_string();
            max_len = max_len.max(UnicodeWidthStr::width(item.as_str()));
            return item;
        })
        .collect();

    strs.sort_by(|a, b| {
        // default by ascii
        a.cmp(b)
    });

    let mut i = strs.len();
    while i * max_len + i - 1 > term_width.into() {
        i -= 1;
    }

    let mut lines: Vec<String> = vec![];
    let rows = strs.len() / i + (strs.len() % i != 0) as usize;

    for row in 0..rows {
        let mut line: Vec<String> = vec![];

        for col in 0..i {
            let idx = col * rows + row;

            if let Some(item) = strs.get(idx) {
                // push space to align
                let space = " ".repeat(max_len - UnicodeWidthStr::width(item.as_str()));
                line.push(format!("{}{}", item, space));
            }
        }
        lines.push(line.join(" "));
    }

    return lines.join("\n");
}
