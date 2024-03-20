use std::env;
use std::{collections::HashMap, fs};

/// Config carries all the configuration for Azure Storage services.
#[derive(Clone, Default)]
#[cfg_attr(test, derive(Debug))]
pub struct Config {
    /// `account_name` will be loaded from
    ///
    /// - this field if it's `is_some`
    pub account_name: Option<String>,
    /// `account_key` will be loaded from
    ///
    /// - this field if it's `is_some`
    pub account_key: Option<String>,
    /// `sas_token` will be loaded from
    ///
    /// - this field if it's `is_some`
    pub sas_token: Option<String>,
    /// Specifies the object id associated with a user assigned managed service identity resource
    ///
    /// The values of client_id and msi_res_id are discarded
    ///
    /// This is part of use AAD(Azure Active Directory) authenticate on Azure VM
    pub object_id: Option<String>,
    /// Specifies the application id (client id) associated with a user assigned managed service identity resource
    ///
    /// The values of object_id and msi_res_id are discarded
    ///
    /// This is part of use AAD(Azure Active Directory) authenticate on Azure VM
    pub client_id: Option<String>,
    /// Specifies the ARM resource id of the user assigned managed service identity resource
    ///
    /// The values of object_id and client_id are discarded
    ///
    /// This is part of use AAD(Azure Active Directory) authenticate on Azure VM
    pub msi_res_id: Option<String>,
    /// Specifies the header that should be used to retrieve the access token.
    ///
    /// This header mitigates server-side request forgery (SSRF) attacks.
    ///
    /// This is part of use AAD(Azure Active Directory) authenticate on Azure VM
    pub msi_secret: Option<String>,
    /// Specifies the endpoint from which the identity should be retrieved.
    ///
    /// If not specified, the default endpoint of `http://169.254.169.254/metadata/identity/oauth2/token` will be used.
    ///
    /// This is part of use AAD(Azure Active Directory) authenticate on Azure VM
    pub endpoint: Option<String>,
    /// `federated_token` value will be loaded from:
    ///
    /// - this field if it's `is_some`
    /// - env value: [`AZURE_FEDERATED_TOKEN`]
    /// - profile config: `federated_toen_file`
    pub federated_token: Option<String>,
    /// `tenant_id` value will be loaded from:
    ///
    /// - this field if it's `is_some`
    /// - env value: [`AZURE_TENANT_ID`]
    /// - profile config: `tenant_id`
    pub tenant_id: Option<String>,
    /// `authority_host` value will be loaded from:
    ///
    /// - this field if it's `is_some`
    /// - env value: [`AZURE_AUTHORITY_HOST_ENV_KEY`]
    /// - profile config: `authority_host`
    pub authority_host: Option<String>,
}

pub const AZURE_FEDERATED_TOKEN: &str = "AZURE_FEDERATED_TOKEN";
pub const AZURE_FEDERATED_TOKEN_FILE: &str = "AZURE_FEDERATED_TOKEN_FILE";
pub const AZURE_TENANT_ID: &str = "AZURE_TENANT_ID_ENV_KEY";
pub const AZURE_AUTHORITY_HOST_ENV_KEY: &str = "AZURE_AUTHORITY_HOST_ENV_KEY";
const AZURE_PUBLIC_CLOUD: &str = "https://login.microsoftonline.com";

impl Config {
    /// Load config from env.
    pub fn from_env(mut self) -> Self {
        let envs = env::vars().collect::<HashMap<_, _>>();

        // federated_token can be loaded from both `AZURE_FEDERATED_TOKEN` and `AZURE_FEDERATED_TOKEN_FILE`.
        if let Some(v) = envs.get(AZURE_FEDERATED_TOKEN_FILE) {
            self.federated_token = Some(fs::read_to_string(v).unwrap_or_default());
        }

        if let Some(v) = envs.get(AZURE_FEDERATED_TOKEN) {
            self.federated_token = Some(v.to_string());
        }

        if let Some(v) = envs.get(AZURE_TENANT_ID) {
            self.tenant_id = Some(v.to_string());
        }

        if let Some(v) = envs.get(AZURE_AUTHORITY_HOST_ENV_KEY) {
            self.authority_host = Some(v.to_string());
        } else {
            self.authority_host = Some(AZURE_PUBLIC_CLOUD.to_string());
        }

        self
    }
}
