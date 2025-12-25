use config::Config;

pub fn get_bind() -> anyhow::Result<String> {
    let config = get_config()?;
    let bind = config.get_string("server.bind")?;
    Ok(bind)
}

pub fn get_host() -> anyhow::Result<String> {
    let config = get_config()?;
    let bind = config.get_string("server.host")?;
    Ok(bind)
}

pub fn get_admin_hash() -> anyhow::Result<String> {
    let config = get_config()?;
    let hash = config.get_string("admin.hash")?;
    Ok(hash)
}

fn get_config() -> anyhow::Result<Config> {
    let config = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .add_source(config::Environment::with_prefix("MEETINIZER").separator("_"))
        .build()?;

    Ok(config)
}
