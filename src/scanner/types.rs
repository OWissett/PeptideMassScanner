// Copyright (c) 2022 Oliver Wissett
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// This file contains types which are used across multiple files.

use std::collections::HashMap;

pub type SeqMap = HashMap<String, (f64, i32)>; // HashMap<Protein Sequence, (fragment mass, number of occurrences)>
pub type Matrix2D = Vec<Vec<f64>>;