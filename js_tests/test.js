const test = require('node:test');
// const assert = require('node:assert').strict;
const fs = require('fs').promises;
const { SessionReceipt } = require("../pkg");

async function fetchBinaryFiles() {
	const receiptPromise = fs.readFile('./example/public/receipt.bin');
	const methodIdPromise = fs.readFile('./example/public/method_id.bin');

	return Promise.all([receiptPromise, methodIdPromise]);
}

test('example valid proof verification', () => {
	fetchBinaryFiles().then(([receipt, method_id]) => {
		SessionReceipt.bincode_deserialize(receipt).validate(method_id);
	});
});
