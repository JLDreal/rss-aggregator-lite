// @generated automatically by Diesel CLI.

diesel::table! {
    categories (rowid) {
        rowid -> Integer,
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        domain -> Nullable<Text>,
    }
}

diesel::table! {
    enclosures (rowid) {
        rowid -> Integer,
        id -> Nullable<Integer>,
        url -> Nullable<Text>,
        length -> Nullable<Text>,
        mime_type -> Nullable<Text>,
    }
}

diesel::table! {
    item_category (rowid) {
        rowid -> Integer,
        item_id -> Nullable<Integer>,
        category_id -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    enclosures,
    item_category,
);
