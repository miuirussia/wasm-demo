import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import * as wasm from "wasm-pkg";

console.time('wasm init');
console.log(wasm.parse('{ "x": 10, "y": 2 }'));
console.timeEnd('wasm init');

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
