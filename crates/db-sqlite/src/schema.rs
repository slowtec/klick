use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    accounts (id) {
        id -> BigInt,
        email -> Text,
        email_confirmed -> Bool,
        password -> Text,
    }
}

table! {
    account_tokens (id) {
        id -> BigInt,
        account_id -> BigInt,
        expires_at -> BigInt,
        nonce -> Text,
    }
}

joinable!(account_tokens -> accounts (account_id));

allow_tables_to_appear_in_same_query!(accounts, account_tokens,);
