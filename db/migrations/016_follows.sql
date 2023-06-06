CREATE TABLE follows (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    follows_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    muted BOOL,

    UNIQUE(user_id, follows_id)
);
CREATE INDEX follows_created_index ON follows (created);
CREATE INDEX follows_follows_id_index ON follows (follows_id);
