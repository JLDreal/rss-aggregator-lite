use crate::schema::categories;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: Option<String>,
    pub domain: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: Option<&'a str>,
    pub domain: Option<&'a str>,
}

impl Category {
    pub fn create(
        conn: &mut SqliteConnection,
        name: Option<&str>,
        domain: Option<&str>,
    ) -> Result<Self, diesel::result::Error> {
        let new_category = NewCategory { name, domain };

        diesel::insert_into(categories::table)
            .values(&new_category)
            .execute(conn)?;

        categories::table.order(categories::id.desc()).first(conn)
    }

    pub fn find_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        categories::table.load(conn)
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: i32) -> Result<Self, diesel::result::Error> {
        categories::table.find(id).first(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        id: i32,
        name: Option<String>,
        domain: Option<String>,
    ) -> Result<Self, diesel::result::Error> {
        diesel::update(categories::table.find(id))
            .set((categories::name.eq(name), categories::domain.eq(domain)))
            .execute(conn)?;

        Self::find_by_id(conn, id)
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(categories::table.find(id)).execute(conn)
    }
}
