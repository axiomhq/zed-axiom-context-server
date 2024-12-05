use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const SERVER_PATH: &str = "axiom-mcp";

struct AxiomMcpContextExtension;

#[derive(Debug, Deserialize)]
struct AxiomContextServerSettings {
    config_file: Option<String>,
    token: String,
    url: Option<String>,
    query_rate: Option<f64>,
    query_burst: Option<i32>,
    datasets_rate: Option<f64>,
    datasets_burst: Option<i32>,
}

impl zed::Extension for AxiomMcpContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("axiom-mcp", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing Axiom settings".into());
        };
        let settings: AxiomContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        // Build command arguments
        let mut args = vec![];

        args.push("-token".to_string());
        args.push(settings.token.clone());

        if let Some(url) = settings.url {
            args.push("-url".to_string());
            args.push(url);
        }

        if let Some(rate) = settings.query_rate {
            args.push("-query-rate".to_string());
            args.push(rate.to_string());
        }

        if let Some(burst) = settings.query_burst {
            args.push("-query-burst".to_string());
            args.push(burst.to_string());
        }

        if let Some(rate) = settings.datasets_rate {
            args.push("-datasets-rate".to_string());
            args.push(rate.to_string());
        }

        if let Some(burst) = settings.datasets_burst {
            args.push("-datasets-burst".to_string());
            args.push(burst.to_string());
        }

        if let Some(config_file) = settings.config_file {
            args.push("-config".to_string());
            args.push(config_file.to_string());
        }

        Ok(Command {
            command: env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string(),
            args,
            env: vec![],
        })
    }
}

zed::register_extension!(AxiomMcpContextExtension);
