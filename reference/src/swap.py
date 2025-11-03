def swap(a, b):
    a, b = b, a
    return a, b


x = 10
y = 20
print(x, y)
x, y = swap(x, y)
print(x, y)  # output: 20 10
