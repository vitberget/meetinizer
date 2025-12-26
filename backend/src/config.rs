use config::Config;

pub fn get_bind() -> anyhow::Result<String> {
    let bind = get_config()?.get_string("server.bind")?;
    Ok(bind)
}

pub fn get_host() -> anyhow::Result<String> {
    let host = get_config()?.get_string("server.host")?;
    Ok(host)
}

pub fn get_jwt_secret() -> anyhow::Result<String> {
    let secret = get_config()?.get_string("jwt.secret")?;
    Ok(secret)
}

pub fn get_jwt_valid_seconds() -> anyhow::Result<i64> {
    let seconds = get_config()?.get_int("jwt.valid_seconds")?;
    Ok(seconds)
}

pub fn get_admin_hash() -> anyhow::Result<String> {
    let hash = get_config()?.get_string("admin.hash")?;
    Ok(hash)
}

fn get_config() -> anyhow::Result<Config> {
    Ok(Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .add_source(config::Environment::with_prefix("MEETINIZER").separator("_"))
        .build()?)
}
