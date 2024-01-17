use std::{
    fmt,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};

use url::Url;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_IP_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_DB_URL: &str = "db.sqlite";

#[derive(Debug, Clone)]
pub struct Config {
    pub address: SocketAddr,
    pub base_url: Url,
    pub db_connection: String,
    pub smtp: Option<SmtpConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let address = (DEFAULT_IP_ADDRESS, DEFAULT_PORT).into();
        let base_url = format!("http://{address}").parse().expect("valid base URL");
        let db_connection = DEFAULT_DB_URL.to_string();
        let smtp = None;
        Self {
            address,
            base_url,
            db_connection,
            smtp,
        }
    }
}

#[derive(Clone)]
pub struct SmtpConfig {
    pub username: String,
    pub password: String,
    pub server: String,
    pub from: String,
    pub encryption: Encryption,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Encryption {
    STARTTLS,
    TLS,
}

impl fmt::Debug for SmtpConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SmtpConfig")
            .field("username", &self.username)
            .field("password", &"***")
            .field("server", &self.server)
            .field("from", &self.from)
            .field("encryption", &self.encryption)
            .field("port", &self.port)
            .finish()
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let raw = raw::Config::from_file(path)?;
        Ok(Self::try_from(raw)?)
    }
}

mod raw {
    use std::{fs, path::Path};

    use super::{DEFAULT_DB_URL, DEFAULT_IP_ADDRESS, DEFAULT_PORT};
    use anyhow::bail;
    use serde::Deserialize;
    use url::Url;

    #[derive(Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Config {
        address: Option<String>,
        port: Option<u16>,
        base_url: Url,
        db_connection: Option<String>,
        smtp: Option<Smtp>,
    }

    #[derive(Deserialize)]
    struct Smtp {
        username: String,
        password: String,
        server: String,
        from: String,
        encryption: Option<String>,
        port: Option<u16>,
    }

    impl Config {
        pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
            let string = fs::read_to_string(path)?;
            let raw = toml::from_str::<Config>(&string)?;
            Ok(raw)
        }
    }

    impl TryFrom<Config> for super::Config {
        type Error = anyhow::Error;
        fn try_from(from: Config) -> Result<Self, Self::Error> {
            let Config {
                address,
                port,
                base_url,
                db_connection,
                smtp,
            } = from;
            let ip_address = match address {
                Some(addr) => addr.parse()?,
                None => DEFAULT_IP_ADDRESS,
            };
            let port = port.unwrap_or(DEFAULT_PORT);
            let address = (ip_address, port).into();
            let db_connection = db_connection.unwrap_or_else(|| DEFAULT_DB_URL.to_string());
            let smtp = smtp.map(super::SmtpConfig::try_from).transpose()?;
            Ok(Self {
                address,
                base_url,
                db_connection,
                smtp,
            })
        }
    }

    impl TryFrom<Smtp> for super::SmtpConfig {
        type Error = anyhow::Error;

        fn try_from(from: Smtp) -> Result<Self, Self::Error> {
            let Smtp {
                username,
                password,
                server,
                from,
                encryption,
                port,
            } = from;

            let encryption = match encryption {
                None => super::Encryption::TLS,
                Some(enc) => match &*enc.to_lowercase() {
                    "starttls" => super::Encryption::STARTTLS,
                    "tls" | "ssl" | "ssl/tls" => super::Encryption::TLS,
                    _ => bail!("Unknown SMTP transport encryption {enc:?}"),
                },
            };
            Ok(Self {
                username,
                password,
                server,
                from,
                encryption,
                port,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE_CFG: &str = include_str!("example-config.toml");

        #[test]
        fn read_from_toml() {
            let raw = toml::from_str::<Config>(EXAMPLE_CFG).unwrap();
            assert_eq!(raw.address.as_deref(), Some("127.0.0.1"));
            assert_eq!(raw.port, Some(3000));
            assert_eq!(raw.base_url.as_str(), "https://example.org/");
            assert_eq!(raw.db_connection.as_deref(), Some("db.sqlite"));
            let smtp = raw.smtp.unwrap();
            assert_eq!(smtp.username, "no-reply@example.org");
            assert_eq!(smtp.password, "very-secret");
            assert_eq!(smtp.server, "smpt.example.org");
            assert_eq!(smtp.from, "Klimabilanzkläranlage <no-reply@example.org>");
        }
    }
}
