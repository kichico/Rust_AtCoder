import os

for i in range(100):
    s = str(i).zfill(4)
    os.system("cargo run -r --bin vis ../input/" + s + ".txt ../output/" + s + ".txt")
