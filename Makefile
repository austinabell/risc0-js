.PHONY: build_web_js
build_web_js:
	wasm-pack build --release --target web

.PHONY: build_node_js
build_node_js:
	wasm-pack build --release --target nodejs

.PHONY: test_js
test_js: build_node_js
	node --test js_tests
