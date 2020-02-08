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

use std::error::Error;
use std::fs;
use std::path::PathBuf;

const TESTS_DIR: &str = "tests";

fn find_test_files(prefix: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut test_files = vec![];
    for entry in fs::read_dir(TESTS_DIR)? {
        let path = entry?.path();
        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            if file_name.starts_with(prefix) && file_name.ends_with(".mc") {
                test_files.push(path);
            }
        }
    }
    Ok(test_files)
}

#[test]
fn test_ok() -> Result<(), Box<dyn Error>> {
    let test_files = find_test_files("test-ok")?;
    for test_file in test_files {
        let file = fs::read_to_string(&test_file)?;
        micko_peg::run(&file)?;
    }
    Ok(())
}

#[test]
fn test_synerr() -> Result<(), Box<dyn Error>> {
    let test_files = find_test_files("test-synerr")?;
    for test_file in test_files {
        let file = fs::read_to_string(&test_file)?;
        let program = micko_peg::run(&file);
        assert!(
            program.is_err(),
            format!("Error not reported for {}", test_file.display())
        );
    }
    Ok(())
}
