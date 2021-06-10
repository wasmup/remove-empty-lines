all:
	RUSTFLAGS='-C link-arg=-s' cargo build --release