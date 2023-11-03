build:
	wasm-pack build --target nodejs --out-dir ../../dist/semsearch/nodejs && cp -avr script ../../dist/script
	wasm-pack build --target web --out-dir ./dist/semsearch/web
