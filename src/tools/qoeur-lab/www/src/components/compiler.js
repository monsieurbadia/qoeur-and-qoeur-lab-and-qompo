const MODE = Object.freeze(["ast", "custom", "eval", "js", "js-ast", "tokens"]);

const SOURCE = Object.freeze({
  "custom": "",
  "function": `ƒ sqrt (x : int) -> int {\n\tx * x\n}\nsqrt(9);`,
  "if": `if true { 1 } else { 0 }`,
  "array": `val a : [] = [1, 2, 3];\na[2];`,
  "hash": `val a : hash = { "firstname": "john", "lastname": "doe" };\na["firstname"];`,
  "unary": `val a : bool = !true;\nval b : bool = !0;\na == b;`,
  "int": `val a : int = 2;\nval b : int = 2;\nval c : int = a + b;\nc;`,
  "big-int": `val a : int = 1_000_000_000;\nval b : int = 1_000_000;\nval c : int = a + b;\nc;`,
  "float": `val a : int = 1.234;\nval b : int = 1e4;\nval c : int = a + b;\nc;`,
  "str": `val a : str = "abc";\nval b : str = "def";\nval c : str = a + b;\nc;`,
});

const COMPILER_STATE = Object.freeze({
  currentExample: "int",
  currentMode: "eval",
  currentSource: SOURCE["int"],
});

export function initCompiler ({
  codeEditor,
  wasm_astify,
  wasm_evalify,
  wasm_tokenify,
  wasm_transformify,
}) {
  class Compiler {
    constructor () {
      this.state = {
        button: {
          compilerBtnRun: document.getElementById("terminal-controls-btn-run-compile"),
          clipboardBtn: document.getElementById("clipboard-btn"),
        },
        select: {
          compilerExample: document.getElementById("terminal-controls-example"),
          compilerMode: document.getElementById("terminal-controls-mode"),
          compilerOptional: document.getElementById("terminal-controls-optional"),
        }
      }
    }

    get button () {
      return this.state.button;
    }

    set button (payload) {
      this.state.button = payload;
    }

    get select () {
      return this.state.select;
    }

    set select (payload) {
      this.state.select = payload;
    }

    compile (payload) {
      switch (this.select.compilerMode.value.trim()) {
        case "ast":
          codeEditor.compilerViewValue = wasm_astify(payload);
          break;
        case "js":
          codeEditor.compilerViewValue = wasm_transformify(payload, "inline");
          break;
        case "js-ast":
          codeEditor.compilerViewValue = wasm_transformify(payload, "json");
          break;
        case "tokens":
          codeEditor.compilerViewValue = wasm_tokenify(payload);
          break;
        default:
          codeEditor.compilerViewValue = wasm_evalify(payload);
          return;
      }
    }

    init () {
      Object.keys(SOURCE).forEach((key) => {
        const sourceCode = SOURCE[key];

        const option = document.createElement("option");
        option.innerHTML = sourceCode;
        option.label = key;
        option.value = key;
      
        this.select.compilerExample.appendChild(option);
      });
      
      [...MODE].forEach((kind) => {
        const option = document.createElement("option");
        option.innerHTML = kind;
        option.value = kind;
        option.label = kind;
      
        this.select.compilerMode.appendChild(option);
      });
    
      this.button.compilerBtnRun.addEventListener("click", (event) => {
        event.preventDefault();    
        this.compile(codeEditor.editorViewValue);
      });

      this.button.clipboardBtn.addEventListener("click", (event) => {
        event.preventDefault();
        this.toClipboard();
      });

      this.select.compilerExample.addEventListener("change", (event) => {
        event.preventDefault();

        console.log(this.compilerExample);

        codeEditor.editorViewValue = event.target.options[event.target.selectedIndex].textContent;
      });

      codeEditor.editorViewValue = COMPILER_STATE.currentSource;
      codeEditor.compilerViewValue = wasm_evalify(COMPILER_STATE.currentSource);
      this.select.compilerExample.value = COMPILER_STATE.currentExample;
      this.select.compilerMode.value = COMPILER_STATE.currentMode;

      return this;
    }

    toClipboard () {
      navigator.clipboard.writeText(codeEditor.compilerViewValue);
    }
  }

  return new Compiler().init();
}
