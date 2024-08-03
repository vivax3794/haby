test_all: test_unit test_int

fmt:
    leptosfmt haby_frontend_core/src
    cargo fmt

test_unit: db
    cargo nextest run --cargo-quiet --cargo-quiet

test_int: spawn_server
    cargo nextest run --test integration_tests -j 1 --fail-fast --cargo-quiet --cargo-quiet

dev_frontend: spawn_server
    cd haby_frontend && trunk serve --watch .. --port 3000

spawn_server: build_server
    @docker compose down server > /dev/null 2>&1
    docker compose up server -d --wait 

build_server: db
    @cd haby_server && cargo sqlx prepare > /dev/null 2>&1
    docker compose build server -q 

db:
    docker compose up db -d --wait 
    @docker compose exec db psql -U postgres -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;" > /dev/null 2>&1
    cd haby_server && sqlx migrate run

down:
    docker compose down
