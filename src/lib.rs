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

mod symtab;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use symtab::{Kinds, Symbol, Symbols, Type};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Syntax error: {0}")]
    Syntax(#[from] pest::error::Error<Rule>),
    #[error("Semantic error: {0}")]
    Semantic(String),
}
type Result<T> = std::result::Result<T, Error>;

#[derive(pest_derive::Parser)]
#[grammar = "minic.pest"]
struct MCParser;

pub fn run(input: &str) -> Result<()> {
    let program = MCParser::parse(Rule::program, input)?;
    Micko::new().code_gen(program)
}

struct Micko {
    symbols: Symbols,
    ordinal: u32,
}

impl Micko {
    fn new() -> Self {
        Self {
            symbols: Symbols::new(),
            ordinal: 1,
        }
    }

    fn code_gen(&mut self, program: Pairs<Rule>) -> Result<()> {
        for part in program {
            match part.as_rule() {
                Rule::function => self.function(part)?,
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }

        if self.symbols.lookup("main", Kinds::FUN).is_none() {
            Err(Error::Semantic(String::from(
                "undefined reference to `main`",
            )))
        } else {
            Ok(())
        }
    }

    fn function(&mut self, function: Pair<Rule>) -> Result<()> {
        let symbols_len = self.symbols.len();

        for part in function.into_inner() {
            match part.as_rule() {
                Rule::id_decl => self.id_decl(part, Kinds::FUN, 0)?,
                Rule::parameter => self.id_decl(part, Kinds::PAR, 1)?,
                Rule::body => self.body(part)?,
                _ => unreachable!(),
            }
        }

        self.symbols.truncate(symbols_len + 1);
        self.ordinal = 1;
        Ok(())
    }

    fn id_decl(&mut self, decl: Pair<Rule>, kind: Kinds, ordinal: u32) -> Result<()> {
        let mut parts = decl.into_inner();
        let s_type = parts.next().unwrap().as_str().parse().unwrap();
        let name = String::from(parts.next().unwrap().as_str());

        if self.symbols.lookup(&name, kind).is_some() {
            return Err(Error::Semantic(format!("redefinition of `{}`", name)));
        }
        self.symbols.push(Symbol {
            name,
            kind,
            s_type,
            ordinal,
        });
        Ok(())
    }

    fn body(&mut self, body: Pair<Rule>) -> Result<()> {
        for part in body.into_inner() {
            match part.as_rule() {
                Rule::variable => self.variable(part)?,
                Rule::compound_statement => {}
                Rule::assignment_statement => self.assignment_statement(part)?,
                Rule::if_statement => {}
                Rule::return_statement => {}
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn variable(&mut self, variable: Pair<Rule>) -> Result<()> {
        let decl = variable.into_inner().next().unwrap();
        self.id_decl(decl, Kinds::VAR, self.ordinal)?;
        self.ordinal += 1;
        Ok(())
    }

    fn assignment_statement(&mut self, assignment: Pair<Rule>) -> Result<()> {
        let mut parts = assignment.into_inner();
        let lhs = self.id(parts.next().unwrap(), Kinds::VAR | Kinds::PAR)?;
        let rhs = self.num_exp(parts.next().unwrap())?;
        if lhs.s_type != rhs.s_type {
            return Err(Error::Semantic(String::from("incompatible types")));
        }
        Ok(())
    }

    fn num_exp(&mut self, num_exp: Pair<Rule>) -> Result<Symbol> {
        let mut parts = num_exp.into_inner();
        let first_exp = self.exp(parts.next().unwrap())?;

        for part in parts {
            match part.as_rule() {
                Rule::exp => {
                    let exp = self.exp(part)?;
                    if first_exp.s_type != exp.s_type {
                        return Err(Error::Semantic(String::from("incompatible types")));
                    }
                }
                Rule::arop => {}
                _ => unreachable!(),
            }
        }

        Ok(first_exp)
    }

    fn exp(&mut self, exp: Pair<Rule>) -> Result<Symbol> {
        let exp = exp.into_inner().next().unwrap();
        match exp.as_rule() {
            Rule::function_call => self.function_call(exp),
            Rule::literal => Ok(self.literal(exp)),
            Rule::id => self.id(exp, Kinds::VAR | Kinds::PAR),
            Rule::num_exp => self.num_exp(exp),
            _ => unreachable!(),
        }
    }

    fn id(&self, id: Pair<Rule>, kind: Kinds) -> Result<Symbol> {
        self.symbols
            .lookup(id.as_str(), kind)
            .cloned()
            .ok_or_else(|| Error::Semantic(format!("`{}` undeclared", id.as_str())))
    }

    fn literal(&mut self, literal: Pair<Rule>) -> Symbol {
        let number = literal.into_inner().next().unwrap();
        let s_type = match number.as_rule() {
            Rule::int_number => Type::Int,
            Rule::uint_number => Type::Unsigned,
            _ => unreachable!(),
        };
        let name = String::from(number.as_str());

        self.symbols
            .push(Symbol {
                name,
                kind: Kinds::LIT,
                s_type,
                ordinal: 0,
            })
            .clone()
    }

    fn function_call(&mut self, fun_call: Pair<Rule>) -> Result<Symbol> {
        let mut parts = fun_call.into_inner();
        let fun = self.id(parts.next().unwrap(), Kinds::FUN)?;
        let mut _arg = None;
        if let Some(num_exp) = parts.next() {
            _arg = Some(self.num_exp(num_exp)?);
        }
        //TODO check number and type of params
        Ok(fun)
    }
}
