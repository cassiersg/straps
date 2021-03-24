// STRAPS - Statistical Testing of RAndom Probing Security
// Copyright (C) 2021 UCLouvain
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod combinatorics;
// This is pub only for benchmarking purpose.
pub mod cum_transform;
mod gadget;
mod ldt;
pub(crate) mod multiprogress;
pub(crate) mod progress;
mod rpm_sim;
mod utils;

pub(crate) use gadget::SimGadget;
pub(crate) use ldt::LeakageDistribution;
pub(crate) use rpm_sim::{CntSim, CntSimSt, GPdt, SampleRes, INPUT_AXIS};
