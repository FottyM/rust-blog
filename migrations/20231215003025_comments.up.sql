-- Add up migration script here

CREATE TABLE IF NOT EXISTS comments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  post_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS comments_id_idx ON comments (id);

CREATE UNIQUE INDEX IF NOT EXISTS comments_user_id_and_post_id_idx ON comments (user_id, post_id) WHERE deleted_at IS NULL;

CREATE TRIGGER IF NOT EXISTS update_updated_at
  AFTER UPDATE ON comments
  WHEN NEW.deleted_at IS NULL
BEGIN
  UPDATE comments
  SET updated_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END