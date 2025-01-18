use std::fs;
use std::path::Path;

#[cfg(target_os = "linux")]
use crate::completion::generate_completion_zsh;
use std::io::Cursor;

// Function to clean up test environment by removing the completion file
fn cleanup() {
    if let Ok(home_dir) = std::env::var("HOME") {
        let completion_file = format!("{}/.zfunc/_cargo-make", home_dir);
        println!("\n\n\n\n{}\n\n\n\n", completion_file);

        if Path::new(&completion_file).exists() {
            fs::remove_file(&completion_file).expect("Failed to clean up test file");
        }
    }
}

#[test]
#[ignore]
fn test_generate_completion_zsh_empty_test() {
    cleanup();
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn test_generate_completion_zsh_overwrite_prompt_yes() {
    cleanup(); // Clean up before the test

    let input = b"y\n"; // Simulate user input of 'y'
    let mut reader = Cursor::new(input);

    let result = generate_completion_zsh(Some(&mut reader));
    assert!(result.is_ok());
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn test_generate_completion_zsh_overwrite_prompt_no() {
    cleanup(); // Clean up before the test
    let input = b"n\n"; // Simulate user input of 'n'
    let mut reader = Cursor::new(input);

    let result = generate_completion_zsh(Some(&mut reader));
    assert!(result.is_ok());
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn test_generate_completion_zsh_creates_directory() {
    cleanup(); // Clean up before the test

    let input = b"y\n"; // Simulate user input of 'y'
    let mut reader = Cursor::new(input);

    let result = generate_completion_zsh(Some(&mut reader));
    assert!(result.is_ok(), "Should succeed in generating completions");

    // Check if the directory was created
    let home_dir = std::env::var("HOME").expect("Failed to get HOME");
    let zfunc_dir = format!("{}/.zfunc", home_dir);
    assert!(
        Path::new(&zfunc_dir).exists(),
        "The zfunc directory should exist"
    );
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn test_generate_completion_zsh_creates_file() {
    cleanup(); // Clean up before the test

    let input = b"y\n"; // Simulate user input of 'y'
    let mut reader = Cursor::new(input);

    let result = generate_completion_zsh(Some(&mut reader));
    assert!(result.is_ok(), "Should succeed in generating completions");

    // Check if the completion file was created
    let home_dir = std::env::var("HOME").expect("Failed to get HOME");
    let completion_file = format!("{}/.zfunc/_cargo-make", home_dir);
    assert!(
        Path::new(&completion_file).exists(),
        "The completion file should exist"
    );
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn test_generate_completion_zsh_overwrite_prompt() {
    cleanup(); // Clean up before the test

    // Create the directory and file first
    let input = b"y\n"; // Simulate user input of 'y'
    let mut reader = Cursor::new(input);

    generate_completion_zsh(Some(&mut reader)).expect("Should succeed in generating completions");

    // Simulate user input for overwrite.
    let input = b"y\n"; // Simulate user input of 'y' again
    let mut reader = Cursor::new(input);

    let result = generate_completion_zsh(Some(&mut reader));
    assert!(result.is_ok(), "Should handle overwrite prompt gracefully");
}
