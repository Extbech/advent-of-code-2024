use serde_json::Value;
use std::fs;

fn main() {
    let path = "target/criterion";
    let mut rows = Vec::new();

    // Header row
    rows.push(vec![
        "Day".to_string(),
        "Solved".to_string(),
        "Solution Part 1 (μs)".to_string(),
        "Solution Part 2 (μs)".to_string(),
    ]);

    for day in 1..=25 {
        let part1_path = format!("{}/AOC day {}/Solution one/base/estimates.json", path, day);
        let part2_path = format!("{}/AOC day {}/Solution two/base/estimates.json", path, day);

        let part1_time = colorize_benchmark(&read_benchmark(&part1_path));
        let part2_time = colorize_benchmark(&read_benchmark(&part2_path));

        let rust_icon = "<img src=\"https://www.rust-lang.org/logos/rust-logo-32x32.png\" alt=\"Rust\" width=\"20\" />";

        rows.push(vec![
            day.to_string(),
            rust_icon.to_string(),
            part1_time,
            part2_time,
        ]);
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
            separator[0] = separator[0].replacen("-", ":", 1);
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
        json["slope"]["point_estimate"]
            .as_f64()
            .map_or("N/A".to_string(), |v| format!("{:.2}", v / 1000.0))
    } else {
        "N/A".to_string()
    }
}

fn colorize_benchmark(value: &str) -> String {
    if value == "N/A" {
        return value.to_string();
    }

    let time: f64 = value.parse().unwrap_or(0.0);

    if time < 50.0 {
        let mut base = format!(r"$${{\color{{green}}");
        base.push_str(&format!("{:.2}}}$$", time));
        base
    } else if time < 100.0 {
        let mut base = format!(r"$${{\color{{orange}}");
        base.push_str(&format!("{:.2}}}$$", time));
        base
    } else {
        let mut base = format!(r"$${{\color{{red}}");
        base.push_str(&format!("{:.2}}}$$", time));
        base
    }
}
