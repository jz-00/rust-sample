CREATE TYPE drink_judgement AS ENUM ('positive', 'neutral', 'negative', 'fault');

CREATE TABLE drinks (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    wine_vintage_id INT8 REFERENCES wine_vintages ON DELETE RESTRICT NOT NULL,
    place_id INT8 REFERENCES places ON DELETE RESTRICT,
    tasting_id INT8 REFERENCES tastings ON DELETE RESTRICT,

    -- ratings 0 - 9
    sweetness INT2 NOT NULL,
    tannin INT2 NOT NULL,
    acidity INT2 NOT NULL,
    body INT2 NOT NULL,

    -- { categories: [<string>], terms: [<string>] }; terms includes categories
    descriptors JSONB NOT NULL,
    judgement drink_judgement NOT NULL,
    -- score 1 - 100
    score INT2,
    summary VARCHAR,
    notes VARCHAR
);
CREATE INDEX drinks_created_index ON drinks (created);
CREATE INDEX drinks_wine_index ON drinks (wine_vintage_id);
CREATE INDEX drinks_place_index ON drinks (place_id);
CREATE INDEX drinks_tasting_index ON drinks (tasting_id);
-- GIN index for containment
CREATE INDEX drinks_descriptors_index ON drinks USING GIN ((descriptors -> 'terms') jsonb_path_ops);
CREATE INDEX drinks_judgement_index ON drinks (judgement);
CREATE INDEX drinks_score_index ON drinks (score);

CREATE TABLE drink_details (
    drink_id INT8 REFERENCES drinks ON DELETE RESTRICT PRIMARY KEY,
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    image JSONB,
    location GEOMETRY(POINT, 4326),
    private_notes VARCHAR
);
CREATE INDEX drink_details_user_index ON drink_details (user_id);
CREATE INDEX drink_details_org_member_index ON drink_details (org_member_id);
CREATE INDEX drink_details_location_index ON drink_details USING gist (location);
