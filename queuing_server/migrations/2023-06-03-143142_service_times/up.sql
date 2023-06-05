CREATE TABLE teller (
   teller_id VARCHAR(255) PRIMARY KEY,
   transaction VARCHAR(255),
   service_time FLOAT
);

CREATE TABLE Transaction(
    transaction VARCHAR(255) PRIMARY KEY,
    server VARCHAR(255)
)
