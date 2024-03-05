CREATE TABLE file_destination (
  id SERIAL PRIMARY KEY,
  file_source_id INT NOT NULL,
  identifier VARCHAR(100) NOT NULL,
  destination JSONB NOT NULL,
  include_headers BOOLEAN NOT NULL,
  "grouping" JSONB,
  batching JSONB,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ,
  FOREIGN KEY(file_source_id) REFERENCES file_source(id)
);
