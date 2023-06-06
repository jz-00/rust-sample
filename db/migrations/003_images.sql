CREATE TABLE images (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- user who uploaded
    user_id INT8 REFERENCES users ON DELETE SET NULL NOT NULL,
    -- if image is owned by user
    owner_id INT8 REFERENCES users ON DELETE CASCADE,
    url VARCHAR NOT NULL,
    width INT4 NOT NULL,
    height INT4 NOT NULL,
    aspect_ratio FLOAT4 NOT NULL,
    thumbnail_url VARCHAR NOT NULL,
    thumbnail_width INT4 NOT NULL,
    thumbnail_height INT4 NOT NULL,
    thumbnail_aspect_ratio FLOAT4 NOT NULL,
    colors JSONB
);
CREATE INDEX images_created_index ON images (created);
