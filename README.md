# risc0-js

## Building

```bash
wasm-pack build --target web

# OR

make build_web_js
```

## Running example

```bash
# Go in the example directory to build
cd example
npm install

# This command will build the program and embed the proof data in the frontend public dir
# before starting the frontend.
npm start

# Run this anytime to re-generate the proof while frontend is running
npm run build-program
```

## Testing

```bash
# Test with chrome or any other wasm-pack test target
wasm-pack test --chrome

# Run nodejs tests
make test_js
```

## Usage

```ts
import init, { SessionReceipt } from "risc0-js";

init().then(() => {
	try {
		// Deserialize encoded receipt directly
		SessionReceipt.bincode_deserialize(receipt).validate(method_id);
	} catch (e) {
		// ...
	}
});

```
