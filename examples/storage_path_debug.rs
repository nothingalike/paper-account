use rust_decimal_macros::dec;
use std::path::PathBuf;

use na_paper_account::{
    AccountManager, Config, init_with_config
};

/// A clean example to demonstrate and verify the storage path functionality
fn main() -> na_paper_account::error::Result<()> {
    // Disable logging to keep output clean
    std::env::set_var("RUST_LOG", "error");
    
    println!("\n===== STORAGE PATH VERIFICATION =====");
    
    // Step 1: Create a custom storage path
    let current_dir = std::env::current_dir()?;
    let custom_storage_path = current_dir.join("debug_storage").join("accounts.json");
    println!("Custom storage path: {:?}", custom_storage_path);
    
    // Step 2: Create a configuration with the custom storage path
    let config = Config {
        storage_path: Some(custom_storage_path.to_string_lossy().to_string()),
        ..Config::default()
    };
    
    // Step 3: Initialize the library with the custom config
    init_with_config(config.clone());
    println!("Initialized with config.storage_path = {:?}", config.storage_path);
    
    // Step 4: Create an account manager - should use the storage path from config
    let mut manager = AccountManager::new();
    println!("AccountManager created with storage_path = {:?}", manager.get_storage_path());
    
    // Step 5: Create a test account
    let account_id = manager.create_account("Debug Account", "USD", dec!(10000))?;
    println!("Created account: {}", account_id.0);
    
    // Step 6: Save the account - should use the custom storage path
    println!("\nSaving account manager...");
    manager.save()?;
    
    // Step 7: Verify the custom storage file exists
    if custom_storage_path.exists() {
        println!("SUCCESS: Custom storage file created at: {:?}", custom_storage_path);
        
        // Step 8: Test loading with AccountManager::load()
        println!("\nCreating a new manager with AccountManager::load()...");
        let loaded_manager = AccountManager::load()?;
        println!("Loaded manager has storage_path = {:?}", loaded_manager.get_storage_path());
        println!("Loaded manager has {} accounts", loaded_manager.account_count());
        
        // Step 9: Test loading from custom path
        println!("\nLoading from custom path...");
        let custom_loaded = AccountManager::load_from_path(&custom_storage_path)?;
        println!("Custom loaded manager has storage_path = {:?}", custom_loaded.get_storage_path());
        println!("Custom loaded manager has {} accounts", custom_loaded.account_count());
    } else {
        println!("ERROR: Custom storage file was NOT created");
        
        // Get the default path for comparison
        let default_path = AccountManager::get_default_storage_path()?;
        println!("Default storage path: {:?}", default_path);
        
        if default_path.exists() {
            println!("WARNING: Default storage file exists instead");
        }
    }
    
    println!("\n===== VERIFICATION COMPLETE =====");
    Ok(())
}
