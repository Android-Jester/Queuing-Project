# Queuing Modelling System

# User(View)

## Auth
1. Login the user to the service
2. If user is guest store the data in the guest database
3. If user is account holder, query his details in the database

## Teller Assignment
1. User selects the actions they want to perform
2. Querying the database for the transactions to feed to the random forest algorithm
3. Determine the position of the user and the teller he/she is assigned to based on least service time
4. If the user prefers to leave at any point, readjust the queue to exclude the user

# Teller(View)
## Auth
1. Login the teller using their id and passcode
2. Obtain a view of the current users in the queue assigned to him or her

## Activities
1. Upon servicing the customer, teller clicks completed upon a successful transaction else cancel for unsuccessful transactions
2. Successful requests are stored in the transactions table in the database
3. Dispel the user from the queue, and readjust the waiting time



# Manager(View)
1. Obtain the analytics of the overall performance of the bank and each of the tellers
2. Generate a document report based on the data acquired from the analytics
