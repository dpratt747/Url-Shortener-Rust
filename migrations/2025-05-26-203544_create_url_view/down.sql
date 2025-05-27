-- This file should undo anything in `up.sql`
DROP VIEW IF EXISTS urls_within_designated_mins CASCADE;

DROP FUNCTION IF EXISTS insert_into_urls_table CASCADE;

DROP TRIGGER IF EXISTS valid_urls_insert_trigger ON urls_within_designated_mins CASCADE;