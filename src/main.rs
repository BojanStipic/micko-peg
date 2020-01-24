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

use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        input
    };

    let _program = micko_peg::parse(&input)?;
    Ok(())
}
