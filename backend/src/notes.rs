use crate::models::{Entry, EntryKind};

/// Format a single Inbox line for a note or todo.
pub fn format_entry(text: &str, is_todo: bool, stamp: &str) -> String {
    let clean = clean_text(text);
    if is_todo {
        format!("- [ ] {stamp} — {clean}")
    } else {
        format!("- {stamp} — {clean}")
    }
}

/// Insert `entry_line` just beneath the `## Inbox` heading (so newest entries
/// sit at the top of the section), creating the section at the end of the file
/// if it is missing. Preserves a trailing newline when the input had one.
pub fn append_to_inbox(content: &str, entry_line: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    if let Some(idx) = lines.iter().position(|l| l.trim() == "## Inbox") {
        lines.insert(idx + 1, entry_line.to_string());
        let mut out = lines.join("\n");
        if content.ends_with('\n') {
            out.push('\n');
        }
        out
    } else {
        let mut out = content.trim_end().to_string();
        out.push_str("\n\n## Inbox\n");
        out.push_str(entry_line);
        out.push('\n');
        out
    }
}

/// Parse every bullet line in the file into a structured entry. Headings and
/// blank/free-text lines are ignored. Recognizes `- [ ]`/`- [x]` todos and
/// `- ` notes, with an optional leading `YYYY-MM-DD HH:MM` timestamp.
pub fn parse_notes(content: &str) -> Vec<Entry> {
    let mut out = Vec::new();
    for (i, raw) in content.lines().enumerate() {
        let line = raw.trim_start();
        let (kind, done, rest) = if let Some(r) = line.strip_prefix("- [ ] ") {
            (EntryKind::Todo, false, r)
        } else if let Some(r) = line
            .strip_prefix("- [x] ")
            .or_else(|| line.strip_prefix("- [X] "))
        {
            (EntryKind::Todo, true, r)
        } else if let Some(r) = line.strip_prefix("- ") {
            (EntryKind::Note, false, r)
        } else {
            continue;
        };

        let (timestamp, text) = split_timestamp(rest);
        out.push(Entry {
            kind,
            done,
            text: text.trim().to_string(),
            timestamp,
            line: i,
        });
    }
    out
}

/// Flip a todo line between `- [ ]` and `- [x]`. Errors if the line is not a todo.
pub fn toggle_todo_at(content: &str, line: usize) -> Result<String, String> {
    let (mut lines, trailing) = split_lines(content);
    let original = lines.get(line).ok_or("Line out of range")?.clone();
    let trimmed = original.trim_start();
    let indent = &original[..original.len() - trimmed.len()];

    let new = if let Some(r) = trimmed.strip_prefix("- [ ] ") {
        format!("{indent}- [x] {r}")
    } else if let Some(r) = trimmed
        .strip_prefix("- [x] ")
        .or_else(|| trimmed.strip_prefix("- [X] "))
    {
        format!("{indent}- [ ] {r}")
    } else {
        return Err("Not a todo line".into());
    };

    lines[line] = new;
    Ok(join_lines(lines, trailing))
}

/// Replace the text of an entry line, keeping its bullet, checkbox state, and
/// any leading timestamp intact.
pub fn update_text_at(content: &str, line: usize, new_text: &str) -> Result<String, String> {
    let (mut lines, trailing) = split_lines(content);
    let original = lines.get(line).ok_or("Line out of range")?.clone();
    let trimmed = original.trim_start();
    let indent = &original[..original.len() - trimmed.len()];
    let clean = clean_text(new_text);

    let (marker, after) = if let Some(r) = trimmed.strip_prefix("- [ ] ") {
        ("- [ ] ", r)
    } else if let Some(r) = trimmed
        .strip_prefix("- [x] ")
        .or_else(|| trimmed.strip_prefix("- [X] "))
    {
        ("- [x] ", r)
    } else if let Some(r) = trimmed.strip_prefix("- ") {
        ("- ", r)
    } else {
        return Err("Not an entry line".into());
    };

    let (timestamp, _) = split_timestamp(after);
    let new_line = match timestamp {
        Some(t) => format!("{indent}{marker}{t} — {clean}"),
        None => format!("{indent}{marker}{clean}"),
    };

    lines[line] = new_line;
    Ok(join_lines(lines, trailing))
}

/// Remove an entry line entirely.
pub fn delete_at(content: &str, line: usize) -> Result<String, String> {
    let (mut lines, trailing) = split_lines(content);
    if line >= lines.len() {
        return Err("Line out of range".into());
    }
    lines.remove(line);
    Ok(join_lines(lines, trailing))
}

fn clean_text(text: &str) -> String {
    text.trim().replace('\n', " ").replace('\r', " ")
}

fn split_lines(content: &str) -> (Vec<String>, bool) {
    (
        content.lines().map(|l| l.to_string()).collect(),
        content.ends_with('\n'),
    )
}

fn join_lines(lines: Vec<String>, trailing: bool) -> String {
    let mut out = lines.join("\n");
    if trailing {
        out.push('\n');
    }
    out
}

/// Split a leading `YYYY-MM-DD HH:MM — ` timestamp off the front of a line.
fn split_timestamp(s: &str) -> (Option<String>, &str) {
    if let Some(pos) = s.find(" — ") {
        let (head, tail) = s.split_at(pos);
        if is_timestamp(head) {
            return (Some(head.to_string()), &tail[" — ".len()..]);
        }
    }
    (None, s)
}

fn is_timestamp(s: &str) -> bool {
    let b = s.as_bytes();
    s.len() == 16
        && b[4] == b'-'
        && b[7] == b'-'
        && b[10] == b' '
        && b[13] == b':'
        && s.chars()
            .enumerate()
            .all(|(i, c)| matches!(i, 4 | 7 | 10 | 13) || c.is_ascii_digit())
}
