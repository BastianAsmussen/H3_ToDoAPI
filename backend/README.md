# H3 ToDo API

Dette er en simpel web API til at håndtere opgaver. API'et er skrevet i Rust.

## Indholdsfortegnelse

- [Installation](#installation)
    - [Start en lokal PostgreSQL-database](#start-en-lokal-postgresql-database)
    - [Kør projektet](#kør-projektet)
- [API](#api)
- [Eksempel](#eksempel)
    - [Hent alle opgaver](#hent-alle-opgaver)
    - [Hent en opgave efter ID](#hent-en-opgave-efter-id)
    - [Hent en opgave efter status](#hent-en-opgave-efter-status)
    - [Opret en opgave](#opret-en-opgave)
    - [Opdater en opgave](#opdater-en-opgave)
    - [Slet en opgave](#slet-en-opgave)

## Installation

For at køre dette projekt, skal du have Rust og Cargo installeret. Du kan finde installere
det [her](https://rustup.rs/).

Når du har installeret Rust og Cargo, kan du køre følgende kommando for at køre projektet lokalt:

### Start en lokal PostgreSQL-database
```bash
echo "DATABASE_URL=postgres://username:password@localhost/dbname" > .env

cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```

### Kør projektet
```bash
cargo run
```

## API

| Metode | Ressource                 | Beskrivelse        |
|--------|---------------------------|--------------------|
| GET    | /todos/all                | Hent alle opgaver. |
| GET    | /todos/by_id/{id}         | Hent en opgave.    |
| GET    | /todos/by_status/{status} | Hent en opgave.    |
| POST   | /todos/new                | Opret en opgave.   |
| PUT    | /todos/update/{id}        | Opdater en opgave. |
| DELETE | /todos/delete/{id}        | Slet en opgave.    |

## Eksempel

### Hent alle opgaver

```bash
curl http://localhost:8000/todos/all
```

### Hent en opgave efter ID

```bash
curl http://localhost:8000/todos/by_id/1
```

### Hent en opgave efter status

#### Hent alle fuldførte opgaver

```bash
curl http://localhost:8000/todos/by_status/complete
```

#### Hent alle ufuldendte opgaver

```bash
curl http://localhost:8000/todos/by_status/incomplete
```

### Opret en opgave

#### Opret en ufuldendt opgave

```bash
curl -X POST -H "Content-Type: application/json" -d '{"title": "Køb mælk"}' http://localhost:8000/todos/new
```

#### Opret en fuldført opgave

```bash
curl -X POST -H "Content-Type: application/json" -d '{"title": "Køb mælk", "completed": true}' http://localhost:8000/todos/new
```

### Opdater en opgave

#### Opdater en opgave

```bash
curl -X PUT -H "Content-Type: application/json" -d '{"title": "Køb mælk og brød", "completed": true}' http://localhost:8000/todos/update/1
```

### Slet en opgave

```bash
curl -X DELETE http://localhost:8000/todos/delete/1
```
