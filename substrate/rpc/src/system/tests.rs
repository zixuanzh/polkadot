// Copyright 2017 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use super::error::*;

impl SystemApi for () {
	fn system_version(&self) -> Result<String> {
		Ok("0.2.0".into())
	}
	fn system_chain(&self) -> Result<String> {
		Ok("test".into())
	}
}

#[test]
fn system_version_works() {
	assert_matches!(
		AuthorApi::system_version(&()),
		Ok("0.2.0".to_owned())
	);
}

#[test]
fn system_chain_works() {
	assert_matches!(
		AuthorApi::system_chain(&()),
		Ok("test".to_owned())
	);
}
