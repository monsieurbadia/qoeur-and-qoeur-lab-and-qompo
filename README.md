# qøeur

> *codify human thoughts into actions 🧠*

qoeur is a new experimental programming language inspired by retro-engineering from Erlang, Javascript and Rust.
## Questions

why the name `qoeur`?   
*`qoeur` is made by passionnates who do work with passion. `qoeur` is a mix between "<ins>code quantum</ins>" & "<ins>coeur</ins>" a french word which means* ❤️ 

what's the problem that we are trying to solve?   
*no problem, no reason, we just wanna learn*

why are we implementing this language?    
*have fun by learning!*

## Mindset

s/o to [Robert Virding](https://www.youtube.com/watch?v=afLRmoSOnHA) 👏👏

* focus, focus, FOCUS!
* semantics is king
* complexity never wins
* focus, FOCUS, FOCUS!!
* keep the language simple
* avoid providing alternate syntaxes for the same thing
* FOCUS, FOCUS, FOCUS!!!

## Goals

* no `gc`
* type safe
* elagant syntax
* small binaries size
* helpful error messages
* very fast compilation time
* compile: `llvm` | `asm` | `wasm`
* compile mode: `link` | `no-link`

## Overview

* [roadmap](./doc/roadmap.md)    
* [syntax](./doc/syntax.md)

## Installation

[Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) must be installed on your machine before.

```
> git clone https://github.com/qurity/qoeur.git
> cd qoeur
> cargo build
```

## Usage

use the `REPL` to play with the qoeur compiler. The `REPL` has different modes that let you specificy the output format.     
Run the command bellow with `args`

**read line**

output  | mode      | command                         | optional                   |
--------|-----------|---------------------------------|----------------------------|
tokens  | `-tokens` | `cargo run -repl -line -tokens` | ...                        |
ast     | `-ast`    | `cargo run -repl -line -ast`    | ...                        |
eval    | `-eval`   | `cargo run -repl -line -eval`   | ...                        |
js      | `-js`     | `cargo run -repl -line -js`     | `inline`, `json`, `pretty` |

**read file**

filename extension: `.q5`

output  | mode      | command                         | path               | optional                   |
--------|-----------|---------------------------------|--------------------|----------------------------|
tokens  | `-tokens` | `cargo run -repl -file -tokens` | `path/to/filename` | ...                        |
ast     | `-ast`    | `cargo run -repl -file -ast`    | `path/to/filename` | ...                        |
eval    | `-eval`   | `cargo run -repl -file -eval`   | `path/to/filename` | ...                        |
js      | `-js`     | `cargo run -repl -file -js`     | `path/to/filename` | `inline`, `json`, `pretty` |

## Testing

assertions has been implemented using [qutonium](https://github.com/qurity/qutonium), both will be groth together.

## References

[ast explorer](https://astexplorer.net)     

## <a name="license"></a> License   

Copyright ©️ 2020 Qurity    

Released under the [MIT](LICENSE) license    
