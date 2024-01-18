use std::net::SocketAddr;

use serde_json::{json, Value};
use tokio::net::TcpListener;

use klick_application::{AccountRepo as _, AccountTokenRepo};
use klick_backend::Config;
use klick_db_sqlite::Connection;
use klick_domain::{EmailAddress, EmailNonce, Nonce};

async fn run_server() -> (SocketAddr, Connection) {
    run_server_with_default_test_config().await
}

async fn run_server_with_default_test_config() -> (SocketAddr, Connection) {
    let config = Config::default();
    run_server_with_config(config).await
}

// NOTE:
// We deliberately do not use the test suite of a framework here,
// because we want to write integration tests that are independent
// of the implementation.
async fn run_server_with_config(mut config: Config) -> (SocketAddr, Connection) {
    let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap())
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    config.address = address;
    config.db_connection = ":memory:".to_string();

    let db = klick_backend::create_db_connection(&config).unwrap();
    let router = klick_backend::create_router(db.clone(), &config).unwrap();
    let server_task = async move {
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    };
    tokio::spawn(server_task);
    (config.address, db)
}

const DEFAULT_ENDPOINT: &str = "/api";

fn endpoint(address: SocketAddr, path: &str) -> String {
    format!("http://{address}{DEFAULT_ENDPOINT}{path}")
}

const TEST_ACCOUNT_EMAIL: &str = "test@user.com";
const TEST_ACCOUNT_PASSWORD: &str = "secret";

async fn register_account(addr: SocketAddr, email: &str, pw: &str) {
    let client = reqwest::Client::new();
    let credentials = json!({ "email": email, "password": pw });
    let endpoint = endpoint(addr, "/users");
    client
        .post(endpoint)
        .json(&credentials)
        .send()
        .await
        .unwrap();
}

async fn login_account(addr: SocketAddr, email: &str, pw: &str) -> String {
    let client = reqwest::Client::new();
    let credentials = json!({ "email": email, "password": pw });
    let req = client.post(endpoint(addr, "/login")).json(&credentials);
    let res = req.send().await.unwrap();
    let data = res.json::<Value>().await.unwrap();
    data["token"].as_str().unwrap().to_string()
}

async fn register_and_login_account(
    db: &Connection,
    addr: SocketAddr,
    email: &str,
    pw: &str,
) -> String {
    register_account(addr, email, pw).await;
    set_email_address_as_confirmed(&db, email);
    login_account(addr, email, pw).await
}

async fn register_test_account(addr: SocketAddr) {
    register_account(addr, TEST_ACCOUNT_EMAIL, TEST_ACCOUNT_PASSWORD).await;
}

async fn register_and_login_test_account(db: &Connection, addr: SocketAddr) -> String {
    register_and_login_account(db, addr, TEST_ACCOUNT_EMAIL, TEST_ACCOUNT_PASSWORD).await
}

fn set_email_address_as_confirmed(db: &Connection, email: &str) {
    let email = email.parse().unwrap();
    let mut record = db.find_account(&email).unwrap().unwrap();
    record.account.email_confirmed = true;
    db.save_account(&record).unwrap();
}

mod auth {
    use super::*;

    #[tokio::test]
    async fn register() {
        let (addr, _) = run_server().await;
        let client = reqwest::Client::new();
        let credentials = json!({ "email": "test@user.com", "password": "secret" });
        let endpoint = endpoint(addr, "/users");
        let req = client.post(endpoint).json(&credentials);

        let res = req.send().await.unwrap();
        assert_eq!(res.status(), 200);

        let data = res.json::<Value>().await.unwrap();
        assert_eq!(data, Value::Null);
    }

    #[tokio::test]
    async fn confirm_email_address() {
        let (addr, db) = run_server().await;
        register_test_account(addr).await;
        let client = reqwest::Client::new();
        let email = TEST_ACCOUNT_EMAIL.parse::<EmailAddress>().unwrap();
        let account_token = db.get_account_token_by_email(&email).unwrap();
        let token = account_token.email_nonce.encode_to_string();
        let json = json!({ "token": token });
        let endpoint = endpoint(addr, "/users/confirm-email-address");
        let req = client.post(endpoint).json(&json);
        let res = req.send().await.unwrap();
        assert_eq!(res.status(), 200);
        let record = db.find_account(&email).unwrap().unwrap();
        assert_eq!(record.account.email_confirmed, true);
    }

    #[tokio::test]
    async fn confirm_email_address_with_invalid_token() {
        let (addr, db) = run_server().await;
        register_test_account(addr).await;
        let client = reqwest::Client::new();
        let email = TEST_ACCOUNT_EMAIL.parse::<EmailAddress>().unwrap();
        let nonce = Nonce::new();
        let email_nonce = EmailNonce {
            email: email.clone(),
            nonce,
        };
        let token = email_nonce.encode_to_string();
        let json = json!({ "token": token });
        let endpoint = endpoint(addr, "/users/confirm-email-address");
        let req = client.post(endpoint).json(&json);
        let res = req.send().await.unwrap();
        assert_eq!(res.status(), 400);
        let data = res.json::<Value>().await.unwrap();
        assert_eq!(data["status"], 400);
        let record = db.find_account(&email).unwrap().unwrap();
        assert_eq!(record.account.email_confirmed, false);
    }

    #[tokio::test]
    async fn login_without_confirmed_email() {
        let (addr, _) = run_server().await;
        register_test_account(addr).await;
        let client = reqwest::Client::new();
        let credentials = json!({ "email": "test@user.com", "password": "secret" });
        let endpoint = endpoint(addr, "/login");
        let req = client.post(endpoint).json(&credentials);
        let res = req.send().await.unwrap();
        assert_eq!(res.status(), 401);
        let data = res.json::<Value>().await.unwrap();
        assert_eq!(data["status"], 401);
    }

    #[tokio::test]
    async fn login_with_confirmed_email() {
        let (addr, db) = run_server().await;
        register_test_account(addr).await;
        set_email_address_as_confirmed(&db, TEST_ACCOUNT_EMAIL);
        let client = reqwest::Client::new();
        let credentials = json!({ "email": "test@user.com", "password": "secret" });
        let endpoint = endpoint(addr, "/login");
        let req = client.post(endpoint).json(&credentials);
        let res = req.send().await.unwrap();
        let data = res.json::<Value>().await.unwrap();
        assert!(data["token"].is_string());
    }

    #[tokio::test]
    async fn logout() {
        let (addr, db) = run_server().await;
        let token = register_and_login_test_account(&db, addr).await;
        let client = reqwest::Client::new();
        let endpoint = endpoint(addr, "/logout");
        let req = client.post(endpoint).bearer_auth(token).json(&());
        let res = req.send().await.unwrap();
        assert_eq!(res.status(), 200);
        let data = res.json::<Value>().await.unwrap();
        assert_eq!(data, Value::Null);
    }
}

mod projects {
    use super::*;

    const EXAMPLE_PROJECT: &str = include_str!("unsaved_example_project_v5.json");

    #[tokio::test]
    async fn create() {
        let (addr, db) = run_server().await;
        let token = register_and_login_test_account(&db, addr).await;
        set_email_address_as_confirmed(&db, TEST_ACCOUNT_EMAIL);
        let client = reqwest::Client::new();
        let endpoint = endpoint(addr, "/project");
        let json: Value = serde_json::from_str(EXAMPLE_PROJECT).unwrap();
        let project = &json["project"];
        let req = client.post(endpoint).bearer_auth(token).json(&project);
        let res = req.send().await.unwrap();
        assert_eq!(res.status(), 200);
        let data = res.json::<Value>().await.unwrap();
        assert_eq!(data, Value::Null);
    }
}
