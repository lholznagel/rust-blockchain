# just build the project
default:
	cargo build --features dev

# run the benchmarks
bench:
	rustup run nightly cargo bench

# run test with stable and nightly
test:
	rustup run stable cargo test
	rustup run nightly cargo test

# check that everything works before commiting
commit: bench test
	git add -p
	git commit

# build a release for all binaries
release:
	rm -rf target/release
	cargo build --release
	exec ./scripts/release.sh

# generate the documentation
doc:
	rm -rf target/doc
	cargo doc --all --frozen

# start a peer
peer:
	clear
	cd peer_cli; RUST_BACKTRACE=1 cargo run console

# start a peer in a docker container
peer_docker_run: peer_docker_build
	docker run -it --net="host" --label peer carina_peer:latest

# only build the docker image
peer_docker_build:
	clear
	cd peer; cargo build
	cp target/debug/carina_peer_cli docker/carina_peer
	cd docker; docker build -t carina_peer .

# remove all started peer containers
docker_clean:
	docker ps -aqf label=peer | xargs --no-run-if-empty docker rm -f 