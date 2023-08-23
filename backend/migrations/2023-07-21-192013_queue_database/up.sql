CREATE TABLE Clients (
   id INTEGER PRIMARY KEY AUTO_INCREMENT,
   name TEXT NOT NULL,
   account_number VARCHAR(255) NOT NULL,
   national_id VARCHAR(16) UNIQUE NOT NULL,
   password VARCHAR(255) NOT NULL
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
   password VARCHAR(255) NOT NULL,
   active BOOL NOT NULL
);
CREATE TABLE Transactions (
   id INTEGER AUTO_INCREMENT PRIMARY KEY,
   detail TEXT NOT NULL,
   server_id VARCHAR(255) NOT NULL,
   client_national_id VARCHAR(15) NOT NULL,
   duration FLOAT NOT NULL,
   created_date TIMESTAMP NOT NULL,
   -- Foreign Key pair
   FOREIGN KEY (server_id) REFERENCES Servers(server_id)
);
CREATE TABLE MainQueue (
   #    id INTEGER PRIMARY KEY AUTO_INCREMENT,
   national_id VARCHAR(16) PRIMARY KEY NOT NULL,
   position INT NOT NULL,
   name TEXT NOT NULL,
   sub_queue_position INT NOT NULL,
   assigned_server VARCHAR(255) NOT NULL,
   server_location INT NOT NULL,
   activity VARCHAR(255) NOT NULL,
   time_duration INT NOT NULL,
   time_joined TIMESTAMP NOT NULL,
   FOREIGN KEY (assigned_server) REFERENCES Servers(server_id)
);