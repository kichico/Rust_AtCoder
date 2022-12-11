import os
from random import randint
with open("input.txt",mode="w") as f:
    num=[]
    for i in range(100):
        num.append(randint(0,1000000))
    for i in num:
        print(i)