import pickle

filename = 'betalinger.pk'

fodboldtur = {}
beløb = 4500


def afslut():
    outfile = open(filename, 'wb')
    pickle.dump(fodboldtur, outfile)
    outfile.close()
    print("Programmet er afsluttet!")


def printliste():
    for item in fodboldtur.items():
        print(f"{item[0]} har betalt: {item[1]}kr (mangler {beløb - item[1]}kr)")
    menu()


def tre_mindste():
    for b, n in sorted([(v, k) for k, v in fodboldtur.items()])[:3]:
        print(f"{n} har kun betalt: {b}kr (mangler {beløb - b}kr)")
    menu()


def registrer_betaling():
    betaling = input("Indtast betaling (Syntax: {navn} {beløb})\n(hint: brug _ istedet for mellemrum i navne)\n") # -> "Ib_Ibsen 500"
    betaling = betaling.split(" ") # -> ["Ib_Ibsen", "500"]
    betaling[0] = betaling[0].replace("_", " ") # -> ["Ib Ibsen", "500"]
    if betaling[0] in fodboldtur:
        try:
            fodboldtur[betaling[0]] += float(betaling[1])
            print(f"Betaling registretert.\n{betaling[0]} har nu betalt {fodboldtur[betaling[0]]}kr i alt (mangler {beløb - fodboldtur[betaling[0]]})")
        except:
            print("ERR: beløb er ikke et tal")
    else:
        print("ERR: navn ikke registreret")
    menu()


def tilføj_person():
    person = input("Indtast navn:\n")
    if person not in fodboldtur:
        fodboldtur[person] = 0
        print(f"{person} tilføjet!")
    else:
        print("ERR: Navn findes allerede")
    menu()


def menu():
    print("\nMENU")
    print("1: Print liste")
    print("2: Find de 3 der har betalt mindst")
    print("3: Registrer betaling")
    print("4: Tilføj person")
    print("5: Gem & Afslut program")
    valg = input("Indtast dit valg:")
    # Skiftede til match cases, fordi de bare er bedre
    match valg:
        case '1':
            printliste()
        case '2':
            tre_mindste()
        case '3':
            registrer_betaling()
        case '4':
            tilføj_person()
        case '5':
            afslut()
    print("\n")


infile = open(filename, 'rb')
fodboldtur = pickle.load(infile)
infile.close()
menu()
