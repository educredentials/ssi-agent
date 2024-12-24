use agent_shared::config::{config, get_credential_confiugurations};
use agent_shared::url_utils::UrlAppendHelpers;
use oid4vci::credential_issuer::authorization_server_metadata::AuthorizationServerMetadata;

use crate::server_config::command::ServerConfigCommand;

/// Returns the startup commands for the application.
pub fn startup_commands(host: url::Url) -> Vec<ServerConfigCommand> {
    vec![load_server_metadata(host), create_credentials_supported()]
}

pub fn load_server_metadata(base_url: url::Url) -> ServerConfigCommand {
    let credential_issuer_metadata = config().credential_issuer_metadata.clone();

    ServerConfigCommand::InitializeServerMetadata {
        authorization_server_metadata: Box::new(AuthorizationServerMetadata {
            issuer: base_url.clone(),
            token_endpoint: Some(base_url.append_path_segment("auth/token")),
            ..Default::default()
        }),
        credential_issuer_metadata,
    }
}

pub fn create_credentials_supported() -> ServerConfigCommand {
    let credential_configurations = get_credential_confiugurations();
    let credential_configuration = credential_configurations
        .first()
        .expect("No credential configurations found")
        .clone();

    ServerConfigCommand::AddCredentialConfiguration {
        credential_configuration,
    }
}
