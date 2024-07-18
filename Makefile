.PHONY: run-dev run-prove

INPUT_FILE := host/test_values/input.json

run-dev:
	RUST_LOG=info RISC0_DEV_MODE=1 cargo run -- $(INPUT_FILE)

run-prove:
	RUST_LOG=info RISC0_DEV_MODE=0 cargo run --release -- $(INPUT_FILE)
