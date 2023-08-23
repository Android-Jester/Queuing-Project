import csv
import pymysql

# Connect to the database
connection = pymysql.connect(
    host="localhost",
    user="root",
    password="$ErrorJester1966",
    database="queue_database",
)


def add_transactions():
    # Open the CSV file
    with open("transactions.csv", "r") as csvfile:
        reader = csv.reader(csvfile, delimiter=",")
        count = 0
        # Iterate over the rows in the CSV file
        for row in reader:
            print(row)
            if count > 0:
                # Insert the row into the database
                sql = "INSERT INTO Transactions (detail, server_id, client_national_id, duration, created_date) VALUES (%s, (SELECT server_id FROM Servers WHERE server_id = %s), %s, %s, %s)"  # noqa: E501
                cursor = connection.cursor()
                cursor.execute(sql, row)
            count = count + 1

        # Commit the changes to the database
        connection.commit()

        # Close the connection to the database
        connection.close()


def add_servers():
    with open("servers.csv", "r") as csvfile:
        reader = csv.reader(csvfile, delimiter=",")
        count = 0
        # Iterate over the rows in the CSV file
        for row in reader:
            print(row)
            if count > 0:
                # Insert the row into the database
                sql = "INSERT INTO Servers (server_id, station, service_time, active, password) VALUES (%s, %s, %s, %s, %s)"  # noqa: E501
                cursor = connection.cursor()
                cursor.execute(sql, row)
            count = count + 1

        # Commit the changes to the database
        connection.commit()

        # Close the connection to the database
        connection.close()


def add_users():
    with open("clients.csv", "r") as csvfile:
        reader = csv.reader(csvfile, delimiter=",")
        count = 0
        # Iterate over the rows in the CSV file
        for row in reader:
            print(row)
            if count > 0:
                # Insert the row into the database
                sql = "INSERT INTO Clients (name, account_number, national_id, password) VALUES (%s, %s, %s, %s)"  # noqa: E501
                cursor = connection.cursor()
                cursor.execute(sql, row)
            count = count + 1

        # Commit the changes to the database
        connection.commit()

        # Close the connection to the database
        connection.close()


add_transactions()
# add_users()
# add_servers()
