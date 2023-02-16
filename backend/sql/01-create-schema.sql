-- drop sequence
DROP SEQUENCE IF EXISTS todo_id_seq;

CREATE SEQUENCE todo_id_seq;
ALTER SEQUENCE todo_id_seq RESTART WITH 1000;


-- Todo status enum
CREATE TYPE todo_status_enum AS ENUM (
    'open',
    'close'
);

-- Todo 
CREATE TABLE todo (
    id bigserial,
    cid bigint not null, -- creator user id
    ctime timestamp with time zone default now(),
    title text not null,
    status todo_status_enum default 'open'
);