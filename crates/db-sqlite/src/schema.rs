use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    accounts (rowid) {
        rowid -> BigInt,
        email -> Text,
        email_confirmed -> Bool,
        password -> Text,
    }
}

table! {
    account_tokens (rowid) {
        rowid -> BigInt,
        account_rowid -> BigInt,
        expires_at -> BigInt,
        nonce -> Text,
    }
}

table! {
    projects (rowid) {
        rowid -> BigInt,
        project_id -> Text,
        account_rowid -> BigInt,
        data -> Text,
    }
}

joinable!(account_tokens -> accounts (account_rowid));
joinable!(projects -> accounts (account_rowid));

allow_tables_to_appear_in_same_query!(accounts, account_tokens, projects);
