import csv
server_ids = ["75373417", "88742759", "87365268", "55734681"]

with open("transactions.csv", "r") as csvfile:
    reader = csv.reader(csvfile, delimiter=",")
    for index, row in enumerate(reader):
        if (row[1] in server_ids) is False:
            print(f"{index}: Not Part {row[1]}")
