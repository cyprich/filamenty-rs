// #set document(title: "cyprich/filamenty-rs")
#set text(font: "Liberation Sans")
#set page(paper: "a4")

#show link: set text(fill: blue )
#show link: underline

// title page

#include "title.typ"

#set page(numbering: "1")

// header 

#set page(
  header: [
    #grid(
        columns: (1fr, 1fr), 
        align(left)[
            // #image("FRI_H_S_sk_f.png")
        ],
        align(right)[
            _Jazyk Rust_\
            Peter Cyprich
        ]
    )
  ]
)


// table of contents



// main content

= Popis aplikacie 

= UML diagram tried 

= Pouzite kniznice

== Crate `app`

- `actix-web`
- `actix-cors`
- `actix-files`

== Crate `lib` 

- `chrono`, features `serde`
- `dotenvy`
- `serde`, features `derive`
- `sqlx`, features `runtime-tokio`, `postgres`, `chrono`

= Pouzivatelska prirucka 

== Spustenie aplikacie 

=== Docker compose 

Poziadavky: #link("https://www.docker.com/get-started/")[docker], #link("https://docs.docker.com/compose/")[docker-compose]

=== Manualne spustenie 

Poziadavky: #link("https://rust-lang.org/tools/install/")[rust], #link("https://www.npmjs.com/")[npm]

== Pouzivanie aplikacie 

= Programatorska prirucka 

== Popis API

== Popis databazy 

= Pouzite zdroje

= Zaver 