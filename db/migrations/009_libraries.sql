CREATE TABLE libraries (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE CASCADE,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    name VARCHAR NOT NULL,
    image JSONB,

    -- stats (total counts)
    wines INT4 DEFAULT 0,

    UNIQUE(user_id, name)
);
CREATE INDEX libraries_created_index ON libraries (created);
CREATE INDEX libraries_name_index ON libraries (name);

CREATE TABLE library_members (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    library_id INT8 REFERENCES libraries ON DELETE CASCADE NOT NULL,
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    role user_role NOT NULL,

    UNIQUE(user_id, library_id)
);
CREATE INDEX library_members_created_index ON library_members (created);
CREATE INDEX library_members_user_index ON library_members (user_id);
CREATE INDEX library_members_library_index ON library_members (library_id);

CREATE TABLE library_wines (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    library_id INT8 REFERENCES libraries ON DELETE CASCADE NOT NULL,
    wine_vintage_id INT8 REFERENCES wine_vintages ON DELETE RESTRICT NOT NULL,
    stored_count INT2 NOT NULL, -- bottle count
    storage_notes VARCHAR,
    image JSONB
);
CREATE INDEX library_wines_created_index ON library_wines (created);
CREATE INDEX library_wines_library_index ON library_wines (library_id);
CREATE INDEX library_wines_wine_index ON library_wines (wine_vintage_id);
CREATE INDEX library_wines_stored_count_index ON library_wines (stored_count);
