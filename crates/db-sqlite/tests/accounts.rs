use time::{Duration, OffsetDateTime};

use klick_application::{AccountRecord, AccountRepo, AccountTokenRepo, ProjectRepo};
use klick_boundary::FormData;
use klick_db_sqlite::Connection;
use klick_domain::{
    authentication::{Account, AccountToken, EmailNonce, Nonce, Password},
    Project, ProjectId,
};

#[test]
fn delete_outdated_unconfirmed_accounts() {
    let db = Connection::establish(":memory:").unwrap();
    db.run_embedded_database_migrations().unwrap();

    let now = OffsetDateTime::now_utc();

    let new_account = AccountRecord {
        account: Account {
            email_address: "new@bar.baz".parse().unwrap(),
            email_confirmed: false,
            created_at: now,
        },
        password: "very-secret".parse::<Password>().unwrap().to_hashed(),
    };

    let outdated_account = AccountRecord {
        account: Account {
            email_address: "outdated@bar.baz".parse().unwrap(),
            email_confirmed: false,
            created_at: now - Duration::days(2),
        },
        password: "very-secret".parse::<Password>().unwrap().to_hashed(),
    };
    db.save_account(&new_account).unwrap();
    db.save_account(&outdated_account).unwrap();

    assert!(db
        .find_account(&"new@bar.baz".parse().unwrap())
        .unwrap()
        .is_some());
    assert!(db
        .find_account(&"outdated@bar.baz".parse().unwrap())
        .unwrap()
        .is_some());

    // -- add account token
    let email_nonce = EmailNonce {
        email: outdated_account.account.email_address.clone(),
        nonce: Nonce::new(),
    };
    let expires_at = OffsetDateTime::now_utc() + Duration::hours(5);
    let token = AccountToken {
        email_nonce,
        expires_at,
    };
    db.replace_account_token(token).unwrap();

    // -- add project
    let id = ProjectId::new();
    let created_at = OffsetDateTime::now_utc();
    let modified_at = None;
    let data = FormData::default().into();
    let project = Project {
        id,
        created_at,
        modified_at,
        data,
    };
    db.save_project(project, &outdated_account.account.email_address)
        .unwrap();

    // -- delete outated
    let created_before = now - Duration::days(2);
    db.delete_old_unconfirmed_accounts(created_before).unwrap();

    assert!(db
        .find_account(&"new@bar.baz".parse().unwrap())
        .unwrap()
        .is_some());
    assert!(db
        .find_account(&"outdated@bar.baz".parse().unwrap())
        .unwrap()
        .is_none());
}
