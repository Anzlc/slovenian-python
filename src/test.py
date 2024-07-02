import time as t

ime = input("natisni svoje ime: ")
svet = "svet"
print(f"Živo, {ime}")
print(svet)

for i in range(10, 0, -1):
    print(f"{i}!")
    t.sleep(.2)

print("Vzlet")