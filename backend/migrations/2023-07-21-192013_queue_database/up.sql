-- Your SQL goes here
CREATE TABLE Clients (
   id INTEGER PRIMARY KEY AUTO_INCREMENT,
   name TEXT NOT NULL,
   account_number VARCHAR(255) NOT NULL,
   national_id VARCHAR(16) UNIQUE NOT NULL,
   password VARCHAR(16) NOT NULL
);

CREATE TABLE Guests_Clients(
   national_id VARCHAR(15) PRIMARY KEY,
   name TEXT NOT NULL,
   transaction_detail TEXT NOT NULL,
   telephone_num TEXT NOT NULL
);


CREATE TABLE Servers(
   server_id VARCHAR(255) PRIMARY KEY,
   station INT NOT NULL,
   service_time INTEGER NOT NULL,
   password VARCHAR(16) NOT NULL,
   active BOOL NOT NULL
);


CREATE TABLE Transactions (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    detail VARCHAR(255) NOT NULL,
    server_id VARCHAR(255) NOT NULL,
    client_national_id VARCHAR(15) NOT NULL,
    duration FLOAT NOT NULL,
    created_date TIMESTAMP NOT NULL,
    -- Foreign Key pair
    FOREIGN KEY (server_id) REFERENCES Servers(server_id)
);