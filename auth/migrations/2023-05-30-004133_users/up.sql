CREATE TABLE users (
   account_number VARCHAR(255) PRIMARY KEY,
   password TEXT NOT NULL
);

CREATE TABLE guests(
   name VARCHAR(255),
   action VARCHAR(255),
   national_id VARCHAR(255) PRIMARY KEY
)