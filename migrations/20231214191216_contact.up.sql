-- Add up migration script here
CREATE TABLE IF NOT EXISTS messages (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  created_at TIMESTAMP CURRENT_TIME DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP CURRENT_TIME DEFAULT CURRENT_TIMESTAMP
);

-- Create id index
CREATE UNIQUE INDEX IF NOT EXISTS messages_id_idx ON messages (id);

-- Create a trigger to update the updated_at field
CREATE TRIGGER IF NOT EXISTS update_updated_at
  AFTER UPDATE ON messages
  WHEN NEW.deleted_at IS NULL
BEGIN
  UPDATE messages
  SET updated_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END