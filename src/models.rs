use diesel::prelude::*;
use rss::EnclosureBuilder;

use crate::schema::categories;

#[derive(PartialEq, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub name: Option<String>,
    pub domain: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub domain: &'a str,
}

#[derive(PartialEq, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::enclosures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Enclosure {
    pub id: i32,
    pub url: Option<String>,
    pub len: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = enclosures)]
pub struct NewEnclosure<'a> {
    pub url: &'a str,
    pub len: &'a str,
    pub mime_type: &'a str,
}

#[derive(PartialEq, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub pub_date: Option<String>,
    pub content: Option<String>,
    pub enclosure_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub pub_date: &'a str,
    pub content: &'a str,
    pub enclosure_id: &'a str,
}
