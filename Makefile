.PHONY: run-dev run-prove

run-dev:
	RUST_LOG=info RISC0_DEV_MODE=1 cargo run -- host/test_values/input_20240717_130851.json

run-prove:
	RUST_LOG=info RISC0_DEV_MODE=0 cargo run --release -- host/test_values/input_20240717_130851.json
