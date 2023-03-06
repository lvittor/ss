import random

n = 100
l = 100
m = 8
rc = 10

print(n)
print(l)
print(m)
print(rc)

for i in range(n):
    print(i, random.uniform(0, l), random.uniform(0, l), random.uniform(1, 5))
