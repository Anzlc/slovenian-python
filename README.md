# Slovenian Python (Piton) 🐍

A simple python transpiler written in Rust.
It introduces Slovenian translations for keywords and some built-in function.

## Keywords

```py
| Slovenian   | English   |
|-------------|-----------|
| definiraj   | def       |
| za          | for       |
| v           | in        |
| dokler      | while     |
| vključi     | import    |
| iz          | from      |
| vrni        | return    |
| natisni     | print     |
| obseg       | range     |
| kot         | as        |
| vnos        | input     |
| če          | if        |
| drugače     | else      |
| drugačeče   | elif      |
| in          | and       |
| ali         | or        |
| razred      | class     |
| preveri     | assert    |
| zlomi       | break     |
| nadaljuj    | continue  |
| izbriši     | del       |
| razen       | except    |
| Ne          | False     |
| Da          | True      |
| končno      | finally   |
| globalno    | global    |
| je          | is        |
| Nič         | None      |
| nelokalen   | nonlocal  |
| ne          | not       |
| brez        | pass      |
| dvigni      | raise     |
| poskusi     | try       |
| z           | with      |
| dajaj       | yield     |
```

## How does it work

I built a simple tokenizer that splits keywords while keeping parts like string intact. Then I loop through all keywords and translate them using a hashmap.

## How to use

If you want to use this program it is very simple download the .exe from Releases tab and if you want to use it anywhere add the file to Path.
Then you can simply run

```bash
piton [PATH]
```

Other options:

```
-t, --transpile PATH
                        Create a python file from .spy
-r, --run PATH      Run the .spy program
-p, --preview PATH  Preview transpiled .spy program
-h, --help          Get help (output this screen)
```

## Example

```
vključi random kot r

ugib = int(vnos("Vnesi št od 1-10: "))

naključna = r.randrange(1, 10)

dokler ugib != naključna:
    če ugib > naključna:
        natisni("Preveč")
    drugače:
        natisni("Premalo")
    ugib = int(vnos("Vnesi št od 1-10: "))
natisni(f"Bravo, naključna št je bila {naključna}")
```

## Contributing

If you find any bugs or issues open an Issue or make a pr.
