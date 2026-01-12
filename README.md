# Filamenty

Semestrálna práca z predmetu Jazyk Rust  
Fullstack webová aplikácia na správu filamentov do 3D tlačiarne

## Spustenie

Pre spustenie je potrebné mať nainštalované programy `git`, `docker` a `docker compose`

- Naklonujte tento repozitár do vami zvoleného priečinku
  - `git clone https://github.com/cyprich/filamenty.rs`
- Premenujte súbor `example.env` na `.env`
  - `mv example.env .env`
- Podľa potreby upravte premenné v súbore `.env`
  - `POSTGRES_USERNAME`, `POSTGRES_PASSWORD`, `POSTGRES_DB` - prihlasovacie údaje do databázy
  - `HOST` - IP adresa alebo doménové meno zariadenia, na ktorom bude bežať aplikácia
  - `BACKEND_PORT`, `FRONTEND_PORT` - sieťové porty backendu a frontendu
- Spustite aplikáciu príkazom `docker compose up -d` (prvé spustenie môže trvať niekoľko minút)
- Otvorte v prehliadači adresu [http://localhost](http://localhost), prípadne upravenú ak ste menili premenné v `.env`

Aplikácia bude bežať na pozadí, až kým nezadáte príkaz `docker compose down` v priečinku s naklonovaným repozitárom

API je zdokumentované pomocou OpenAPI dokumentácie, dostupné na adrese [http://localhost:5000/swagger-ui/](http://localhost:5000/swagger-ui/) (prípadne inej podľa zmeny premenných v `.env`)

Všetky podrobnosti nájdete v [dokumentácií](https://github.com/cyprich/filamenty-rs/blob/master/docs/dokumentacia.pdf)

---

**Upozornenie**

Tento repozitár obsahuje moje riešenia zadaní, prípadne semestrálnej práce  
Kód je zverejnený výhradne na vzdelávacie účely ako inšpirácia pre ostatných  
**Použitie tohto kódu na odovzdanie ako vlastné riešenie (plagiátorstvo) je prísne zakázané** a môže viesť k disciplinárnym opatreniam  
Ak si z tohto repozitára beriete inšpiráciu, uistite sa, že rozumiete riešeniam a vytvorte si vlastnú implementáciu  
Autor nenesie zodpovednosť za akékoľvek následky vyplývajúce z nesprávneho použitia tohto materiálu, vrátane prípadných obvinení z plagiátorstva

---
