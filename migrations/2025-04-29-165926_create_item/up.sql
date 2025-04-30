-- Your SQL goes here
CREATE TABLE items (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title NVARCHAR NOT NULL,
  author NVARCHAR,
  pub_date NVARCHAR,
  content TEXT,
  enclosure_id INTEGER,


  link NVARCHAR,
  source_url NVARCHAR,
  FOREIGN KEY(enclosure_id) REFERENCES enclosures(id)
);
