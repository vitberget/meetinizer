use mail_send::SmtpClientBuilder;
use mail_send::mail_builder::MessageBuilder;
use tracing::info;

use crate::config::{get_mail_from, get_mail_password, get_mail_port, get_mail_server, get_mail_user};

pub async fn mail_link(email: &str, login_url: &str) -> anyhow::Result<()> {
    let server = get_mail_server()?;
    let port = get_mail_port()?;
    let user = get_mail_user()?;
    let password = get_mail_password()?;
    let from = get_mail_from()?;

    let message = MessageBuilder::new()
        .from(from)
        .to(email)
        .subject("Hi!")
        .html_body(format!("<h1>Welcome to...</h1><a href=\"{login_url}\">{login_url}</a>"))
        .text_body("Hello world!");

    let tls = true;
    info!("Connecting to SMTP server {server}:{port} tls:{true}");
    let mut client = SmtpClientBuilder::new(server, port as u16)
        .implicit_tls(tls)
        .credentials((user, password))
        .connect()
        .await?;

    info!("Sending mail {email} {login_url}");
    client.send(message).await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Depends on Internet and valid accound"]
    async fn test_send_link() -> anyhow::Result<()> {
        mail_link("test@vitberget.se", "https://vitberget.se/whatever").await?; 
        Ok(())
    }
}
