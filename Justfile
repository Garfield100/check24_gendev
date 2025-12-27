_:
    @just --list

run_valkey:
    docker compose up

run_home:
    cd clients/web && just build
    cd services/home_backend/inbound/ && cargo run -r