.PHONY: test-all commit release doc peer hole_puncher

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
commit:
	make bench
	make test
	git add -A
	git commit

# build a release for all binaries
release:
	rm -rf target/release
	cargo build --release
	exec ./release.sh

# generate the documentation
doc:
	rm -rf target/doc
	cargo doc --all --frozen

# start the hole puncher
hole_puncher:
	clear
	cd hole_puncher; cargo run

# start a peer
peer:
	clear
	cd peer; cargo run

# start a peer in a docker container
peer_docker_run:
	make peer_docker_build
	docker run -it --net="host" blockchain_peer:latest

# run multiple peers in docker container
# number is determind by the script docker/start.sh
peer_docker_run_multi:
	make peer_docker_build
	./docker/start.sh

# only build the docker image
peer_docker_build:
	clear
	cd peer; cargo build
	cp target/debug/blockchain_peer docker/blockchain_peer
	cd docker; docker build -t blockchain_peer .

# remove all started peer containers
docker_clean:
	docker ps -aqf label=peer | xargs --no-run-if-empty docker rm -f 