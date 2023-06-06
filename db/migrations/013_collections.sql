CREATE TYPE collection_context AS ENUM ('tag', 'post', 'comment');

CREATE TABLE collections (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE CASCADE,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    name VARCHAR NOT NULL,
    context collection_context NOT NULL,
    visibility visibility NOT NULL,
    notes VARCHAR,

    -- owning record if present
    post_id INT8 REFERENCES posts ON DELETE CASCADE,

    UNIQUE(user_id, name)
);
CREATE INDEX collections_created_index ON collections (created);
CREATE INDEX collections_user_index ON collections (user_id);
CREATE INDEX collections_context_index ON collections (context);
CREATE INDEX collections_visibility_index ON collections (visibility);

-- add FOREIGN KEY constraint after collections table is defined
ALTER TABLE posts ADD CONSTRAINT posts_collection_id_fkey
FOREIGN KEY (collection_id) REFERENCES collections ON DELETE RESTRICT;

CREATE TABLE collection_items (
    collection_id INT8 REFERENCES collections ON DELETE CASCADE PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    wine_id INT8 REFERENCES wines ON DELETE RESTRICT,
    wine_vintage_id INT8 REFERENCES wine_vintages ON DELETE RESTRICT,
    drink_id INT8 REFERENCES drinks ON DELETE RESTRICT,
    tasting_id INT8 REFERENCES tastings ON DELETE RESTRICT,
    post_id INT8 REFERENCES posts ON DELETE CASCADE,
    place_id INT8 REFERENCES places ON DELETE RESTRICT,
    producer_id INT8 REFERENCES producers ON DELETE CASCADE,
    user_id INT8 REFERENCES users ON DELETE CASCADE
);
CREATE INDEX collection_items_created_index ON collection_items (created);
CREATE INDEX collection_items_wine_index ON collection_items (wine_id);
CREATE INDEX collection_items_wine_vintage_index ON collection_items (wine_vintage_id);
CREATE INDEX collection_items_drink_index ON collection_items (drink_id);
CREATE INDEX collection_items_tasting_index ON collection_items (tasting_id);
CREATE INDEX collection_items_post_index ON collection_items (post_id);
CREATE INDEX collection_items_place_index ON collection_items (place_id);
CREATE INDEX collection_items_producer_index ON collection_items (producer_id);
CREATE INDEX collection_items_user_index ON collection_items (user_id);
