CREATE TABLE file_source (
  id SERIAL PRIMARY KEY,
  context VARCHAR(50) NOT NULL,
  identifier VARCHAR(100) NOT NULL,
  description TEXT NOT NULL,
  source JSONB NOT NULL,
  headers BOOLEAN NOT NULL,
  compression JSONB,
  hide_columns INTEGER[],
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ,
  UNIQUE(context, identifier)
);

CREATE INDEX idx_file_source_context ON file_source(context);
CREATE INDEX idx_file_source_identifier ON file_source(identifier);
