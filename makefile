# Instala diesel_cli con soporte para PostgreSQL
diesel-cli:
	cargo install diesel_cli --no-default-features --features postgres

# Configura la base de datos para tu proyecto
diesel-setup:
	diesel setup --database-url postgres://postgres:postgres@localhost/app_db

# Ejecuta las migraciones de la base de datos
diesel-migrate:
	diesel migration run --database-url postgres://postgres:postgres@localhost/app_db

# Revierte la última migración de la base de datos
diesel-revert:
	diesel migration revert --database-url postgres://postgres:postgres@localhost/app_db

# instalar dependencias del proyecto
cargo-add:
	cargo add diesel --features postgres
	cargo add chrono