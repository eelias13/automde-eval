import subprocess
import numpy as np

data = [[] for _ in range(9)] 
for _ in range(100):
    p = subprocess.Popen(["./target/debug/automode-eval"], stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    (stdout, stderr) = p.communicate()
    out:str = stdout.decode('utf-8')
    out = out.replace("]", "")
    out = out.replace("[","")
    out = out.split(", ")
    for i in range(len(out)):
        data[i].append(float(out[i].strip()))

max_val = []
min_val = []
for e in data:
    max_val.append(max(e)) 
    min_val.append(min(e))

print(f"max_val: {max_val}", f"min_val: {min_val}")

