use crate::schema::item_category;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = item_category)]
pub struct ItemCategory {
    pub item_id: i32,
    pub category_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = item_category)]
pub struct NewItemCategory {
    pub item_id: i32,
    pub category_id: Option<i32>,
}

impl ItemCategory {
    pub fn create(
        conn: &mut SqliteConnection,
        item_id: i32,
        category_id: Option<i32>,
    ) -> Result<Self, diesel::result::Error> {
        let new_item_category = NewItemCategory {
            item_id,
            category_id,
        };

        diesel::insert_into(item_category::table)
            .values(&new_item_category)
            .execute(conn)?;

        item_category::table.find(item_id).first(conn)
    }

    pub fn find_by_item_id(
        conn: &mut SqliteConnection,
        item_id: i32,
    ) -> Result<Self, diesel::result::Error> {
        item_category::table.find(item_id).first(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        item_id: i32,
        category_id: Option<i32>,
    ) -> Result<Self, diesel::result::Error> {
        diesel::update(item_category::table.find(item_id))
            .set(item_category::category_id.eq(category_id))
            .execute(conn)?;

        Self::find_by_item_id(conn, item_id)
    }

    pub fn delete(
        conn: &mut SqliteConnection,
        item_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(item_category::table.find(item_id)).execute(conn)
    }
}
