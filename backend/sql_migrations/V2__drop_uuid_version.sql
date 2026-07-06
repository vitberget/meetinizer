DROP INDEX IF EXISTS unique_index_name_uuid_version;
ALTER TABLE meetings DROP COLUMN uuid;
ALTER TABLE meetings DROP COLUMN version;
