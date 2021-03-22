import * as wasm from "../pkg";
import * as path from 'path';

const SOURCE = `${path.resolve(__dirname, '..')}example.qmp`;

function compileToWasm (source) {
  return wasm.WebCompiler.new(source).compile();
}

function responseToText (response) {
  return response.text();
}

function throwError (error) {
  throw new Error(error);
}

export async function compiletoWasm () {
  fetch(SOURCE)
    .then(responseToText)
    .then(compileToWasm)
    .catch(throwError)
};

compiletoWasm();
