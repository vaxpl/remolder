table! {
    files (id) {
        id -> Integer,
        hash -> Text,
        path -> Text,
        mime -> Text,
        size -> Integer,
        createdAt -> Timestamp,
    }
}
