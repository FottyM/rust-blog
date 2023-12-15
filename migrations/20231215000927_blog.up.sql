-- Add up migration script here

CREATE TABLE IF NOT EXISTS blogs (
  id INTEGER PRIMARY KEY NOT NULL,
  title VARCHAR(255) NOT NULL UNIQUE,
  content TEXT NOT NULL,
  published_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  author_id INTEGER NOT NULL,
  category_id INTEGER,
  status VARCHAR(5) CHECK (status IN ('draft', 'pending', 'published', 'archived'))
);

CREATE UNIQUE INDEX IF NOT EXISTS blogs_id_idx ON blogs (id) WHERE status IS NOT 'archived';

CREATE UNIQUE INDEX IF NOT EXISTS blogs_title_idx ON blogs (title) WHERE status IS NOT 'archived';

CREATE UNIQUE INDEX IF NOT EXISTS blogs_published_at_idx ON blogs (published_at) WHERE status = 'published';

CREATE UNIQUE INDEX IF NOT EXISTS blogs_author_id_and_category_id_idx ON blogs (author_id, category_id) WHERE status IS NOT 'archived';

CREATE TRIGGER IF NOT EXISTS update_published_at
  AFTER UPDATE ON blogs
  WHEN NEW.status = 'published'
BEGIN
  UPDATE blogs
  SET published_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_updated_at
  AFTER UPDATE ON blogs
  WHEN NEW.deleted_at IS NULL
BEGIN
  UPDATE blogs
  SET updated_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END;