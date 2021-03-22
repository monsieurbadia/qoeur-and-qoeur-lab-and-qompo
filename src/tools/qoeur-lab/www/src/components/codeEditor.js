export function initCodeEditor (option) {
  class CodeEditor {
    constructor () {
      this.state = {
        compilerView: initCodeEditorView(document.getElementById("code-compiler-view"), "rust", ""),
        editorView: initCodeEditorView(document.getElementById("code-editor-view"), "javascript", ""),
      };

      this.compilerViewValue = "";
      this.editorViewValue = "";
    }

    get compilerView () {
      return this.state.compilerView;
    }

    set compilerView (payload) {
      this.state.compilerView = payload;
    }

    get editorView () {
      return this.state.editorView;
    }

    set editorView (payload) {
      this.state.editorView = payload;
    }

    get compilerViewValue () {
      return this.state.compilerView.getValue();
    }

    set compilerViewValue (payload) {
      this.state.compilerView.setValue(payload);
    }

    get editorViewValue () {
      return this.state.editorView.getValue();
    }

    set editorViewValue (payload) {
      this.state.editorView.setValue(payload);
    }

    init () {
      return this;
    }
  }
 
  return new CodeEditor().init();
}

function initCodeEditorView (input, mode, value) {
  return CodeMirror.fromTextArea(
    input,
    {
      lineNumbers: true,
      mode,
      value,
    }
  );
}
