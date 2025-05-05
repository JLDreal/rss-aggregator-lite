// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Nullable<Text>,
        domain -> Nullable<Text>,
    }
}

diesel::table! {
    enclosures (id) {
        id -> Integer,
        url -> Nullable<Text>,
        len -> Nullable<Text>,
        mime_type -> Nullable<Text>,
    }
}

diesel::table! {
    item_category (item_id) {
        item_id -> Integer,
        category_id -> Nullable<Integer>,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        title -> Text,
        author -> Nullable<Text>,
        pub_date -> Nullable<Text>,
        content -> Nullable<Text>,
        enclosure_id -> Nullable<Integer>,
        link -> Nullable<Text>,
        source_url -> Nullable<Text>,
    }
}

diesel::joinable!(item_category -> categories (category_id));
diesel::joinable!(item_category -> items (item_id));
diesel::joinable!(items -> enclosures (enclosure_id));

diesel::allow_tables_to_appear_in_same_query!(categories, enclosures, item_category, items,);
