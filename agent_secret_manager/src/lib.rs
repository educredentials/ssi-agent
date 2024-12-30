use agent_shared::config::{config, SecretManagerConfig};
use did_manager::SecretManager;

pub mod subject;

// TODO: find better solution for this
pub async fn secret_manager() -> SecretManager {
    let SecretManagerConfig {
        generate_stronghold,
        stronghold_path: snapshot_path,
        stronghold_password: password,
        issuer_eddsa_key_id,
        issuer_es256_key_id,
        issuer_did,
        issuer_fragment,
    } = config().secret_manager.clone();

    // TODO: move this to a separate binary entirely, not part of the secret manager getter!
    if generate_stronghold {
        SecretManager::generate(snapshot_path, password).await.unwrap()
    } else {
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
}
