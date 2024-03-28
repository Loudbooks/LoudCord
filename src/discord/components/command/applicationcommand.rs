use std::collections::HashMap;

use serde::Serialize;

use crate::discord::components::command::applicationcommandsoption::ApplicationCommandOption;
use crate::discord::mapping::applicationcommandtype::ApplicationCommandType;
use crate::discord::mapping::applicationintegrationtype::ApplicationIntegrationType;
use crate::discord::mapping::applicationinteractioncontexttype::ApplicationInteractionContextType;

#[derive(Debug, Serialize)]
pub struct ApplicationCommand {
    r#type: Option<ApplicationCommandType>,
    name: String,
    name_localizations: Option<HashMap<String, String>>,
    description: String,
    description_localizations: Option<HashMap<String, String>>,
    options: Option<Vec<ApplicationCommandOption>>,
    default_member_permissions: Option<String>,
    dm_permission: Option<bool>,
    default_permission: Option<bool>,
    nsfw: Option<bool>,
    integration_types: Option<Vec<ApplicationIntegrationType>>,
    contexts: Option<Vec<ApplicationInteractionContextType>>,
}

pub struct ApplicationCommandBuilder {
    r#type: Option<ApplicationCommandType>,
    name: String,
    name_localizations: Option<HashMap<String, String>>,
    description: String,
    description_localizations: Option<HashMap<String, String>>,
    options: Option<Vec<ApplicationCommandOption>>,
    default_member_permissions: Option<String>,
    dm_permission: Option<bool>,
    default_permission: Option<bool>,
    nsfw: Option<bool>,
    integration_types: Option<Vec<ApplicationIntegrationType>>,
    contexts: Option<Vec<ApplicationInteractionContextType>>,
}

impl ApplicationCommandBuilder {
    pub fn new(name: &str, description: &str, application_integration_type: ApplicationCommandType, contexts: Vec<ApplicationInteractionContextType>) -> ApplicationCommandBuilder {
        ApplicationCommandBuilder {
            r#type: Some(application_integration_type),
            name: name.to_string(),
            name_localizations: None,
            description: description.to_string(),
            description_localizations: None,
            options: None,
            default_member_permissions: None,
            dm_permission: None,
            default_permission: None,
            nsfw: None,
            integration_types: None,
            contexts: Some(contexts)
        }
    }
    
    pub fn name_localizations(mut self, name_localizations: HashMap<String, String>) -> ApplicationCommandBuilder {
        self.name_localizations = Some(name_localizations);
        self
    }

    pub fn description_localizations(mut self, description_localizations: HashMap<String, String>) -> ApplicationCommandBuilder {
        self.description_localizations = Some(description_localizations);
        self
    }

    pub fn options(mut self, options: Vec<ApplicationCommandOption>) -> ApplicationCommandBuilder {
        self.options = Some(options);
        self
    }

    pub fn default_member_permissions(mut self, default_member_permissions: &str) -> ApplicationCommandBuilder {
        self.default_member_permissions = Some(default_member_permissions.to_string());
        self
    }

    pub fn dm_permission(mut self, dm_permission: bool) -> ApplicationCommandBuilder {
        self.dm_permission = Some(dm_permission);
        self
    }

    pub fn default_permission(mut self, default_permission: bool) -> ApplicationCommandBuilder {
        self.default_permission = Some(default_permission);
        self
    }

    pub fn nsfw(mut self, nsfw: bool) -> ApplicationCommandBuilder {
        self.nsfw = Some(nsfw);
        self
    }

    pub fn integration_types(mut self, integration_types: Vec<ApplicationIntegrationType>) -> ApplicationCommandBuilder {
        self.integration_types = Some(integration_types);
        self
    }
    
    pub fn build(self) -> ApplicationCommand {
        ApplicationCommand {
            r#type: self.r#type,
            name: self.name,
            name_localizations: self.name_localizations,
            description: self.description,
            description_localizations: self.description_localizations,
            options: self.options,
            default_member_permissions: self.default_member_permissions,
            dm_permission: self.dm_permission,
            default_permission: self.default_permission,
            nsfw: self.nsfw,
            integration_types: self.integration_types,
            contexts: self.contexts,
        }
    }
}