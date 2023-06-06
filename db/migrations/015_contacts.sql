-- two rows for each bidirectional contact relationship
CREATE TABLE contacts (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    contact_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    muted BOOL,

    UNIQUE(user_id, contact_id)
);
CREATE INDEX contacts_created_index ON contacts (created);

CREATE TABLE contact_requests (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    requested_contact_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,

    UNIQUE(user_id, requested_contact_id)
);
CREATE INDEX contact_requests_created_index ON contact_requests (created);
