use super::*;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    // Function to clean up test environment by removing the completion file
    fn cleanup() {
        let home_dir = std::env::var("HOME").expect("Failed to get HOME");
        let completion_file = format!("{}/.zfunc/_cargo-make", home_dir);
        if Path::new(&completion_file).exists() {
            fs::remove_file(&completion_file).expect("Failed to clean up test file");
        }
    }

    #[test]
    fn test_generate_completion_zsh_creates_directory() {
        cleanup(); // Clean up before the test

        let result = generate_completion_zsh();
        assert!(result.is_ok(), "Should succeed in generating completions");

        // Check if the directory was created
        let home_dir = std::env::var("HOME").expect("Failed to get HOME");
        let zfunc_dir = format!("{}/.zfunc", home_dir);
        assert!(Path::new(&zfunc_dir).exists(), "The zfunc directory should exist");
    }

    #[test]
    fn test_generate_completion_zsh_creates_file() {
        cleanup(); // Clean up before the test

        let result = generate_completion_zsh();
        assert!(result.is_ok(), "Should succeed in generating completions");

        // Check if the completion file was created
        let home_dir = std::env::var("HOME").expect("Failed to get HOME");
        let completion_file = format!("{}/.zfunc/_cargo-make", home_dir);
        assert!(Path::new(&completion_file).exists(), "The completion file should exist");
    }

    #[test]
    fn test_generate_completion_zsh_overwrite_prompt() {
        cleanup(); // Clean up before the test

        // Create the directory and file first
        generate_completion_zsh().expect("Should succeed in generating completions");

        // Simulate user input for overwrite.
        // You might want to refactor the `generate_completion_zsh` function to take an input parameter
        // for easier testing.
        
        // For now, let's just check that we can call it again
        let result = generate_completion_zsh(); // This will prompt in the real environment
        assert!(result.is_ok(), "Should handle overwrite prompt gracefully");
    }

    // Additional tests can be added here for other scenarios
}
