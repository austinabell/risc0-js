# risc0-js

## Building

```
wasm-pack build --target web
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
