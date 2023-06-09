CREATE TABLE users (
   account_number VARCHAR(255) PRIMARY KEY,
);

CREATE TABLE teller(
    server_id VARCHAR(255) PRIMARY KEY,
    server_station INT,
    active BOOLEAN,
);


CREATE TABLE transaction (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    transaction_detail VARCHAR(255),
    server_id VARCHAR(255),
    user_account_number VARCHAR(255),
    duration FLOAT,
    transaction_time TIMESTAMP,
    -- Foreign Key pair
    FOREIGN KEY (user_account_number) REFERENCES users(account_number)
    FOREIGN KEY (server_id) REFERENCES teller(server_id)
);