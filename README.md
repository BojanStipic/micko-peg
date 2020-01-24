# micko-peg

miniC compiler implementation that uses
[PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) parser—[pest](https://github.com/pest-parser/pest).

pest is a general purpose parser written in Rust with a focus on accessibility, correctness, and performance.
It uses parsing expression grammars (or PEG) as input, which are similar in spirit to regular expressions,
but which offer the enhanced expressivity needed to parse complex languages.

miniC is a programming language created for educational purposes.
It is a strict subset of the C programming language.
miniC is used at the University of Novi Sad, Faculty of Technical Sciences, to teach the Compilers course.

## Prerequisites

* Rust compiler

## Installation

Compile and install with Cargo:
```bash
cargo build --release
cargo install --path .
```

## Usage

```bash
micko-peg < FILE
```

## License

    Copyright (C) 2020 Bojan Stipic

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
