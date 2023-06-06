CREATE TYPE wine_color AS ENUM ('red', 'white', 'rose');

CREATE TABLE wines (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- autocomplete
    name VARCHAR NOT NULL,
    image JSONB,
    color VARCHAR NOT NULL,
    styles VARCHAR ARRAY,
    producer_id INT8 REFERENCES producers ON DELETE RESTRICT,
    country VARCHAR NOT NULL,
    regions VARCHAR ARRAY,
    classification_type VARCHAR,
    classification_name VARCHAR,
    -- union of grapes for all vintages
    grapes VARCHAR ARRAY,
    -- #of grapes, used for sorting
    grape_count INT2 NOT NULL DEFAULT 0,
    -- average abv of all vintages
    abv FLOAT4,
    producer_notes VARCHAR,

    UNIQUE(producer_id, name)
);
CREATE INDEX wines_created_index ON wines (created);
CREATE INDEX wines_name_index ON wines (name);
CREATE INDEX wines_color_index ON wines (color);
-- GIN index for containment
CREATE INDEX wines_styles_index ON wines USING GIN (styles array_ops);
CREATE INDEX wines_producer_index ON wines (producer_id);
CREATE INDEX wines_country_index ON wines (country);
-- GIN index for containment
CREATE INDEX wines_regions_index ON wines USING GIN (regions array_ops);
CREATE INDEX wines_classification_type_index ON wines (classification_type);
CREATE INDEX wines_classification_name_index ON wines (classification_name);
-- GIN index for containment
CREATE INDEX wines_grapes_index ON wines USING GIN (grapes array_ops);
CREATE INDEX wines_grape_count_index ON wines (grape_count);
CREATE INDEX wines_abv_index ON wines (abv);

CREATE TABLE wine_vintages (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    wine_id INT8 REFERENCES wines ON DELETE CASCADE NOT NULL,
    vintage INT2 NOT NULL,
    vintage_image JSONB,
    vintage_grapes VARCHAR ARRAY,
    vintage_grape_count INT2 NOT NULL DEFAULT 0,
    vintage_abv FLOAT4,
    vintage_producer_notes VARCHAR,

    UNIQUE (wine_id, vintage)
);
CREATE INDEX wine_vintages_created_index ON wine_vintages (created);
CREATE INDEX wine_vintages_vintage_index ON wine_vintages (vintage);
