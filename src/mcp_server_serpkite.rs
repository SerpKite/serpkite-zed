use schemars::JsonSchema;
use serde::Deserialize;

use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const MCP_REMOTE_PACKAGE: &str = "mcp-remote";
const MCP_REMOTE_VERSION: &str = "0.1.38";
const MCP_URL: &str = "https://api.serpkite.com/v1/mcp";

struct SerpKiteModelContextExtension;

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct SerpKiteContextServerSettings {
    /// Optional SerpKite API key (skt_live_…) from https://app.serpkite.com/keys.
    /// Leave empty to sign in with OAuth in the browser instead.
    #[serde(default)]
    serpkite_api_key: Option<String>,
}

impl zed::Extension for SerpKiteModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        if zed::npm_package_installed_version(MCP_REMOTE_PACKAGE)?.as_deref()
            != Some(MCP_REMOTE_VERSION)
        {
            zed::npm_install_package(MCP_REMOTE_PACKAGE, MCP_REMOTE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-serpkite", project)?;
        let settings: SerpKiteContextServerSettings = settings
            .settings
            .map(|v| serde_json::from_value(v).map_err(|e| e.to_string()))
            .transpose()?
            .unwrap_or_default();

        let mut args = vec![MCP_URL.to_string()];
        let mut env = Vec::new();
        if let Some(key) = settings.serpkite_api_key.filter(|k| !k.trim().is_empty()) {
            // mcp-remote expands ${VAR} in header values; keeping the key in
            // the environment avoids the space in "Bearer <key>" inside args.
            args.push("--header".to_string());
            args.push("Authorization:${SERPKITE_AUTH_HEADER}".to_string());
            env.push((
                "SERPKITE_AUTH_HEADER".to_string(),
                format!("Bearer {}", key.trim()),
            ));
        }

        let command = if cfg!(target_os = "windows") {
            "node_modules/.bin/mcp-remote.cmd".to_string()
        } else {
            let path = "node_modules/.bin/mcp-remote";
            zed::make_file_executable(path)?;
            path.to_string()
        };

        Ok(Command { command, args, env })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(SerpKiteContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(SerpKiteModelContextExtension);
