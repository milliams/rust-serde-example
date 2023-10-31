use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    provider: String,
    service: String,
    users: Vec<User>,
    #[serde(rename = "LegacyType", skip_serializing_if = "Option::is_none")]
    legacy_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    role: Role,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Role {
    Admin,
    User,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config_file = args.get(1).context("Config file must be provided")?;
    let config_string = std::fs::read_to_string(config_file)?;
    let config: Config = serde_json::from_str(&config_string)?;
    println!("{}", serde_yaml::to_string(&config)?);
    let Config {
        provider,
        service,
        users,
        legacy_type,
    } = config;
    println!("In {provider}, create {service} for:");
    for User { username, role } in users {
        println!("- {username} with role {role:?}");
    }
    if let Some(legacy) = legacy_type {
        println!("Also, legacy thing: {legacy}");
    }
    Ok(())
}
