CREATE TABLE users (
   account_number VARCHAR(255) PRIMARY KEY,
   password TEXT NOT NULL
)-- Your SQL goes here

CREATE TABLE guest(
   name TEXT,
   action VARCHAR(255),
   national_id VARCHAR(255) PRIMARY KEY
)