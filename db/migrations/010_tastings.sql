CREATE TYPE tasting_status AS ENUM ('future', 'past', 'archived', 'canceled');

CREATE TYPE attendee_status AS ENUM ('invited', 'accepted', 'requested', 'canceled');

CREATE TYPE payment_type AS ENUM ('pre_pay', 'cash', 'card');

CREATE TABLE tastings (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT8 REFERENCES users ON DELETE SET NULL NOT NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    image JSONB,
    invitation visibility NOT NULL,
    status tasting_status NOT NULL,
    min_attendees INT2,
    max_attendees INT2,
    -- require explicit approval if public tasting and location used
    explicit_attendee_approval BOOL NOT NULL,
    datetime TIMESTAMPTZ NOT NULL,
    place_id INT8 REFERENCES places ON DELETE RESTRICT,
    -- if public tasting, only visible to accepted attendees
    location GEOMETRY(POINT, 4326),
    -- if public tasting, only visible to accepted attendees
    address VARCHAR,
    -- total cost will be split per person
    total_cost FLOAT4,
    -- cost per person
    individual_cost FLOAT4,
    -- required if cost set
    currency currency,
    payment_type payment_type
);
CREATE INDEX tastings_created_index ON tastings (created);
CREATE INDEX tastings_user_index ON tastings (user_id);
CREATE INDEX tastings_org_member_index ON tastings (org_member_id);
CREATE INDEX tastings_title_index ON tastings (title);
CREATE INDEX tastings_invitation_index ON tastings (invitation);
CREATE INDEX tastings_datetime_index ON tastings (datetime);
CREATE INDEX tastings_place_index ON tastings (place_id);
CREATE INDEX tastings_location_index ON tastings USING gist (location);

CREATE TABLE tasting_attendees (
    tasting_id INT8 REFERENCES tastings ON DELETE RESTRICT NOT NULL,
    user_id INT8 REFERENCES users ON DELETE SET NULL NOT NULL,
    status attendee_status NOT NULL,

    PRIMARY KEY (tasting_id, user_id)
);
CREATE INDEX tasting_attendees_user_index ON tasting_attendees (user_id);

CREATE TABLE tasting_wines (
    tasting_id INT8 REFERENCES tastings ON DELETE RESTRICT NOT NULL,
    wine_vintage_id INT8 REFERENCES wine_vintages ON DELETE RESTRICT NOT NULL,
    notes VARCHAR,

    -- stats
    judgement_count INT4 NOT NULL,
    positive_pct FLOAT4 NOT NULL,
    neutral_pct FLOAT4 NOT NULL,
    negative_pct FLOAT4 NOT NULL,
    fault_pct FLOAT4 NOT NULL,
    score_count INT4 NOT NULL,
    score_avg FLOAT4 NOT NULL,

    PRIMARY KEY (tasting_id, wine_vintage_id)
);
CREATE INDEX tasting_wines_wine_index ON tasting_wines (wine_vintage_id);
