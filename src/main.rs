use ansi_term::Colour;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

fn should_color(file_extension: &str, is_dir: bool) -> Option<Colour> {
    if is_dir {
        return Some(Colour::Blue); // Color for directories
    }
    
    match file_extension {
        "rs" => Some(Colour::Green),       // Rust
        "py" => Some(Colour::Yellow),       // Python
        "js" => Some(Colour::Cyan),         // JavaScript
        "jsx" => Some(Colour::Cyan),        // JSX
        "ts" => Some(Colour::Cyan),         // TypeScript
        "tsx" => Some(Colour::Cyan),        // TSX
        "java" => Some(Colour::Blue),       // Java
        "c" => Some(Colour::Red),           // C
        "cpp" => Some(Colour::Red),         // C++
        "cs" => Some(Colour::Purple),       // C#
        "go" => Some(Colour::Green),        // Go
        "php" => Some(Colour::Purple),      // PHP
        "html" => Some(Colour::Purple),     // HTML
        "css" => Some(Colour::Blue),        // CSS
        "json" => None,                     // JSON (no color)
        "lock" => None,                     // Lock files (no color)
        "toml" => None,                     // TOML files (no color)
        "yaml" => None,                     // YAML (no color)
        "md" => Some(Colour::Purple),       // Markdown
        "sh" => Some(Colour::Yellow),       // Shell script
        "bash" => Some(Colour::Yellow),     // Bash
        "rb" => Some(Colour::Red),          // Ruby
        "swift" => Some(Colour::Green),     // Swift
        "kt" => Some(Colour::Cyan),         // Kotlin
        "scala" => Some(Colour::Yellow),    // Scala
        "r" => Some(Colour::Green),         // R
        "sql" => Some(Colour::Purple),      // SQL
        "txt" => None,                      // Text files (no color)
        "log" => None,                      // Log files (no color)
        _ => None,                          // Unsupported file types (no color)
    }
}

fn print_tree(path: PathBuf, prefix: &str) {
    let entries = WalkDir::new(&path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok);

    for entry in entries {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_extension = entry.path().extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let is_dir = entry.path().is_dir(); // Check if the entry is a directory

        // Check if coloring is allowed and get the color if applicable
        match should_color(file_extension, is_dir) {
            Some(color) => {
                println!("{}{}", prefix, color.paint(file_name)); // Apply color
            }
            None => {
                println!("{}{}", prefix, file_name); // No color for unsupported file types
            }
        }

        // Recursively print the tree if the entry is a directory
        if is_dir {
            print_tree(entry.path().to_path_buf(), &format!("{}  ", prefix));
        }
    }
}

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    
    // Corrected println! macro
    println!("{}", current_dir.display());

    // Start printing the tree from the current directory
    print_tree(current_dir, "");
}
