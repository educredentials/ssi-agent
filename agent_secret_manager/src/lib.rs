use agent_shared::config::{config, SecretManagerConfig};
use did_manager::SecretManager;

pub mod subject;

pub async fn secret_manager() -> SecretManager {
    let SecretManagerConfig {
        stronghold_path: snapshot_path,
        stronghold_password: password,
        issuer_eddsa_key_id,
        issuer_es256_key_id,
        issuer_did,
        issuer_fragment,
    } = config().secret_manager.clone();

    SecretManager::load(
        snapshot_path,
        password,
        issuer_eddsa_key_id,
        issuer_es256_key_id,
        None,
        issuer_did,
        issuer_fragment,
    )
    .await
    .unwrap()
}
