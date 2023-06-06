-- Maintain same structure for different contexts (tasting, drink, post, collection).

CREATE TABLE tasting_comments (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE SET NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    target_id INT8 REFERENCES tastings ON DELETE RESTRICT NOT NULL,
    reply_to_id INT8 REFERENCES tasting_comments ON DELETE RESTRICT,
    message VARCHAR NOT NULL
);
CREATE INDEX tasting_comments_created_index ON tasting_comments (created);
CREATE INDEX tasting_comments_user_index ON tasting_comments (user_id);
CREATE INDEX tasting_comments_org_member_index ON tasting_comments (org_member_id);
CREATE INDEX tasting_comments_target_index ON tasting_comments (target_id);

CREATE TABLE drink_comments (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE SET NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    target_id INT8 REFERENCES drinks ON DELETE RESTRICT NOT NULL,
    reply_to_id INT8 REFERENCES drink_comments ON DELETE RESTRICT,
    message VARCHAR NOT NULL
);
CREATE INDEX drink_comments_created_index ON drink_comments (created);
CREATE INDEX drink_comments_user_index ON drink_comments (user_id);
CREATE INDEX drink_comments_org_member_index ON drink_comments (org_member_id);
CREATE INDEX drink_comments_target_index ON drink_comments (target_id);

CREATE TABLE post_comments
(
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE SET NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    target_id INT8 REFERENCES posts ON DELETE RESTRICT NOT NULL,
    reply_to_id INT8 REFERENCES post_comments ON DELETE RESTRICT,
    message VARCHAR NOT NULL
);
CREATE INDEX post_comments_created_index ON post_comments (created);
CREATE INDEX post_comments_user_index ON post_comments (user_id);
CREATE INDEX post_comments_org_member_index ON post_comments (org_member_id);
CREATE INDEX post_comments_target_index ON post_comments (target_id);

CREATE TABLE collection_comments
(
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE SET NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    target_id INT8 REFERENCES collections ON DELETE RESTRICT NOT NULL,
    reply_to_id INT8 REFERENCES collection_comments ON DELETE RESTRICT,
    message VARCHAR NOT NULL
);
CREATE INDEX collection_comments_created_index ON collection_comments (created);
CREATE INDEX collection_comments_user_index ON collection_comments (user_id);
CREATE INDEX collection_comments_org_member_index ON collection_comments (org_member_id);
CREATE INDEX collection_comments_target_index ON collection_comments (target_id);
