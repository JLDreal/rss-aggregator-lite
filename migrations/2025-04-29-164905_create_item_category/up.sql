-- Your SQL goes here
CREATE TABLE item_category (
    item_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    category_id INTEGER,
    FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY(category_id) REFERENCES categories(id) ON DELETE CASCADE
);
CREATE INDEX item_index ON item_category(item_id);
CREATE INDEX category_index ON item_category(category_id);
