table! {
    todos (id) {
        id -> Int8,
        name -> Text,
        detail -> Nullable<Text>,
        date_from -> Nullable<Timestamp>,
        date_to -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
