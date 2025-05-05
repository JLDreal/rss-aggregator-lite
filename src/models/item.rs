use crate::schema::items;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub pub_date: Option<String>,
    pub content: Option<String>,
    pub enclosure_id: Option<i32>,
    pub link: Option<String>,
    pub source_url: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub author: Option<&'a str>,
    pub pub_date: Option<&'a str>,
    pub content: Option<&'a str>,
    pub enclosure_id: Option<&'a i32>,
    pub link: Option<&'a str>,
    pub source_url: Option<&'a str>,
}

impl Item {
    pub fn create(
        conn: &mut SqliteConnection,
        title: &str,
        author: Option<&str>,
        pub_date: Option<&str>,
        content: Option<&str>,
        enclosure_id: Option<&i32>,
        link: Option<&str>,
        source_url: Option<&str>,
    ) -> Result<Self, diesel::result::Error> {
        let new_item = NewItem {
            title,
            author,
            pub_date,
            content,
            enclosure_id,
            link,
            source_url,
        };

        diesel::insert_into(items::table)
            .values(&new_item)
            .execute(conn)?;

        items::table.order(items::id.desc()).first(conn)
    }

    pub fn find_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        items::table.load(conn)
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: i32) -> Result<Self, diesel::result::Error> {
        items::table.find(id).first(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        id: i32,
        title: String,
        author: Option<String>,
        pub_date: Option<String>,
        content: Option<String>,
        enclosure_id: Option<i32>,
        link: Option<String>,
        source_url: Option<String>,
    ) -> Result<Self, diesel::result::Error> {
        diesel::update(items::table.find(id))
            .set((
                items::title.eq(title),
                items::author.eq(author),
                items::pub_date.eq(pub_date),
                items::content.eq(content),
                items::enclosure_id.eq(enclosure_id),
                items::link.eq(link),
                items::source_url.eq(source_url),
            ))
            .execute(conn)?;

        Self::find_by_id(conn, id)
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(items::table.find(id)).execute(conn)
    }
}
