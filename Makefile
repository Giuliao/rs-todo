.PHONY: database
run-database:
	docker run -d \
	--name postgres \
	-e POSTGRES_PASSWORD=123456 \
	-p 5432:5432 \
	-v ./tmp/pg_data:/var/lib/postgresql/data \
	postgres:latest

diesel-init:
	diesel setup
	diesel migration run