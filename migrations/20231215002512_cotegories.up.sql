-- Add up migration script here
CREATE TABLE IF NOT EXISTS categories (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS categories_id_idx ON categories (id);

CREATE UNIQUE INDEX IF NOT EXISTS categories_name_and_id_idx ON categories (name, id);

CREATE TRIGGER IF NOT EXISTS update_updated_at
  AFTER UPDATE ON categories
  WHEN NEW.deleted_at IS NULL
BEGIN
  UPDATE categories
  SET updated_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END