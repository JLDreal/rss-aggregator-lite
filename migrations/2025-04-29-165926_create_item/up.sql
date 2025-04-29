-- Your SQL goes here
CREATE TABLE items (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title NVARCHAR NOT NULL,
  author NVARCHAR,
  pub_date NVARCHAR,
  content TEXT,
  enclosure_id int,

  --categories for later


  -- enclosure
  FOREIGN KEY(enclosure_id) REFERENCES enclosures(id),

  link NVARCHAR,
  source_url NVARCHAR
)
