dev:
	cargo run --release
	./target/release/cfc

build:
	cargo build --release

clean:
	rm -rf ./target

docker:
	docker build -t peterramaldes/cfc .

docker-push:
	docker buildx build --push --tag peterramaldes/cfc .
