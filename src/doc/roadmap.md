## qoeur roadmap

features of qoeur.

### compiler

**core**

* [ ] `llvm` backend | *`needs help`*
* [ ] `assembly` backend | *`needs help`*
* [ ] `web assembly` backend | *`needs help`*
* [x] scanner
* [x] parser
* [] interpreter | `wip`
* [ ] reporter | *error messages a la `elm`*
* [ ] comment | *block, doc, line*
* [ ] primitives | *`int`, `float`, `bool`, `str`, `fn`*
* [ ] variable | *binding, immutable, lifetime tracking*
* [x] function | *calls, first-class, higher-order*
* [ ] control-flow | *`if`, `else if`, `else`*
* [ ] pattern matching | `match`
* [x] operator | *unary, binary*
* [ ] `loop` infinite
* [ ] `for in` loop
* [ ] `for range` loop
* [ ] `while` loop
* [ ] `nil`
* [ ] `return` values
* [ ] `use` import modules
* [ ] closures
* [ ] data-structures | *`array`, `hash`, `struct`, `tuple`*
* [ ] detect pure functions
* [ ] literal | *hexadecimal, octal, binary*
* [ ] multi-threading
* [ ] testing
* [ ] rewrite compiler
* [ ] ..

**linter**

* [ ] unused variables
* [ ] unused parameters
* [ ] unused `use`
* [ ] unmodified mutable variables
* [ ] ineffective assignments
* [ ] ..

**operators**

* [x] `+`, `-`, `*`, `/`, `=`
* [x] `+=`, `-=`, `*=`, `/=`, `==`, `!=`
* [x] `<`, `<<`, `<=`, `>=`, `>>`, `>`
* [ ] `&`, `|`, `&&`, `||`
* [ ] `%`
* [ ] ..

**optimizations**

* [ ] exclude unused functions
* [ ] function call inlining
* [ ] expression optimization
* [ ] ..

### extensions

* [ ] vscode extension
* [ ] `IDE`
* [ ] ..

### repl

* [x] cmd | *`copyright`, `help`, `license`, `repl`, `version`*
* [x] input | *`-file`, `-line`*
* [x] mode | *`-ast`, `-eval`, `-tokens`*
* [x] path | *optional path to the file*
