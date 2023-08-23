import mysql.connector

host = "localhost"
user = "raided"
password = "password"
database = "testdb"

mydb = mysql.connector.connect(
    host = host,
    user = user,
    password = password,
    database = database,    
)

my_cursor = mydb.cursor()
my_cursor.execute("CREATE TABLE home")
GRANT ALL PRIVILEGES ON  testdb.* to 'raided'@'localhost';