format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

d-up:
	docker-compose up --build -d

all: format lint test run
