use rust_decimal_macros::dec;
use std::path::PathBuf;

use na_paper_account::{
    AccountManager, Config, init_with_config, init_logger
};

fn main() -> na_paper_account::error::Result<()> {
    // Initialize the logger with debug level
    init_logger("debug");
    
    // Create a custom storage path - use an absolute path for clarity
    let current_dir = std::env::current_dir()?;
    let custom_storage_path = current_dir.join("custom_storage_dir").join("accounts.json");
    
    println!("\n=== STORAGE PATH TEST ===");
    println!("Custom storage path: {:?}", custom_storage_path);
    
    // Create a configuration with the custom storage path
    let config = Config {
        storage_path: Some(custom_storage_path.to_string_lossy().to_string()),
        ..Config::default()
    };
    
    println!("Config storage_path set to: {:?}", config.storage_path);
    
    // Initialize the library with the custom config
    init_with_config(config);
    
    // Create an account manager - should use the storage path from config
    let mut manager = AccountManager::new();
    
    // Create a test account
    let account_id = manager.create_account(
        "Test Account", 
        "USD", 
        dec!(10000)
    )?;
    
    println!("\nCreated account: {}", account_id.0);
    
    // Save the account - this should use the custom storage path
    println!("\nSaving account manager...");
    manager.save()?;
    
    // Get the default path for comparison
    let default_path = AccountManager::get_default_storage_path()?;
    println!("\nDefault storage path: {:?}", default_path);
    
    // Check if our custom storage file exists
    println!("\nChecking if custom storage file exists...");
    if custom_storage_path.exists() {
        println!("SUCCESS: Custom storage file was created at: {:?}", custom_storage_path);
    } else {
        println!("ERROR: Custom storage file was NOT created at: {:?}", custom_storage_path);
        
        // Check if the default storage file exists instead
        if default_path.exists() {
            println!("WARNING: Default storage file exists instead at: {:?}", default_path);
        }
    }
    
    println!("\n=== TEST COMPLETE ===");
    Ok(())
}
