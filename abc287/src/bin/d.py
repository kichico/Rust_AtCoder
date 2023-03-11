import re

s=input()
t=input()
for x in range(len(t)+1):
    ss=s[:x]+s[x+1:]
    flg=True
    for u,v in zip(ss,t):
        if u=='?' or v=='?' or u==v:
            continue
        else:
            print("No")
            flg=False
            break
    if flg:
        print("Yes")