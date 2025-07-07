-- Your SQL goes here

CREATE INDEX idx_urls_created_at ON urls(created_at);

-- A view that gets urls that are no older than 30 minutes
CREATE OR REPLACE VIEW urls_within_designated_mins AS
SELECT *
FROM urls
WHERE created_at >= NOW() - INTERVAL '30 minutes';

-- Override insert to urls_within_designated_mins table and instead insert into urls table
CREATE OR REPLACE FUNCTION insert_into_urls_table()
    RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO urls(long_url, short_url) VALUES (NEW.long_url, NEW.short_url);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS valid_urls_insert_trigger ON urls_within_designated_mins CASCADE;

CREATE TRIGGER valid_urls_insert_trigger
    INSTEAD OF INSERT ON urls_within_designated_mins
    FOR EACH ROW EXECUTE FUNCTION insert_into_urls_table();