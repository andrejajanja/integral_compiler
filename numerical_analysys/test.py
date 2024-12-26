n = 8
x = 1.1

pows = []
current_pow = 1.0

for i in range(n):
    current_pow *= x
    pows.append(current_pow)

print(pows)