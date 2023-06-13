CREATE TABLE users (
   account_number VARCHAR(255) PRIMARY KEY
);

CREATE TABLE teller(
   server_id VARCHAR(255) PRIMARY KEY,
   server_station INT NOT NULL,
   service_time FLOAT NOT NULL,
   active BOOLEAN NOT NULL
);

CREATE TABLE transaction (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    transaction_detail VARCHAR(255) NOT NULL,
    server_id VARCHAR(255) NOT NULL,
    user_account_number VARCHAR(255) NOT NULL,
    duration FLOAT NOT NULL,
    transaction_time TIMESTAMP NOT NULL,
    -- Foreign Key pair
    FOREIGN KEY (user_account_number) REFERENCES users(account_number),
    FOREIGN KEY (server_id) REFERENCES teller(server_id)
);