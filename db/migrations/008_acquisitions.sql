CREATE TYPE currency AS ENUM (
    'AED', 'AFN', 'ALL', 'AMD', 'ANG', 'AOA', 'ARS', 'AUD', 'AWG', 'AZN',
    'BAM', 'BBD', 'BDT', 'BGN', 'BHD', 'BIF', 'BMD', 'BND', 'BOB', 'BRL',
    'BSD', 'BTN', 'BWP', 'BYN', 'BZD', 'CAD', 'CDF', 'CHF', 'CLP', 'CNY',
    'COP', 'CRC', 'CUC', 'DJF', 'DKK', 'DOP', 'DZD', 'EGP', 'ERN', 'ETB',
    'EUR', 'FJD', 'FKP', 'GBP', 'GEL', 'GHS', 'GIP', 'GMD', 'GNF', 'GTQ',
    'GYD', 'HKD', 'HNL', 'HRK', 'HTG', 'HUF', 'IDR', 'ILS', 'INR', 'IQD',
    'IRR', 'ISK', 'JMD', 'JOD', 'JPY', 'KES', 'KGS', 'KHR', 'KMF', 'KPW',
    'KRW', 'KWD', 'KYD', 'KZT', 'LAK', 'LBP', 'LKR', 'LRD', 'LSL', 'LYD',
    'MAD', 'MDL', 'MGA', 'MMK', 'MNT', 'MRU', 'MUR', 'MVR', 'MWK', 'MXN',
    'MYR', 'MZN', 'NAD', 'NGN', 'NIO', 'NOK', 'NPR', 'NZD', 'OMR', 'PEN',
    'PGK', 'PHP', 'PKR', 'PLN', 'PYG', 'QAR', 'RON', 'RSD', 'RUB', 'RWF',
    'SAR', 'SBD', 'SCR', 'SDG', 'SEK', 'SGD', 'SHP', 'SLL', 'SOS', 'SRD',
    'SSP', 'SVC', 'SYP', 'THB', 'TJS', 'TMT', 'TND', 'TOP', 'TRY', 'TTD',
    'TWD', 'TZS', 'UAH', 'UGX', 'USD', 'UYU', 'UZS', 'VES', 'VND', 'VUV',
    'WST', 'XAF', 'XCD', 'XOF', 'XPF', 'YER', 'ZAR', 'ZMW', 'ZWL'
);

CREATE TABLE acquisitions (
    id SERIAL8 PRIMARY KEY,
    wine_vintage_id INT8 REFERENCES wine_vintages ON DELETE RESTRICT NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    count INT2 NOT NULL, -- bottle count
    cost FLOAT4, -- per bottle
    currency currency,
    place_id INT8 REFERENCES places ON DELETE RESTRICT
);
CREATE INDEX acquisitions_wine_index ON acquisitions (wine_vintage_id);
CREATE INDEX acquisitions_date_index ON acquisitions (date);
CREATE INDEX acquisitions_cost_index ON acquisitions (cost);
CREATE INDEX acquisitions_currency_index ON acquisitions (currency);
CREATE INDEX acquisitions_place_index ON acquisitions (place_id);

CREATE TABLE acquisition_details (
    acquisition_id INT8 REFERENCES acquisitions ON DELETE RESTRICT PRIMARY KEY,
    user_id INT8 REFERENCES users ON DELETE CASCADE NOT NULL,
    org_member_id INT8 REFERENCES org_members ON DELETE SET NULL,
    location GEOMETRY(POINT, 4326),
    notes VARCHAR
);
CREATE INDEX acquisition_details_user_index ON acquisition_details (user_id);
CREATE INDEX acquisition_details_location_index ON acquisition_details USING gist (location);
