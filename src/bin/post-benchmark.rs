use serde_json::Value;
use std::fs;

fn main() {
    let path = "target/criterion";
    let mut rows = Vec::new();

    // Header row
    rows.push(vec![
        "Day".to_string(),
        "Solved".to_string(),
        "Part 1 runtime".to_string(),
        "Part 2 runtime".to_string(),
    ]);

    for day in 1..=25 {
        let part1_path = format!("{}/AOC day {}/Solution one/base/estimates.json", path, day);
        let part2_path = format!("{}/AOC day {}/Solution two/base/estimates.json", path, day);

        let part1_time = colorize_benchmark(&read_benchmark(&part1_path));
        let part2_time = colorize_benchmark(&read_benchmark(&part2_path));

        let solved = get_solved(&part1_time, &part2_time);

        rows.push(vec![day.to_string(), solved, part1_time, part2_time]);
    }
    // Determine column widths
    let col_widths: Vec<usize> = (0..rows[0].len())
        .map(|col| rows.iter().map(|row| row[col].len()).max().unwrap_or(0))
        .collect();

    // Build table
    let mut table = String::new();

    for (i, row) in rows.iter().enumerate() {
        let formatted_row: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(col, cell)| format!("{:width$}", cell, width = col_widths[col]))
            .collect();

        table.push_str(&formatted_row.join("  |  "));
        table.push('\n');

        if i == 0 {
            // Add separator after the header
            let mut separator: Vec<String> = col_widths.iter().map(|&w| "-".repeat(w)).collect();
            separator[0] = separator[0].replacen('-', ":", 1);
            let len = separator.last().unwrap().len();
            separator.last_mut().unwrap().insert(len, ':');
            table.push_str(&separator.join("-:|:-"));
            table.push('\n');
        }
    }

    // Write to README.md
    let readme_path = "README.md";
    let content = fs::read_to_string(readme_path).expect("Could not read README.md");
    let updated_content = content + "\n\n### Benchmark Results\n\n" + &table;
    fs::write(readme_path, updated_content).expect("Could not write README.md");
}

fn read_benchmark(path: &str) -> String {
    if let Ok(data) = fs::read_to_string(path) {
        let json: Value = serde_json::from_str(&data).unwrap();
        json["mean"]["point_estimate"]
            .as_f64()
            .map_or("N/A".to_string(), format_val)
    } else {
        "N/A".to_string()
    }
}

fn colorize_benchmark(value: &str) -> String {
    if value == "N/A" {
        return value.to_string();
    }

    if value.contains("ns") {
        let mut base = r"$${\color{purple}".to_string();
        base.push_str(&format!("{}}}$$", value));
        base
    } else if value.contains("μs") {
        let mut base = r"$${\color{green}".to_string();
        base.push_str(&format!("{}}}$$", value));
        base
    } else if value.contains("ms") {
        let mut base = r"$${\color{yellow}".to_string();
        base.push_str(&format!("{}}}$$", value));
        base
    } else {
        let mut base = r"$${\color{red}".to_string();
        base.push_str(&format!("{}}}$$", value));
        base
    }
}

fn get_solved(value_1: &str, value_2: &str) -> String {
    if value_1 == "N/A" && value_2 == "N/A" {
        ":x:".to_string()
    } else {
        "<img src=\"https://www.rust-lang.org/logos/rust-logo-32x32.png\" alt=\"Rust\" width=\"20\" />".to_string()
    }
}

fn format_val(v: f64) -> String {
    if (v / 1000.0) < 1.0 {
        format!("{:.2} ns", v)
    } else if (v / 1000.0) < 1000.0 {
        format!("{:.2} μs", v / 1000.0)
    } else if (v / 1000.0) < (1000.0 * 1000.0) {
        format!("{:.2} ms", v / (1000.0 * 1000.0))
    } else {
        format!("{:.2} s", v / (1000.0 * 1000.0 * 1000.0))
    }
}
