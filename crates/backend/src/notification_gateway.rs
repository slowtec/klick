use anyhow::bail;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use url::Url;

use klick_application::{NotificationEvent, NotificationGateway};
use klick_domain::{EmailAddress, EmailNonce};

use crate::config::{Config, Encryption};

#[derive(Clone)]
pub struct Gateway {
    base_url: Url,
    mailer: Option<Mailer>,
}

#[derive(Clone)]
struct Mailer {
    transport: SmtpTransport,
    from: Mailbox,
}

impl Gateway {
    pub fn new(config: &Config) -> Self {
        let mailer = mailer_from_cfg(config)
            .map_err(|err| log::warn!("Cannot create SMTP mailer: {err}"))
            .ok();
        let base_url = config.base_url.clone();
        Self { base_url, mailer }
    }
}

fn mailer_from_cfg(config: &Config) -> anyhow::Result<Mailer> {
    let Some(smtp) = &config.smtp else {
        bail!("no SMTP configuration found");
    };
    let from = smtp.from.parse::<Mailbox>()?;
    let creds = Credentials::new(smtp.username.clone(), smtp.password.clone());
    let transport = match smtp.encryption {
        Encryption::TLS => SmtpTransport::relay(&smtp.server)?,
        Encryption::STARTTLS => SmtpTransport::starttls_relay(&smtp.server)?,
    };
    let mut transport = transport.credentials(creds);
    if let Some(port) = smtp.port {
        transport = transport.port(port);
    }
    let transport = transport.build();
    let is_ok = transport.test_connection()?;
    if !is_ok {
        log::warn!("SMTP connection does not work");
    };
    let mailer = Mailer { from, transport };
    Ok(mailer)
}

impl NotificationGateway for Gateway {
    fn notify(&self, event: NotificationEvent) {
        match event {
            NotificationEvent::AccountWasCreated(email_address, nonce) => {
                log::info!("A new account ({email_address:?}) was created.");

                let Some(mailer) = &self.mailer else {
                    log::warn!("No mailer configured: No confirmation e-mail can be sent.");
                    return;
                };

                if let Err(err) =
                    send_address_confirmation_mail(email_address, &nonce, &self.base_url, mailer)
                {
                    log::warn!("Unable to send confirmation e-mail: {err}");
                }
            }
        }
    }
}

fn send_address_confirmation_mail(
    email_address: EmailAddress,
    nonce: &EmailNonce,
    base_url: &Url,
    mailer: &Mailer,
) -> anyhow::Result<()> {
    // TODO: use templates

    let subject = "Bestätigen Sie Ihre E-Mail Addresse".to_string();

    let link = confirmation_url(nonce, base_url)?;
    let body = vec![
        "Bitte bestätigen Sie Ihre E-Mail Addresse ",
        "indem Sie auf folgenden Link klicken:\n\n",
        &link,
    ]
    .join("");

    let email = Message::builder()
        .from(mailer.from.clone())
        .to(email_address.as_str().parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)?;
    mailer.transport.send(&email)?;

    Ok(())
}

const CONFIRM_EMAIL_PAGE_PATH: &str = "confirm-email-address";

fn confirmation_url(token: &EmailNonce, base_url: &Url) -> anyhow::Result<String> {
    let token = token.encode_to_string();
    let link = base_url.join(CONFIRM_EMAIL_PAGE_PATH)?;
    Ok(format!("{link}/{token}"))
}

#[test]
fn create_confirmation_url() {
    let nonce = klick_domain::Nonce::new();
    let email_nonce = EmailNonce {
        email: "foo@bar.com".parse().unwrap(),
        nonce: nonce.clone(),
    };
    let base_url = "http://localhost:3000/".parse().unwrap();
    let url = confirmation_url(&email_nonce, &base_url).unwrap();
    let expected = format!(
        "http://localhost:3000/confirm-email-address/{}",
        email_nonce.encode_to_string()
    );
    assert_eq!(url.to_string(), expected);
}
