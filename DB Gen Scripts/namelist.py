import csv
import datetime
import random

transaction_details = [
    "Deposit",
    "Withdrawal",
    "Foreign Exchange",
    "Payment of Fees",
    "Bill Payment",
]


def namesGen():
    names = []
    with open("names.txt", "r") as file:
        names = file.read().split("\n")
    return names


def passwordGen():
    passwords = []
    with open("password.txt", "r") as file:
        passwords = file.read().split()
    return passwords


def generate_rand_national_id(end):
    national_ids = []
    national_ids.append(
        "GHA-"
        + str(random.randint(111111111, 999999999))
        + "-"
        + str(random.randint(0, 9))
    )
    for _ in range(0, end):
        for national_id in national_ids:
            random_id = (
                "GHA-"
                + str(random.randint(111111111, 999999999))
                + "-"
                + str(random.randint(0, 9))
            )
            if national_id != random_id:
                national_ids.append(random_id)
                break
    return national_ids


def generate_rand_timestamps(end):
    # Get the start and end dates
    start_date = datetime.datetime(2022, 6, 10, 8, 0)
    datetime.datetime(2022, 6, 30, 16, 0)

    # Create a list of timestamps
    timestamps = []
    for i in range(end):
        current_date = start_date + datetime.timedelta(
            days=i,
            seconds=random.randint(0, 60),
            minutes=random.randint(0, 60),
            hours=random.randint(8, 15),
        )
        print(f"Type of {current_date.timestamp}")
        timestamp = datetime.datetime.fromtimestamp(
            int(current_date.timestamp())
        ).strftime("%Y-%m-%d %H:%M:%S")
        timestamps.append(timestamp)

    # Convert the timestamps to MySQL timestamp format
    mysql_timestamps = [str(timestamp) + ".000000" for timestamp in timestamps]

    # Print the timestamps in MySQL timestamp format
    return mysql_timestamps


def generate_rand_account_num(end):
    count = 0
    account_nums = []
    account_nums.append(random.randint(111111111111, 999999999999))
    for index in range(0, end):
        print(f"Index: {index}")
        for account_num in account_nums:
            random_id = random.randint(111111111111, 999999999999)
            if account_num != random_id:
                account_nums.append(random_id)
                print(f"Count: {count}")
                break
            count += 1
    return account_nums


def generate_rand_duration(end):
    duration = []
    duration.append(random.randint(1 * 60, 5 * 60))
    for _ in range(0, end):
        for service_time in duration:
            random_id = random.randint(1 * 60, 5 * 60)
            if service_time != random_id:
                duration.append(random_id)
                break
    return duration


def generate_rand_servers(end):
    server_ids = []
    server_ids.append(random.randint(11111111, 99999999))
    for index in range(0, end):
        print(f"Index: {index}")
        for account_num in server_ids:
            random_id = random.randint(11111111, 99999999)
            if account_num != random_id:
                server_ids.append(random_id)
                break
    return server_ids


names = namesGen()
passwords = passwordGen()
national_ids = generate_rand_national_id(100)
account_numbers = generate_rand_account_num(100)
timestamps = generate_rand_timestamps(100)
durations = generate_rand_duration(100)
server_ids = generate_rand_servers(3)
print(f"Server_ids: {server_ids}")


def generate_random_users(filename, headers):
    with open(filename, "w", newline="") as csvfile:
        writer = csv.writer(csvfile, delimiter=",")
        writer.writerow(headers)
        for i in range(100):
            name = names[i]
            account_nums = account_numbers[i]
            national_id = national_ids[i]
            password = passwords[i]
            row = [name, account_nums, national_id, password]
            writer.writerow(row)


def generate_random_transactions(filename, headers):
    with open(filename, "w", newline="") as csvfile:
        writer = csv.writer(csvfile, delimiter=",")
        writer.writerow(headers)
        for _ in range(100):
            transaction_detail = transaction_details[
                random.randint(0, len(transaction_details) - 1)
            ]
            server_id = server_ids[random.randint(0, len(server_ids) - 1)]
            national_id = national_ids[random.randint(0, len(national_ids) - 1)]
            duration = str(durations[random.randint(0, len(durations) - 1)])
            timestamp = timestamps[random.randint(0, len(timestamps) - 1)]
            row = [transaction_detail, server_id, national_id, duration, timestamp]
            writer.writerow(row)


def generate_random_tellers(filename, headers):
    with open(filename, "w", newline="") as csvfile:
        writer = csv.writer(csvfile, delimiter=",")
        writer.writerow(headers)
        for i in range(4):
            server_id = server_ids[i]
            station = i
            service_time = durations[random.randint(0, len(durations) - 1)]
            active = random.randint(0, 1)
            password = passwords[random.randint(0, len(passwords) - 1)]
            row = [server_id, station, service_time, active, password]
            writer.writerow(row)


generate_random_users(
    "clients.csv", ["name", "account_number", "national_id", "password"]
)

generate_random_tellers(
    "servers.csv", ["server_id", "station", "service_time", "active", "password"]
)

generate_random_transactions(
    "transactions.csv", ["detail", "server_id", "national_id", "duration", "timestamp"]
)
