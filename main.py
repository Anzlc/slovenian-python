import random as r

ugib = int(input("Vnesi št od 1-10: "))

naključna = r.randrange(1, 10)

while ugib != naključna:
    if ugib > naključna:
        print("Preveč")
    else:
        print("Premalo")
    ugib = int(input("Vnesi št od 1-10: "))
print(f"Bravo, naključna št je bila {naključna}")