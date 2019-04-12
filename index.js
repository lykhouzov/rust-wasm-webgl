document.getElementById('canvas').setAttribute('width', document.documentElement.clientWidth);
document.getElementById('canvas').setAttribute('height', document.documentElement.clientHeight);
import('./pkg/rust_wasm_webgl')
  .catch(console.error);