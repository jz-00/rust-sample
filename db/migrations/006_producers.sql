CREATE TABLE producers (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- autocomplete
    name VARCHAR NOT NULL,
    image JSONB
);
CREATE INDEX producers_created_index ON producers (created);

CREATE TABLE producer_places (
    producer_id INT8 REFERENCES producers ON DELETE CASCADE,
    place_id INT8 REFERENCES places ON DELETE RESTRICT,

    PRIMARY KEY (producer_id, place_id)
);
