use crate::schema::enclosures;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = enclosures)]
pub struct Enclosure {
    pub id: i32,
    pub url: Option<String>,
    pub len: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = enclosures)]
pub struct NewEnclosure<'a> {
    pub url: Option<&'a str>,
    pub len: Option<&'a str>,
    pub mime_type: Option<&'a str>,
}

impl Enclosure {
    pub fn create(
        conn: &mut SqliteConnection,
        url: Option<&str>,
        len: Option<&str>,
        mime_type: Option<&str>,
    ) -> Result<Self, diesel::result::Error> {
        let new_enclosure = NewEnclosure {
            url,
            len,
            mime_type,
        };

        diesel::insert_into(enclosures::table)
            .values(&new_enclosure)
            .execute(conn)?;

        enclosures::table.order(enclosures::id.desc()).first(conn)
    }

    pub fn find_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        enclosures::table.load(conn)
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: i32) -> Result<Self, diesel::result::Error> {
        enclosures::table.find(id).first(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        id: i32,
        url: Option<String>,
        len: Option<String>,
        mime_type: Option<String>,
    ) -> Result<Self, diesel::result::Error> {
        diesel::update(enclosures::table.find(id))
            .set((
                enclosures::url.eq(url),
                enclosures::len.eq(len),
                enclosures::mime_type.eq(mime_type),
            ))
            .execute(conn)?;

        Self::find_by_id(conn, id)
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(enclosures::table.find(id)).execute(conn)
    }
}
