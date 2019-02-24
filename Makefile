APP_ARGS := 42

run:
	cargo run $(APP_ARGS)

test:
	./test.sh

check:
	cargo check $(OPTION)

clean:
	cargo clean
