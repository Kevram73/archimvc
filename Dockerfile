FROM rust:1.81 AS builder

WORKDIR /usr/src/app

# Copier les fichiers de dépendances
COPY Cargo.toml Cargo.lock* ./

# Créer une structure minimale pour la compilation des dépendances
RUN mkdir src && \
    echo 'fn main() { println!("Dummy") }' > src/main.rs && \
    cargo build --release && \
    rm src/main.rs

# Copier le vrai code source
COPY src src/

# Compiler l'application finale
RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/archimvc /usr/local/bin/

EXPOSE 8080

CMD ["archimvc"]
