import time as t

ime = input("Vnesi natisni svoje ime: ")
print(f"Živo, {ime}")

for i in range(10, 0, -1):
    print(f"{i}!")
    t.sleep(.2)

print("Vzlet")