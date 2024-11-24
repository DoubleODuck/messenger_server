include .env

create_migrations:
	sqlx migrate add -r init

migrate_up:
	sqlx migrate run --database-url "postgres://${DB_USER}:${PASSWORD}@${HOST}:${DB_PORT}/${DB_NAME}?sslmode=disable"

migrate_down:
	sqlx migrate revert --database-url "postgres://${DB_USER}:${PASSWORD}@${HOST}:${DB_PORT}/${DB_NAME}?sslmode=disable"