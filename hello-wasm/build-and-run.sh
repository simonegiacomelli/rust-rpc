wasm-pack build
cd www
export NODE_OPTIONS=--openssl-legacy-provider
npm run start
cd ..