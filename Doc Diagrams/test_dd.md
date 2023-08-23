| No | Test Objective | Test Step | Expected Result | Error Result |
|---|---|---|---|---|
| 1 | Start the server and connect to the MySQL Database Server | 1. Start the database server<br/>2. start the program in the command line and add the URL of the database as an argument | Program Started with showing the log of the process | Passed |
| 2 | Login Account holding customer | 1. Enter your account number and password<br/>2. Click on the Login button | User is moved to the Transaction screen | Passed |
| 3 | Joining Queue | 1. Provide the transaction detail<br/>2. Click on the Join Queue Button | User is moved to the Queue page | Passed |
| 4 | Leave Queue | Click on the Leave Queue Button | User is moved back to the Login Screen and the Queue Page of other users would be updated | Passed |
| 5 | Login Bank Employee | 1. Enter your employee ID and password<br/>2. Click on the Login button | Employee is moved back to the Dashboard Screen and sees the list of Customers assigned to him or her | Passed |
| 6 | Recording Successful Transactions | Upon Completing the transaction, click on the Complete button | Customer's name is removed from the table and transaction data is stored in the database | Passed |
| 7 | Cancelling transaction |  Click on the Cancel button | Customer's name is removed from the table | Passed |
| 8 | Logout Teller | Upon an empty queue, click the logout button | Employee is sent back to the login button | Passed |rm 
