// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        completed -> Bool,
    }
}
