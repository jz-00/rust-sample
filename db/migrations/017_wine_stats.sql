CREATE TABLE wine_review_stats (
    wine_id INT8 REFERENCES wines ON DELETE RESTRICT PRIMARY KEY,

    -- composition (0 - 10)
    composition_count INT4 NOT NULL,

    sweetness_total INT4 NOT NULL,
    sweetness_min INT4 NOT NULL,
    sweetness_max INT4 NOT NULL,
    sweetness_avg FLOAT4 NOT NULL,

    tannin_total INT4 NOT NULL,
    tannin_min INT4 NOT NULL,
    tannin_max INT4 NOT NULL,
    tannin_avg FLOAT4 NOT NULL,

    acidity_total INT4 NOT NULL,
    acidity_min INT4 NOT NULL,
    acidity_max INT4 NOT NULL,
    acidity_avg FLOAT4 NOT NULL,

    body_total INT4 NOT NULL,
    body_min INT4 NOT NULL,
    body_max INT4 NOT NULL,
    body_avg FLOAT4 NOT NULL,

    -- judgement
    judgement_count INT4 NOT NULL,

    positive_total INT4 NOT NULL,
    positive_pct FLOAT4 NOT NULL,

    neutral_total INT4 NOT NULL,
    neutral_pct FLOAT4 NOT NULL,

    negative_total INT4 NOT NULL,
    negative_pct FLOAT4 NOT NULL,

    fault_total INT4 NOT NULL,
    fault_pct FLOAT4 NOT NULL,

    -- score (1 - 100, optional)
    score_count INT4 NOT NULL,

    score_total INT4 NOT NULL,
    score_min INT4 NOT NULL,
    score_max INT4 NOT NULL,
    score_avg FLOAT4 NOT NULL
);
CREATE INDEX wine_review_stats_composition_count_index ON wine_review_stats (composition_count);
CREATE INDEX wine_review_stats_sweetness_avg_index ON wine_review_stats (sweetness_avg);
CREATE INDEX wine_review_stats_tannin_avg_index ON wine_review_stats (tannin_avg);
CREATE INDEX wine_review_stats_acidity_avg_index ON wine_review_stats (acidity_avg);
CREATE INDEX wine_review_stats_body_avg_index ON wine_review_stats (body_avg);
CREATE INDEX wine_review_stats_judgement_count_index ON wine_review_stats (judgement_count);
CREATE INDEX wine_review_stats_positive_pct_index ON wine_review_stats (positive_pct);
CREATE INDEX wine_review_stats_neutral_pct_index ON wine_review_stats (neutral_pct);
CREATE INDEX wine_review_stats_negative_pct_index ON wine_review_stats (negative_pct);
CREATE INDEX wine_review_stats_fault_pct_index ON wine_review_stats (fault_pct);
CREATE INDEX wine_review_stats_score_count_index ON wine_review_stats (score_count);
CREATE INDEX wine_review_stats_score_avg_index ON wine_review_stats (score_avg);

CREATE TABLE wine_descriptor_stats (
    wine_id INT8 REFERENCES wines ON DELETE RESTRICT,
    descriptor VARCHAR,
    count INT4 NOT NULL,
    total INT4 NOT NULL,
    pct FLOAT4 NOT NULL,

    PRIMARY KEY (wine_id, descriptor)
);
CREATE INDEX wine_descriptor_stats_descriptor_index ON wine_descriptor_stats (descriptor);
CREATE INDEX wine_descriptor_stats_count_index ON wine_descriptor_stats (count);
CREATE INDEX wine_descriptor_stats_pct_index ON wine_descriptor_stats (pct);

CREATE TABLE wine_cost_stats (
    wine_id INT8 REFERENCES wines ON DELETE RESTRICT,
    currency currency,
    count INT4 NOT NULL,
    total FLOAT4 NOT NULL,
    min FLOAT4 NOT NULL,
    max FLOAT4 NOT NULL,
    avg FLOAT4 NOT NULL,

    PRIMARY KEY (wine_id, currency)
);
CREATE INDEX wine_cost_stats_currency_index ON wine_cost_stats (currency);
CREATE INDEX wine_cost_stats_count_index ON wine_cost_stats (count);
CREATE INDEX wine_cost_stats_avg_index ON wine_cost_stats (avg);


-- vintage specific stats
CREATE TABLE wine_vintage_review_stats (
    wine_vintage_id INT8 REFERENCES wine_vintages ON DELETE RESTRICT PRIMARY KEY,

    -- composition (0 - 10)
    composition_count INT4 NOT NULL,

    sweetness_total INT4 NOT NULL,
    sweetness_min INT4 NOT NULL,
    sweetness_max INT4 NOT NULL,
    sweetness_avg FLOAT4 NOT NULL,

    tannin_total INT4 NOT NULL,
    tannin_min INT4 NOT NULL,
    tannin_max INT4 NOT NULL,
    tannin_avg FLOAT4 NOT NULL,

    acidity_total INT4 NOT NULL,
    acidity_min INT4 NOT NULL,
    acidity_max INT4 NOT NULL,
    acidity_avg FLOAT4 NOT NULL,

    body_total INT4 NOT NULL,
    body_min INT4 NOT NULL,
    body_max INT4 NOT NULL,
    body_avg FLOAT4 NOT NULL,

    -- judgement
    judgement_count INT4 NOT NULL,

    positive_total INT4 NOT NULL,
    positive_pct FLOAT4 NOT NULL,

    neutral_total INT4 NOT NULL,
    neutral_pct FLOAT4 NOT NULL,

    negative_total INT4 NOT NULL,
    negative_pct FLOAT4 NOT NULL,

    fault_total INT4 NOT NULL,
    fault_pct FLOAT4 NOT NULL,

    -- score (1 - 100)
    score_count INT4 NOT NULL,

    score_total INT4 NOT NULL,
    score_min INT4 NOT NULL,
    score_max INT4 NOT NULL,
    score_avg FLOAT4 NOT NULL
);

CREATE TABLE wine_vintage_descriptor_stats (
    wine_vintage_id INT8 REFERENCES wine_vintages ON DELETE RESTRICT,
    descriptor VARCHAR,
    count INT4 NOT NULL,
    total INT4 NOT NULL,
    pct FLOAT4 NOT NULL,

    PRIMARY KEY (wine_vintage_id, descriptor)
);

CREATE TABLE wine_vintage_cost_stats (
    wine_vintage_id INT8 REFERENCES wine_vintages ON DELETE RESTRICT,
    currency currency,
    count INT4 NOT NULL,
    total FLOAT4 NOT NULL,
    min FLOAT4 NOT NULL,
    max FLOAT4 NOT NULL,
    avg FLOAT4 NOT NULL,

    PRIMARY KEY (wine_vintage_id, currency)
);
