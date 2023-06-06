CREATE TABLE posts (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    -- TODO - text search
    title VARCHAR,
    -- TODO - text search
    message VARCHAR NOT NULL,
    image JSONB,
    -- add FOREIGN KEY constraint after collections table is defined
    collection_id INT8
);
CREATE INDEX posts_created_index ON posts (created);
CREATE INDEX posts_user_index ON posts (user_id);
CREATE INDEX posts_org_member_index ON posts (org_member_id);
CREATE INDEX posts_collection_index ON posts (collection_id);
