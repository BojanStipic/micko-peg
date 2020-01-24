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

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pairs;

#[derive(pest_derive::Parser)]
#[grammar = "minic.pest"]
struct MCParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    MCParser::parse(Rule::program, input)
}
