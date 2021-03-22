import * as wasm from "qoeur-lab";
import { initCodeEditor } from './components/codeEditor';
import { initCompiler } from './components/compiler';

async function run() {
  const codeEditor = initCodeEditor(wasm);
  const compiler = initCompiler({
    codeEditor,
    ...wasm
   });
}

run();
