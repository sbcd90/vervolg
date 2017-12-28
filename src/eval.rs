// MIT License
//
// Copyright (c) 2017 Hans-Martin Will
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::iter::{IntoIterator, Iterator};

use csv::StringRecord;

/// We are using the StringRecord type provided by the CSV library as row representation
pub type Row = StringRecord;
pub type Error = String;
pub type RowResult = Result<Row, Error>;

/// Representation of a set of rows
pub struct RowSet {

}

impl IntoIterator for RowSet {
    type Item = RowResult;
    type IntoIter = RowSetIterator;

    fn into_iter(self) -> RowSetIterator {
        RowSetIterator {
            // TODO
        }
    }
}

pub struct RowSetIterator {

}

impl Iterator for RowSetIterator {
    type Item = RowResult;

    fn next(&mut self) -> Option<RowResult> {
        // TODO
        None
    }
}

/// An evaluation engine for SQL statements
pub struct Evaluator {

}

/// An operator that used to construct query pipelines.
trait Operator {

}

