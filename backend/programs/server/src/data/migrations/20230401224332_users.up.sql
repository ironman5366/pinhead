CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE TABLE IF NOT EXISTS user_tokens (
    -- The nanoid that'll be prepended to the token to uniquely identify it
    key VARCHAR(21) PRIMARY KEY,
    user_id INT NOT NULL,
    digest VARCHAR(128) NOT NULL UNIQUE,

    expiry TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_user_tokens_user FOREIGN KEY (user_id) REFERENCES users (id)
);


