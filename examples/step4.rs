use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    provider: String,
    service: String,
    users: Vec<User>,
}

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    role: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config_file = args.get(1).context("Config file must be provided")?;
    let config_string = std::fs::read_to_string(config_file)?;
    let config: Config = serde_json::from_str(&config_string)?;
    let Config {
        provider,
        service,
        users,
    } = config;
    println!("In {provider}, create {service} for:");
    for User { username, role } in users {
        println!("- {username} with role {role:?}");
    }
    Ok(())
}
