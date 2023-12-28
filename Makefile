prog := my-pico-test

.PHONY: write probe-run

probe-run:
	RUSTFLAGS="-C link-arg=-Tdefmt.x -C link-arg=-Tlink.ld" cargo build --features probe-run
	probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/$(prog)

write:
	cargo build --release
	elf2uf2-rs -d target/thumbv6m-none-eabi/release/$(prog)
