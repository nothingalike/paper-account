use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use log::{debug, info, warn, trace};

use crate::account::Account;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::types::AccountId;

/// Manages multiple paper trading accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountManager {
    /// Map of accounts by ID
    accounts: HashMap<String, Account>,
    /// Path to the storage file
    #[serde(skip)]
    storage_path: Option<PathBuf>,
}

impl AccountManager {
    /// Create a new account manager
    pub fn new() -> Self {
        // Check if there's a storage path in the global config
        let config = crate::config::get();
        let storage_path_debug = config.storage_path.clone();
        let storage_path = config.storage_path.map(PathBuf::from);
        
        if let Some(path) = &storage_path_debug {
            info!("AccountManager::new() - Using storage path from config: {}", path);
        } else {
            debug!("AccountManager::new() - No storage path in config, using None");
        }
        
        Self {
            accounts: HashMap::new(),
            storage_path,
        }
    }

    /// Set the storage path for account persistence
    pub fn with_storage<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path_buf = path.as_ref().to_path_buf();
        info!("AccountManager::with_storage() - Setting storage path to: {:?}", path_buf);
        self.storage_path = Some(path_buf);
        self
    }

    /// Get the default storage path in the user's app data directory
    pub fn get_default_storage_path() -> Result<PathBuf> {
        let app_data = dirs::data_local_dir()
            .ok_or_else(|| Error::Custom("Could not determine app data directory".into()))?;
        
        let paper_account_dir = app_data.join("paper-account");
        
        // Create the directory if it doesn't exist
        if !paper_account_dir.exists() {
            fs::create_dir_all(&paper_account_dir)?;
        }
        
        Ok(paper_account_dir.join("accounts.json"))
    }

    /// Create a new account
    pub fn create_account<S: Into<String>>(
        &mut self, 
        name: S, 
        base_currency: S, 
        initial_deposit: Decimal
    ) -> Result<AccountId> {
        let account = Account::new(name, base_currency, initial_deposit);
        let id = account.id;
        self.accounts.insert(id.0.to_string(), account);
        Ok(id)
    }

    /// Create a new account with custom configuration
    pub fn create_account_with_config<S: Into<String>>(
        &mut self, 
        name: S, 
        base_currency: S, 
        initial_deposit: Decimal,
        config: Config
    ) -> Result<AccountId> {
        let account = Account::new(name, base_currency, initial_deposit).with_config(config);
        let id = account.id;
        self.accounts.insert(id.0.to_string(), account);
        Ok(id)
    }

    /// Get an account by ID
    pub fn get_account(&self, id: &AccountId) -> Option<&Account> {
        self.accounts.get(&id.0.to_string())
    }

    /// Get a mutable reference to an account by ID
    pub fn get_account_mut(&mut self, id: &AccountId) -> Option<&mut Account> {
        self.accounts.get_mut(&id.0.to_string())
    }

    /// Get all accounts
    pub fn get_accounts(&self) -> &HashMap<String, Account> {
        &self.accounts
    }

    /// Remove an account by ID
    pub fn remove_account(&mut self, id: &AccountId) -> Option<Account> {
        self.accounts.remove(&id.0.to_string())
    }

    /// Get the number of accounts
    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    /// Save accounts to storage
    pub fn save(&self) -> Result<()> {
        let path = match &self.storage_path {
            Some(path) => {
                info!("AccountManager::save() - Using provided storage path: {:?}", path);
                path.clone()
            },
            None => {
                let default_path = Self::get_default_storage_path()?;
                warn!("AccountManager::save() - No storage path set, using default: {:?}", default_path);
                default_path
            },
        };
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                debug!("AccountManager::save() - Creating parent directory: {:?}", parent);
                fs::create_dir_all(parent)?;
            }
        }
        
        let serialized = serde_json::to_string_pretty(&self)?;
        trace!("AccountManager::save() - Account data serialized, writing to file");
        fs::write(&path, serialized)?;
        
        info!("AccountManager::save() - Successfully saved {} accounts to: {:?}", 
            self.account_count(), path);
        Ok(())
    }

    /// Load accounts from storage
    pub fn load() -> Result<Self> {
        // Check if there's a storage path in the global config
        let config = crate::config::get();
        
        if let Some(path) = &config.storage_path {
            info!("AccountManager::load() - Using path from config: {:?}", path);
            return Self::load_from_path(path);
        }
        
        // Fall back to default path if no path in config
        let default_path = Self::get_default_storage_path()?;
        info!("AccountManager::load() - No path in config, using default path: {:?}", default_path);
        Self::load_from_path(default_path)
    }

    /// Load accounts from a specific path
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        info!("AccountManager::load_from_path() - Loading from path: {:?}", path);
        
        if !path.exists() {
            warn!("AccountManager::load_from_path() - Path does not exist, creating new manager: {:?}", path);
            return Ok(Self::new().with_storage(path));
        }
        
        debug!("AccountManager::load_from_path() - Reading file: {:?}", path);
        let data = fs::read_to_string(path)?;
        let mut manager: Self = serde_json::from_str(&data)?;
        debug!("AccountManager::load_from_path() - Setting storage path to: {:?}", path);
        manager.storage_path = Some(path.to_path_buf());
        
        info!("AccountManager::load_from_path() - Successfully loaded {} accounts", manager.account_count());
        Ok(manager)
    }

    /// Transfer funds between accounts
    pub fn transfer(
        &mut self, 
        from_id: &AccountId, 
        to_id: &AccountId, 
        amount: Decimal
    ) -> Result<()> {
        // Validate accounts exist
        if !self.accounts.contains_key(&from_id.0.to_string()) {
            return Err(Error::Custom(format!("Source account not found: {}", from_id.0)));
        }
        
        if !self.accounts.contains_key(&to_id.0.to_string()) {
            return Err(Error::Custom(format!("Destination account not found: {}", to_id.0)));
        }
        
        // Check sufficient funds
        let from_balance = self.accounts.get(&from_id.0.to_string()).unwrap().cash_balance;
        
        if from_balance < amount {
            return Err(Error::InsufficientFunds {
                required: amount,
                available: from_balance,
            });
        }
        
        // Perform transfer
        self.accounts.get_mut(&from_id.0.to_string()).unwrap().cash_balance -= amount;
        self.accounts.get_mut(&to_id.0.to_string()).unwrap().cash_balance += amount;
        
        Ok(())
    }

    /// Get the current storage path
    pub fn get_storage_path(&self) -> Option<&PathBuf> {
        self.storage_path.as_ref()
    }
}
