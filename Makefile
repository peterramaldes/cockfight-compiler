build:
	@cargo build --release

clean:
	@rm -rf ./target

run: build
	@./target/release/cfc $(file)

docker:
	@docker build -t peterramaldes/cfc .

docker-down:
	@docker compose down -v --remove-orphans

docker-push:
	@docker buildx build --push --tag peterramaldes/cfc .

docker-dev: docker-down
	@docker compose -f docker-compose.dev.yml up --build 
