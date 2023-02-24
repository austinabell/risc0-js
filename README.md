# risc0-js

## Building

```
wasm-pack build --target bundler
```

## Running example

```
cd example
npm start
```

## Usage

```ts
try {
	new Receipt(journal, sealBytes).validate(id);
} catch (e) {
	// ...
}

```

## Converting receipt and ID to base64

TODO this opinionated and might change, but the base64 was generated with the following code on a r0 program to use with the frontend:

```rust
let jb64: String = general_purpose::STANDARD_NO_PAD.encode(&receipt.journal);
let seal_u8: &[u8] = bytemuck::cast_slice(&receipt.seal);
let sb64: String = general_purpose::STANDARD_NO_PAD.encode(seal_u8);
let method_id: &[u8] = bytemuck::cast_slice(&METHOD_NAME_ID);
let ib64: String = general_purpose::STANDARD_NO_PAD.encode(method_id);
// let slice: &
println!("journal: {}", jb64);
println!("seal: {}", sb64);
println!("id: {}", ib64);
```