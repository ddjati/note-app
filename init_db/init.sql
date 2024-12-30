CREATE DATABASE IF NOT EXISTS rust_axum_sqlx;
USE rust_axum_sqlx;
CREATE TABLE IF NOT EXISTS notes (
    id CHAR(36) PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
INSERT INTO notes (id, title, content, is_published) VALUES('f1cd96ca-0515-49de-be6d-3e238748668e', 'a note', 'here some reminder, mention @danangdjati', TRUE)