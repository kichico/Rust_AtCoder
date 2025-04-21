import random

n = random.randint(1, 10)
m = random.randint(1, 10)
a = []
b = []
for i in range(n):
    a.append(str(random.randint(100, 10000)))
for i in range(m):
    b.append(str(random.randint(100, 10000)))

print(n, m)
print(" ".join(a))
print(" ".join(b))
