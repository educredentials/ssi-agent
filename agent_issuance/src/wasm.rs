//! WASM bindings for agent_issuance
//!
//! This module provides WebAssembly bindings for the credential issuance agent,
//! enabling credential issuance functionality within web browsers.
//!
//! # Architecture
//!
//! The WASM implementation uses:
//! - In-memory event store (browser LocalStorage optional)
//! - Web Crypto API for cryptographic operations
//! - Browser fetch API for HTTP requests
//! - wasm-bindgen for JS interop
//!
//! # Limitations
//!
//! - No persistent storage by default (in-memory only)
//! - Simplified key management (no Stronghold)
//! - Single-threaded execution model
//! - No direct database access

use wasm_bindgen::prelude::*;
use web_sys::console;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Initialize the WASM module
///
/// This should be called once when the module is loaded.
/// It sets up panic hooks for better error messages.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    console::log_1(&"Agent Issuance WASM module initialized".into());
}

/// Configuration for the WASM issuance agent
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WasmIssuanceConfig {
    issuer_url: String,
    issuer_name: String,
    debug: bool,
}

#[wasm_bindgen]
impl WasmIssuanceConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(issuer_url: String, issuer_name: String) -> Self {
        Self {
            issuer_url,
            issuer_name,
            debug: false,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn issuer_url(&self) -> String {
        self.issuer_url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_issuer_url(&mut self, url: String) {
        self.issuer_url = url;
    }

    #[wasm_bindgen(getter)]
    pub fn issuer_name(&self) -> String {
        self.issuer_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_issuer_name(&mut self, name: String) {
        self.issuer_name = name;
    }
}

/// In-memory event store for WASM environment
///
/// This replaces the database-backed event store used in native environments.
/// Events are stored in memory and optionally persisted to LocalStorage.
struct WasmEventStore {
    events: Mutex<HashMap<String, Vec<String>>>,
}

impl WasmEventStore {
    fn new() -> Self {
        Self {
            events: Mutex::new(HashMap::new()),
        }
    }

    fn append(&self, aggregate_id: &str, event: String) -> Result<(), JsValue> {
        let mut events = self.events.lock().map_err(|e| {
            JsValue::from_str(&format!("Lock error: {}", e))
        })?;

        events.entry(aggregate_id.to_string())
            .or_insert_with(Vec::new)
            .push(event);

        Ok(())
    }

    fn get(&self, aggregate_id: &str) -> Result<Vec<String>, JsValue> {
        let events = self.events.lock().map_err(|e| {
            JsValue::from_str(&format!("Lock error: {}", e))
        })?;

        Ok(events.get(aggregate_id).cloned().unwrap_or_default())
    }
}

/// WASM-compatible issuance agent
///
/// This is the main entry point for credential issuance operations in the browser.
#[wasm_bindgen]
pub struct WasmIssuanceAgent {
    config: WasmIssuanceConfig,
    event_store: Arc<WasmEventStore>,
}

#[wasm_bindgen]
impl WasmIssuanceAgent {
    /// Create a new WASM issuance agent
    #[wasm_bindgen(constructor)]
    pub fn new(config: WasmIssuanceConfig) -> Result<WasmIssuanceAgent, JsValue> {
        if config.debug {
            console::log_1(&"Creating WASM Issuance Agent".into());
        }

        Ok(Self {
            config,
            event_store: Arc::new(WasmEventStore::new()),
        })
    }

    /// Get the agent configuration
    pub fn get_config(&self) -> WasmIssuanceConfig {
        self.config.clone()
    }

    /// Create a credential offer
    ///
    /// # Parameters
    /// - `credential_configuration_ids`: JSON array of credential configuration IDs
    /// - `grant_types`: JSON array of grant types (e.g., ["urn:ietf:params:oauth:grant-type:pre-authorized_code"])
    ///
    /// # Returns
    /// JSON string containing the credential offer
    pub async fn create_credential_offer(
        &self,
        credential_configuration_ids: JsValue,
        grant_types: JsValue,
    ) -> Result<String, JsValue> {
        let config_ids: Vec<String> = serde_wasm_bindgen::from_value(credential_configuration_ids)
            .map_err(|e| JsValue::from_str(&format!("Invalid credential_configuration_ids: {}", e)))?;

        let grants: Vec<String> = serde_wasm_bindgen::from_value(grant_types)
            .map_err(|e| JsValue::from_str(&format!("Invalid grant_types: {}", e)))?;

        if self.config.debug {
            console::log_1(&format!(
                "Creating credential offer for configs: {:?}, grants: {:?}",
                config_ids, grants
            ).into());
        }

        // Generate a unique offer ID
        let offer_id = generate_random_id();

        // Create offer data structure
        let offer = serde_json::json!({
            "offer_id": offer_id,
            "credential_issuer": self.config.issuer_url,
            "credential_configuration_ids": config_ids,
            "grants": grants,
            "created_at": js_sys::Date::now(),
        });

        // Store event
        let event = serde_json::to_string(&serde_json::json!({
            "type": "CredentialOfferCreated",
            "aggregate_id": offer_id,
            "data": offer,
        })).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;

        self.event_store.append(&offer_id, event)?;

        // Return the offer
        serde_json::to_string(&offer)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize offer: {}", e)))
    }

    /// Get credential offer by ID
    pub fn get_credential_offer(&self, offer_id: &str) -> Result<String, JsValue> {
        let events = self.event_store.get(offer_id)?;

        if events.is_empty() {
            return Err(JsValue::from_str("Offer not found"));
        }

        // Return the latest event's data
        events.last()
            .ok_or_else(|| JsValue::from_str("No events found"))
            .and_then(|event| {
                serde_json::from_str::<serde_json::Value>(event)
                    .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))
                    .and_then(|v| {
                        v.get("data")
                            .ok_or_else(|| JsValue::from_str("Missing data field"))
                            .and_then(|data| {
                                serde_json::to_string(data)
                                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
                            })
                    })
            })
    }

    /// Verify a credential request
    ///
    /// # Parameters
    /// - `credential_request`: JSON string containing the credential request
    ///
    /// # Returns
    /// JSON string containing verification result
    pub async fn verify_credential_request(
        &self,
        credential_request: &str,
    ) -> Result<String, JsValue> {
        if self.config.debug {
            console::log_1(&format!("Verifying credential request: {}", credential_request).into());
        }

        // Parse the credential request
        let request: serde_json::Value = serde_json::from_str(credential_request)
            .map_err(|e| JsValue::from_str(&format!("Invalid credential request: {}", e)))?;

        // Basic validation
        if !request.is_object() {
            return Err(JsValue::from_str("Credential request must be an object"));
        }

        // TODO: Implement full proof verification using Web Crypto API
        // For now, return a basic verification result
        let result = serde_json::json!({
            "verified": true,
            "subject_id": request.get("proof")
                .and_then(|p| p.get("iss"))
                .and_then(|i| i.as_str())
                .unwrap_or("unknown"),
            "verified_at": js_sys::Date::now(),
        });

        serde_json::to_string(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Issue a credential
    ///
    /// # Parameters
    /// - `credential_data`: JSON string containing credential subject data
    /// - `credential_configuration_id`: The credential configuration to use
    ///
    /// # Returns
    /// JSON string containing the signed credential (JWT format)
    pub async fn issue_credential(
        &self,
        credential_data: &str,
        credential_configuration_id: &str,
    ) -> Result<String, JsValue> {
        if self.config.debug {
            console::log_1(&format!(
                "Issuing credential: config={}, data={}",
                credential_configuration_id, credential_data
            ).into());
        }

        // Parse credential data
        let data: serde_json::Value = serde_json::from_str(credential_data)
            .map_err(|e| JsValue::from_str(&format!("Invalid credential data: {}", e)))?;

        // Generate credential ID
        let credential_id = generate_random_id();

        // Create unsigned credential
        let issuance_date = js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default();
        let credential = serde_json::json!({
            "@context": [
                "https://www.w3.org/2018/credentials/v1"
            ],
            "id": format!("urn:uuid:{}", credential_id),
            "type": ["VerifiableCredential"],
            "issuer": self.config.issuer_url,
            "issuanceDate": issuance_date,
            "credentialSubject": data,
        });

        // TODO: Sign the credential using Web Crypto API
        // For now, return unsigned credential wrapped in a response
        let response = serde_json::json!({
            "credential_id": credential_id,
            "credential": credential,
            "format": "jwt_vc_json",
            "issued_at": js_sys::Date::now(),
        });

        serde_json::to_string(&response)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all credential offers
    pub fn get_all_offers(&self) -> Result<String, JsValue> {
        // This is a simplified implementation
        // In a real implementation, you would iterate through all aggregate IDs
        let offers = serde_json::json!({
            "offers": [],
            "count": 0,
        });

        serde_json::to_string(&offers)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Clear all stored data
    pub fn clear_storage(&self) -> Result<(), JsValue> {
        let mut events = self.event_store.events.lock()
            .map_err(|e| JsValue::from_str(&format!("Lock error: {}", e)))?;

        events.clear();

        if self.config.debug {
            console::log_1(&"Storage cleared".into());
        }

        Ok(())
    }
}

/// Generate a random ID using browser crypto API
fn generate_random_id() -> String {
    use js_sys::Math;

    // Generate a simple random ID
    // In production, use Web Crypto API for better randomness
    let timestamp = js_sys::Date::now() as u64;
    let random = (Math::random() * 1_000_000.0) as u64;

    format!("{:x}{:x}", timestamp, random)
}

/// Utility function to log messages to browser console
#[wasm_bindgen]
pub fn log(message: &str) {
    console::log_1(&message.into());
}

/// Get the WASM module version
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_config_creation() {
        let config = WasmIssuanceConfig::new(
            "https://issuer.example.com".to_string(),
            "Test Issuer".to_string(),
        );

        assert_eq!(config.issuer_url(), "https://issuer.example.com");
        assert_eq!(config.issuer_name(), "Test Issuer");
    }

    #[wasm_bindgen_test]
    fn test_random_id_generation() {
        let id1 = generate_random_id();
        let id2 = generate_random_id();

        assert_ne!(id1, id2, "Generated IDs should be unique");
    }
}
