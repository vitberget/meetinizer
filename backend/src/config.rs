use config::Config;

pub fn get_bind() -> anyhow::Result<String> {
    let config = get_config()?;
    let bind = config.get_string("server.bind")?;
    Ok(bind)
}

fn get_config() -> anyhow::Result<Config> {
    let config = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .build()?;

    Ok(config)
}
