n = 200
m = 200
printer = [[]]
for i in range(n):
    row = []
    for j in range(m):
        if j == 0 or j == m - 1:
            row.append("#")
        elif i == 0 or i == n - 1:
            row.append("#")
        else:
            row.append(".")
    printer.append(row)
    print(printer[-1])
with open("test.txt", mode="w") as f:
    for i in range(n):
        f.write("".join(printer[i]) + "\n")
