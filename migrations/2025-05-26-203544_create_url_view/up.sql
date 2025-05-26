-- Your SQL goes here

CREATE VIEW valid_urls AS
SELECT *
FROM urls
WHERE created_at >= NOW() - INTERVAL '30 minutes';

CREATE OR REPLACE FUNCTION insert_into_urls_table()
    RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO urls(long_url, short_url) VALUES (NEW.long_url, NEW.short_url);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER valid_urls_insert_trigger
    INSTEAD OF INSERT ON valid_urls
    FOR EACH ROW EXECUTE FUNCTION insert_into_urls_table();