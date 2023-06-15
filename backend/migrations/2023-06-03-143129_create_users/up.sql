CREATE TABLE Users (
   user_id INTEGER PRIMARY KEY AUTO_INCREMENT,
   account_number VARCHAR(255) NOT NULL,
   national_id VARCHAR(15) UNIQUE NOT NULL,
   password VARCHAR(16) NOT NULL
);


CREATE TABLE Guests(
   national_id VARCHAR(15) PRIMARY KEY,
   name VARCHAR(255) NOT NULL,
   transaction_type VARCHAR(255) NOT NULL,
   telephone_num VARCHAR(10) NOT NULL
);



CREATE TABLE Tellers(
   server_id VARCHAR(255) PRIMARY KEY,
   server_station INT NOT NULL,
   service_time FLOAT NOT NULL,
   active BOOLEAN NOT NULL,
   password VARCHAR(16) NOT NULL
);


CREATE TABLE Transactions (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    transaction_detail VARCHAR(255) NOT NULL,
    server_id VARCHAR(255) NOT NULL,
    national_id VARCHAR(15),
    guest_national_id VARCHAR(15),
    duration FLOAT NOT NULL,
    transaction_time TIMESTAMP NOT NULL,
    -- Foreign Key pair
    FOREIGN KEY (national_id) REFERENCES Users(national_id),
    FOREIGN KEY (guest_national_id) REFERENCES Guests(national_id),
    FOREIGN KEY (server_id) REFERENCES Tellers(server_id)
);


