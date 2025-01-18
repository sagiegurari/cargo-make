use log::{error, info};
use std::io::BufRead;
use std::path::Path;
use std::{fs, io};

/// # Completions Module
///
/// This module handles the generation of shell completion scripts for the `cargo-make` tool.
///
/// ## Functionality
/// - `generate_completion_zsh`: Generates a Zsh completion script, creates the necessary directory,
///   and prompts for overwriting existing files.
///
/// ## Improvements to Consider
/// 1. **Modularity**: Separate the completion logic into different modules for different shells
///    (e.g., Zsh, Bash, Fish) to improve code organization.
/// 2. **Cross-Platform Support**: Abstract the completion generation into a trait or interface
///    to facilitate adding support for other shell types.
/// 3. **Enhanced Error Handling**: Provide more informative error messages for file operations.
/// 4. **User Input Handling**: Ensure user input is trimmed and handled correctly.
/// 5. **Testing**: Implement unit tests to verify the correct behavior of completion generation functions.

#[cfg(test)]
#[path = "completion_test.rs"]
mod completion_test;

pub fn generate_completions(shell: &str) -> Result<(), Box<dyn std::error::Error>> {
    match shell {
        "zsh" => {
            generate_completion_zsh(None)?; // Use the `?` operator to propagate errors
            Ok(()) // Return Ok if no error occurred
        }
        _ => {
            // Return an error for unsupported shell
            Err(Box::from(format!(
                "Unsupported shell for completion: {}",
                shell
            )))
        }
    }
}

// Modify the function to accept an optional input stream
fn generate_completion_zsh(
    input: Option<&mut dyn io::Read>,
) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let zfunc_dir = format!("{}/.zfunc", home_dir);
    let completion_file = format!("{}/_cargo-make", zfunc_dir);

    if !Path::new(&zfunc_dir).exists() {
        if let Err(e) = fs::create_dir_all(&zfunc_dir) {
            error!("Failed to create directory {}: {}", zfunc_dir, e);
            return Err(Box::new(e));
        }
        info!("Created directory: {}", zfunc_dir);
    }

    if Path::new(&completion_file).exists() {
        let mut input_str = String::new();
        let reader: Box<dyn io::Read> = match input {
            Some(input) => Box::new(input),
            None => Box::new(io::stdin()),
        };

        // Create a BufReader to read from the provided input or stdin
        let mut buf_reader = io::BufReader::new(reader);
        println!(
            "File {} already exists. Overwrite? (y/n): ",
            completion_file
        );
        buf_reader.read_line(&mut input_str)?;

        if input_str.trim().to_lowercase() != "y" {
            println!("Aborted overwriting the file.");
            return Ok(());
        }
    }

    let completion_script = r#"
#compdef 'cargo make' 'cargo-make make' makers

_cargo_make() {
    local tasks
    local makefile="Makefile.toml"

    if [[ ! -f $makefile ]]; then
        return 1
    fi

    tasks=($(awk -F'[\\[\\.\\]]' '/^\[tasks/ {print $3}' "$makefile"))

    if [[ ${#tasks[@]} -eq 0 ]]; then
        return 1
    fi

    _describe -t tasks 'cargo-make tasks' tasks
}

_cargo_make "$@"
"#;

    fs::write(&completion_file, completion_script)?;
    println!("\nWrote tasks completion script to: {}", completion_file);

    println!("To enable Zsh completion, add the following lines to your ~/.zshrc:\n");
    println!("    fpath=(~/.zfunc $fpath)");
    println!("    autoload -Uz compinit && compinit");
    println!("\nThen, restart your terminal or run 'source ~/.zshrc'.");

    Ok(())
}
