-- This file should undo anything in `up.sql`
DROP VIEW IF EXISTS valid_urls;

DROP FUNCTION IF EXISTS insert_into_urls_table CASCADE;