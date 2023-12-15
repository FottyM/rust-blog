-- Add up migration script here

CREATE TABLE IF NOT EXISTS tags (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS tags_name_idx ON tags (name);

CREATE UNIQUE INDEX IF NOT EXISTS tags_id_idx ON tags (id);

CREATE TRIGGER IF NOT EXISTS update_updated_at
  AFTER UPDATE ON tags
BEGIN
  UPDATE tags
  SET updated_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END;


CREATE TABLE IF NOT EXISTS tag_blog_link (
  id INTEGER PRIMARY KEY NOT NULL,
  tag_id INTEGER NOT NULL,
  blog_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS tag_blog_link_tag_id_and_blog_id_idx ON tag_blog_link (tag_id, blog_id);

CREATE TRIGGER IF NOT EXISTS update_updated_at
  AFTER UPDATE ON tag_blog_link
BEGIN
  UPDATE tag_blog_link
  SET updated_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END;