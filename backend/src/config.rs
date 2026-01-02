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


pub fn get_admin_hash() -> anyhow::Result<String> {
    let hash = get_config()?.get_string("admin.hash")?;
    Ok(hash)
}

pub fn get_mail_server() -> anyhow::Result<String> {
    let server = get_config()?.get_string("mail.server")?;
    Ok(server)
}

pub fn get_mail_port() -> anyhow::Result<i64> {
    let port = get_config()?.get_int("mail.port")?;
    Ok(port)
}

pub fn get_mail_user() -> anyhow::Result<String> {
    let user = get_config()?.get_string("mail.user")?;
    Ok(user)
}

pub fn get_mail_password() -> anyhow::Result<String> {
    let password = get_config()?.get_string("mail.password")?;
    Ok(password)
}

pub fn get_mail_from() -> anyhow::Result<String> {
    let from = get_config()?.get_string("mail.from")?;
    Ok(from)
}

pub fn get_mail_seconds() -> anyhow::Result<i64> {
    let seconds = get_config()?.get_int("mail.seconds")?;
    Ok(seconds)
}

fn get_config() -> anyhow::Result<Config> {
    Ok(Config::builder()
        .add_source(config::File::with_name("settings.toml").required(false)) 
        .add_source(config::Environment::with_prefix("MEETINIZER").separator("_"))
        .build()?)
}
