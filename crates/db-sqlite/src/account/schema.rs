diesel::table! {
    accounts (email) {
        email -> Text,
        email_confirmed -> Bool,
        password -> Text,
    }
}
