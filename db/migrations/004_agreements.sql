CREATE TABLE agreements (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    accepted TIMESTAMPTZ,
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    title VARCHAR NOT NULL,
    version INT2 NOT NULL,

    UNIQUE(user_id, org_member_id, title, version)
);
CREATE INDEX agreements_created_index ON agreements (created);
