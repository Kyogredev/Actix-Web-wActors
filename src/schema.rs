// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        name -> Varchar,
        age -> Int2,
        is_sub -> Bool,
    }
}
