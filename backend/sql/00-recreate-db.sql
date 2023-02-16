-- DEV ONLY - Comment out for keeping db between restart

DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- drop sequence
DROP SEQUENCE IF EXISTS todo_id_seq;

-- DEV ONLY - for quick iteration
CREATE USER app_user PASSWORD '111111';
CREATE DATABASE app_db OWNER app_user ENCODING = 'UTF-8';