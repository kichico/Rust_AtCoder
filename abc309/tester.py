import random

n = int(8e4)
m = 4000
print(f"{n} {m}")
par = []
for i in range(n - 1):
    par.append(str(random.randrange(1, n - 1)))
print(" ".join(par))
for i in range(m):
    print(f"{random.randrange(1,n)} {random.randrange(1,n)}")
