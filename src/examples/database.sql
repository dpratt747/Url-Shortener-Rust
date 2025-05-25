DROP TABLE IF EXISTS urls;

CREATE TABLE IF NOT EXISTS urls
(
    id         SERIAL PRIMARY KEY,
    long_url   VARCHAR(255) NOT NULL,
    short_url  VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);

SELECT *
FROM urls;

INSERT INTO urls ( long_url, short_url) VALUES (random(), random());