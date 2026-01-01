use mail_send::SmtpClientBuilder;
use mail_send::mail_builder::MessageBuilder;
use tracing::info;

use crate::config::{get_mail_from, get_mail_password, get_mail_port, get_mail_server, get_mail_user};

pub async fn mail_link(email: &str, meeting: &str, login_url: &str) -> anyhow::Result<()> {
    let server = get_mail_server()?;
    let port = get_mail_port()?;
    let user = get_mail_user()?;
    let password = get_mail_password()?;
    let from = get_mail_from()?;

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

    let tls = true;
    info!("Connecting to SMTP server {server}:{port} tls:{tls}");
    let mut client = SmtpClientBuilder::new(server, port as u16)
        .implicit_tls(tls)
        .credentials((user, password))
        .connect()
        .await?;

    info!("Sending mail {email} {login_url}");
    client.send(message).await?;

    Ok(())
}

pub async fn test_email(email: &str) {
    match mail_link(email, "TestEmail", "https://www.example.com/not/a/valid/login/link").await {
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
