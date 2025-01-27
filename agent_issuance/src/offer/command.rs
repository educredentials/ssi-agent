use oid4vci::{
    credential_issuer::{
        authorization_server_metadata::AuthorizationServerMetadata,
        credential_issuer_metadata::CredentialIssuerMetadata,
    },
    credential_request::CredentialRequest,
    token_request::TokenRequest,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OfferCommand {
    CreateCredentialOffer {
        offer_id: String,
    },
    AddCredentials {
        offer_id: String,
        credential_ids: Vec<String>,
    },

    // OpenID4VCI Pre-Authorized Code Flow
    // TODO: add option for credential_offer_uri (by reference)
    CreateFormUrlEncodedCredentialOffer {
        offer_id: String,
        credential_issuer_metadata: CredentialIssuerMetadata,
    },
    CreateTokenResponse {
        offer_id: String,
        token_request: TokenRequest,
    },
    VerifyCredentialRequest {
        offer_id: String,
        credential_issuer_metadata: CredentialIssuerMetadata,
        authorization_server_metadata: Box<AuthorizationServerMetadata>,
        credential_request: CredentialRequest,
    },
    CreateCredentialResponse {
        offer_id: String,
        signed_credentials: Vec<serde_json::Value>,
    },
}
