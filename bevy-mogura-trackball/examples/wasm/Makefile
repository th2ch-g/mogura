EXAMPLES := exponential_map gliding_clamp constellation_clamp egui

.PHONY: help
help:
	@printf "make [help]     Prints this help.\n"
	@printf "make install    Installs required tools except rustup and cargo.\n"
	@printf "make wasm [-j]  Builds optimized WASM examples.\n"
	@printf "make html [-j]  Generates HTML for examples.\n"
	@printf "make page [-j]  Makes all (page: wasm html).\n"
	@printf "make host [-j]  Runs local HTTP server (host: page).\n"
	@printf "make clean      Removes ./target directory.\n"

.PHONY: install
install:
	command -v apt && sudo apt install binaryen jq || true
	command -v dnf && sudo dnf install binaryen jq || true
	command -v yum && sudo yum install binaryen jq || true
	command -v pacman && sudo pacman -S --needed binaryen jq || true
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli basic-http-server

.PHONY: wasm
wasm: $(EXAMPLES:%=target/%.wasm)
.PHONY: html
html: $(EXAMPLES:%=target/%.html) target/index.html
.PHONY: page
page: wasm html

target:
	mkdir target
	cp -a -- woff woff2 *.png *.css *.js $@

.PHONY: update
udpate:;

target/metadata: ../../Cargo.toml | target
	cargo metadata --no-deps --format-version 1 > $@

target/%.features: target/metadata
	cat $< | jq -r ".packages[] | select(.name==\"bevy_trackball\") | .targets[] | \
	select(.name==\"$*\") | .\"required-features\" | join(\",\")" > $@

../../target/wasm32-unknown-unknown/wasm-release/examples/%.wasm: target/%.features update
	@printf "    \033[1;32mUpdating\033[0m $*(example)\n"
	cargo build --profile wasm-release --target wasm32-unknown-unknown -F $$(cat $<), --example $*

target/%_bg.wasm: ../../target/wasm32-unknown-unknown/wasm-release/examples/%.wasm
	@printf "     \033[1;32mBinding\033[0m $*(example)\n"
	wasm-bindgen --target web --out-dir target --out-name $* $<

target/%.wasm: target/%_bg.wasm
	@printf "  \033[1;32mOptimizing\033[0m $*(example)\n"
	mv $< $@
	wasm-opt -Oz --output $< $@
	touch $@

target/index.html: | target
	ln -sf $(word 1,$(EXAMPLES)).html target/index.html

target/input.code: ../../src/lib.rs | target
	sed -n '/| Keyboard |/,/| `Enter`  |/p' $< | jq -Rr @html > $@

target/%.code: ../%.rs target/input.code
	cat target/input.code > $@
	printf '%s\n' '//!' >> $@
	jq -Rr @html $< >> $@

target/%.html: target/%.code example.html
	sed -e 's/{EXAMPLE}/'$*'/g' -e '/{SOURCE}/{r '$< -e 'd}' example.html > $@

.PHONY: host
host: wasm html
	@printf "     \033[1;32mServing\033[0m examples\n"
	basic-http-server target

.PHONY: clean
clean:
	rm -rf -- target

.SECONDARY:
Makefile:;
