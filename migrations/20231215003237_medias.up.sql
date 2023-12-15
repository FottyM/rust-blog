-- Add up migration script here

CREATE TABLE IF NOT EXISTS medias (
  id INTEGER PRIMARY KEY NOT NULL,
  file_path VARCHAR(255) NOT NULL,
  post_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS medias_id_idx ON medias (id);

CREATE UNIQUE INDEX IF NOT EXISTS medias_post_id_idx ON medias (post_id);

CREATE TRIGGER IF NOT EXISTS update_updated_at
  AFTER UPDATE ON medias
  WHEN NEW.deleted_at IS NULL
BEGIN
  UPDATE medias
  SET updated_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END
