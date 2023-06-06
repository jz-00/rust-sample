-- settings enums...
CREATE TYPE app_theme AS ENUM ('default', 'light', 'dark');

-- includes both individuals and organizations
CREATE TABLE users (
    id SERIAL8 PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Firebase uid, NULL implies user is an organization
    uid VARCHAR UNIQUE,
    -- autocomplete
    name VARCHAR NOT NULL,
    whereabouts VARCHAR,
    image JSONB,
    settings JSONB NOT NULL,
    active TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- stats (total counts)
    followers INT4 NOT NULL DEFAULT 0,
    following INT4 NOT NULL DEFAULT 0,
    contacts INT4 NOT NULL DEFAULT 0,
    drinks INT4 NOT NULL DEFAULT 0,
    tastings_organized INT4 NOT NULL DEFAULT 0,
    tastings_attended INT4 NOT NULL DEFAULT 0
);
CREATE INDEX users_created_index ON users (created);
CREATE INDEX users_name_index ON users (name);
CREATE INDEX users_active_index ON users (active);
CREATE INDEX users_followers_index ON users (followers);
CREATE INDEX users_drinks_index ON users (drinks);
CREATE INDEX users_tastings_organized_index ON users (tastings_organized);
CREATE INDEX users_tastings_attended_index ON users (tastings_attended);

CREATE TABLE user_privacy (
    user_id INT8 REFERENCES users ON DELETE CASCADE PRIMARY KEY,

    -- privacy_scopes
    drinks visibility NOT NULL DEFAULT 'private',
    drink_places visibility NOT NULL DEFAULT 'private',
    drink_reviews visibility NOT NULL DEFAULT 'private',
    tastings visibility NOT NULL DEFAULT 'private',
    acquisitions visibility NOT NULL DEFAULT 'private',
    acquisition_dates visibility NOT NULL DEFAULT 'private',
    acquisition_places visibility NOT NULL DEFAULT 'private',
    acquisition_costs visibility NOT NULL DEFAULT 'private'
);
CREATE INDEX user_privacy_drinks_index ON user_privacy (drinks);
CREATE INDEX user_privacy_tastings_index ON user_privacy (tastings);
CREATE INDEX user_privacy_acquisitions_index ON user_privacy (acquisitions);
