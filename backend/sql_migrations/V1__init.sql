CREATE TABLE meetings (
    id int,
    name varchar(255),
    uuid varchar(50),
    version varchar(50),
    created datetime default current_timestamp,
    json text
);

CREATE UNIQUE INDEX unique_index_name_uuid_version ON meetings(name, uuid, version);
