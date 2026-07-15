use anyhow::bail;
use mail_send::{Credentials, SmtpClientBuilder};
use mail_send::mail_builder::MessageBuilder;
use tracing::info;

use crate::config::{get_mail_from, get_mail_password, get_mail_port, get_mail_server, get_mail_user};

pub async fn mail_link(email: &str, meeting: &str, login_url: &str) -> anyhow::Result<()> {
    let server = get_mail_server()?;
    let port = get_mail_port()?;
    // TODO tls configurable
    let tls = true;
    let user = get_mail_user()?;
    let password = get_mail_password()?;
    let from = get_mail_from()?;


    info!("Connecting to SMTP server {server}:{port} tls:{tls}");

    let builder = match SmtpClientBuilder::new(&server, port as u16) {
        Ok(builder) => builder,
        Err(err) => { bail!("Failed creating SMTP {server}:{port} {err}"); }
    };

    let mut client = builder 
        .implicit_tls(tls)
        // TODO add support for other then Plain
        .credentials(Credentials::Plain { username: &user, secret: &password })
        .connect()
        .await?;

    let message = MessageBuilder::new()
        .from(from)
        .to(email)
        .subject(format!("Login to {meeting}"))
        .html_body(format!("<h1>Login to {meeting}</h1>\
                            <p>Please use this link <a href=\"{login_url}\">{login_url}</a> to log in and vote.\
                            <p>Best regards / the Meetinizer Robot"))
        .text_body("Login to {meeting}\n\n\
                    Please use this link {login_url} to log in and vote.\n\n\
                    Best regards / the Meetinizer Robot");

    info!("Sending mail {email} {login_url}");
    client.send(message).await?;

    Ok(())
}

pub async fn test_email(email_recipient: &str) {
    match mail_link(email_recipient, "TestEmail", "https://www.example.com/not/a/valid/login/link").await {
        Ok(_) => {
            println!("Mail sent successfully, please check your inbox.");
        }
        Err(error) => {
            eprintln!("Error while sending mail:");
            eprintln!("{error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Depends on Internet and valid accound"]
    async fn test_send_link() -> anyhow::Result<()> {
        mail_link("test@vitberget.se","example", "https://vitberget.se/whatever").await?; 
        Ok(())
    }
}
