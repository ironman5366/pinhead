CREATE TABLE IF NOT EXISTS content_fields (
    id  SERIAL PRIMARY KEY,
    name VARCHAR(255),
    code VARCHAR(255) UNIQUE NOT NULL,
    system BOOL NOT NULL,
    schema JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS content_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR (255) NOT NULL,
    collection BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS content_type_fields (
    id SERIAL PRIMARY KEY,
    content_type_id INT NOT NULL,
    content_field_id INT NOT NULL,
    field_rank INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_content_type FOREIGN KEY (content_type_id) REFERENCES content_types(id),
    CONSTRAINT fk_content_field FOREIGN KEY (content_field_id) REFERENCES content_fields(id)
);

ALTER TABLE documents
    ADD COLUMN content_type_id INTEGER NOT NULL REFERENCES content_types(id);
