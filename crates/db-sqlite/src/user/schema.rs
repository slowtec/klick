diesel::table! {
    users (email) {
        email -> Text,
        email_confirmed -> Bool,
        password -> Text,
    }
}
