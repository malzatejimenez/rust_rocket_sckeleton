# arranca docker compose
docker:
	docker-compose up -d

# genera un secret y lo copia en el porta papeles
gensecret:
	openssl rand -base64 32 | xclip -selection clipboard

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
	cargo add chrono --features serde
	cargo add serde --features derive
	cargo add serde_json
	cargo add --dev reqwest --features json,blocking 
	cargo add log
	cargo add clap
	cargo add dotenv
	cargo add argon2

# se corre la aplicacion asignando al variable de entorno de la base de datos
cargo-run:
	cargo run

# probar la ruta para crear un nuevo rustacean
create-rustacean:
	curl 127.0.0.1:8000/rustaceans -d '{"name":"John Doe", "email":"john@doe.com"}' -H 'Content-Type: application/json'

# probar la ruta para obtener el listado de rustaceans
get-rustaceans:
	curl 127.0.0.1:8000/rustaceans -H 'Content-Type: application/json'

# probar la ruta para obtener un rustacean específico
find-rustacean:
	curl 127.0.0.1:8000/rustaceans/9 -H 'Content-Type: application/json'

# probar la ruta para editar un rustacean específico
update-rustacean:
	curl 127.0.0.1:8000/rustaceans/9 -d '{"created_at":"2023-09-16T16:10:53.705456","email":"johnie@doe.com","id":1,"name":"Johnie Doe"}' -X PUT -H 'Content-Type: application/json'

# probar la ruta para eliminar un rustacean específico
delete-rustacean:
	curl 127.0.0.1:8000/rustaceans/1 -X DELETE -H 'Content-Type: application/json'

# probar la ruta para inicio de sesión
login:
	curl 127.0.0.1:8000/login -d '{"username":"admin", "password":"1234"}' -H 'Content-Type: application/json'

# probar la ruta para inicio de sesión erroneo
loginerr:
	curl 127.0.0.1:8000/login -d '{"username":"admin", "password":"12345"}' -H 'Content-Type: application/json'

# probar la ruta para inicio de sesión erroneo
loginusererr:
	curl 127.0.0.1:8000/login -d '{"username":"adminn", "password":"1234"}' -H 'Content-Type: application/json'



