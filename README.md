﻿# Fodboldtur

## Udgangspunkt

Der var allerede kode givet i starten, som vi kunne bygge videre på.
Koden inkluderede allerede flere features, som kan vises på dette flowchart

![første flowchart](Images/InitialFlowChart.jpg)


## Viderudvikling

Efter at have videre udviklet på `fodboldtur.py`, kan man opstille et nyt flowchart over dens funktionalitet:

![Andet flowchart](Images/after.jpg)


## Rust version

Jeg er lidt fan af Rust, så jeg valgte at lave en rust version (efter jeg havde lavet `fodboldtur.py` færdig)
Denne version findes i `/payment_cli/` mappen.
Du kan enten vælge at se koden igennem, eller bare kører `payment_cli.exe` i terminalen ved kommandoen
```powershell
.\[path to payment_cli.exe]
```
Denne kommando kan køres fra alle paths, men der bliver lavet en `data.json` fil i den mappe du kalder den fra.

Rust versionen er dog ikke feature-complete på nogen måde, har kun få kommandoer.
