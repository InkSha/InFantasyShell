use unicode_width::UnicodeWidthStr;

use crate::cmd::terminal::Terminal;

const COL_GAP: &'static str = "  ";

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
    let gap = UnicodeWidthStr::width(COL_GAP);

    let mut i = 1;
    while i * (max_len + gap) + gap < term_width.into() {
        i += 1;
    }

    let mut lines: Vec<String> = vec![];
    // floor
    let part_len = strs.len() / (i + 1);

    println!("{} {} {} {}", i, part_len, strs.len(), term_width);

    for order in 0..=part_len {
        let mut line: Vec<&str> = vec![];
        for index in 0..=i {
            if let Some(l) = strs.get((order * part_len) + index) {
                line.push(l);
            }
        }
        lines.push(line.join(COL_GAP));
    }

    return lines.join("\n");
}
