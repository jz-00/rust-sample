CREATE TYPE user_role AS ENUM ('admin', 'editor', 'viewer', 'inactive');

CREATE TABLE orgs (
    id INT8 REFERENCES users ON DELETE CASCADE PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX orgs_created_index ON orgs (created);

CREATE TABLE org_members (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    org_id INT8 REFERENCES orgs ON DELETE CASCADE NOT NULL,
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    role user_role NOT NULL,

    UNIQUE(org_id, user_id)
);
CREATE INDEX org_members_created_index ON org_members (created);
CREATE INDEX org_members_user_index ON org_members (user_id);
