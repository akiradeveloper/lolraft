docker-build:
	docker build -t lol:dev --build-arg USER=${USER} --build-arg UID=`id -u` - < Dockerfile

.PHONY: install
install:
	cargo install --path kvs --bin kvs-server
	cargo install --path kvs --bin kvs-client
	cargo install --path lol-admin
	cargo install --path lol-monitor

test: install
	cargo test -- --test-threads=1

bench: install
	cargo +nightly bench