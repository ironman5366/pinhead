CREATE TABLE IF NOT EXISTS documents (
   id SERIAL PRIMARY KEY,
   title VARCHAR(255) NOT NULL,
   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS document_versions (
   id SERIAL PRIMARY KEY,
   document_id INTEGER NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
   content JSONB NOT NULL,
   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_document_versions_created_at ON document_versions (created_at);
