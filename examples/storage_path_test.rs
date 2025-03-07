use rust_decimal_macros::dec;
use std::path::PathBuf;

use na_paper_account::{
    AccountManager, Config, init_with_config, init_logger
};

/// Example demonstrating storage path configuration with logging enabled
fn main() -> na_paper_account::error::Result<()> {
    // Initialize the logger with debug level
    init_logger("debug");

    println!("\n===== STORAGE PATH TEST WITH LOGGING =====");
    
    // Step 1: Create a custom storage path
    let custom_storage_path = PathBuf::from("./custom_storage_dir/accounts.json");
    println!("Custom storage path: {:?}", custom_storage_path);

    // Step 2: Create a configuration with the custom storage path
    let config = Config {
        storage_path: Some(custom_storage_path.to_string_lossy().to_string()),
        ..Config::default()
    };
    
    // Step 3: Initialize the library with the custom config
    init_with_config(config);
    
    // Step 4: Create an account manager - should use the storage path from config
    let mut manager = AccountManager::new();
    
    // Step 5: Create a test account
    let account_id = manager.create_account(
        "Test Account", 
        "USD", 
        dec!(10000)
    )?;
    
    println!("Created account: {}", account_id.0);
    
    // Step 6: Save the account - this should use the custom storage path
    manager.save()?;
    
    // Step 7: Load the account from the custom path to verify
    let loaded_manager = AccountManager::load()?;
    
    println!("Loaded {} accounts", loaded_manager.account_count());
    
    // Step 8: Verify the storage paths
    println!("\nVerifying storage paths:");
    println!("Default storage path: {:?}", AccountManager::get_default_storage_path()?);
    println!("Custom storage path: {:?}", custom_storage_path);
    println!("Manager storage path: {:?}", manager.get_storage_path());
    println!("Loaded manager storage path: {:?}", loaded_manager.get_storage_path());
    
    println!("\n===== TEST COMPLETED SUCCESSFULLY =====");
    Ok(())
}
