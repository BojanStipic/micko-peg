// micko-peg - miniC compiler implementation that uses PEG parser
// Copyright (C) 2020 Bojan Stipic
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use bitflags::bitflags;
use std::str::FromStr;

bitflags! {
    pub struct Kinds: u32 {
        const REG = 0b00001;
        const FUN = 0b00010;
        const PAR = 0b00100;
        const VAR = 0b01000;
        const LIT = 0b10000;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Unsigned,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "int" => Ok(Self::Int),
            "unsigned" => Ok(Self::Unsigned),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub kind: Kinds,
    pub s_type: Type,
    pub ordinal: u32,
}

/// Symbol table.
#[derive(Debug)]
pub struct Symbols(Vec<Symbol>);

impl Symbols {
    /// Constructs a new, empty symbol table.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Appends a symbol to the symbol table.
    pub fn push(&mut self, symbol: Symbol) -> &Symbol {
        self.0.push(symbol);
        self.0.last().unwrap()
    }

    /// Search for a symbol with specified `name` and `kind`.
    /// Returns `None` if there is no such symbol in the symbol table.
    pub fn lookup(&self, name: &str, kind: Kinds) -> Option<&Symbol> {
        self.0
            .iter()
            .rev()
            .find(|symbol| symbol.name == name && symbol.kind.intersects(kind))
    }

    /// Returns the number of elements in the symbol table.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Shortens the symbol table, keeping the first `len` elements and dropping the rest.
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }
}
