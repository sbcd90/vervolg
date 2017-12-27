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

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Unit,
    Primitive(Primitive),
    Array(Primitive, usize),
    Variant(Vec<Case>),
    Record(Vec<Field>),
    Tuple(Vec<Box<Type>>),
    Function { arguments: Box<Type>, result: Box<Type> },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Primitive {
    Boolean,
    Char(usize),
    Varchar(usize),
    Binary(usize),
    Varbinary(usize),
    Integer(usize),
    Float(usize),
    Date,
    Time,
    Timestamp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub label: String,
    pub typ: Box<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Case {
    pub label: String,
    pub typ: Box<Type>,
}