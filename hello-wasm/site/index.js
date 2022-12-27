
const js = import("./node_modules/@jirepos/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});
