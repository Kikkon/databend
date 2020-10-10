// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under Apache License, Version 2.0.

use std::fmt;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DivFunction {
    left: Box<Function>,
    right: Box<Function>,
}

impl DivFunction {
    pub fn create(args: &[Function]) -> Result<Function> {
        Ok(Function::Div(DivFunction {
            left: Box::from(args[0].clone()),
            right: Box::from(args[1].clone()),
        }))
    }

    pub fn name(&self) -> String {
        "DivFunction".to_string()
    }

    pub fn return_type(&self, input_schema: &DataSchema) -> Result<DataType> {
        self.left.return_type(input_schema)
    }

    pub fn nullable(&self, _input_schema: &DataSchema) -> Result<bool> {
        Ok(false)
    }

    pub fn evaluate(&self, block: &DataBlock) -> Result<DataArrayRef> {
        array_div(self.left.evaluate(block)?, self.right.evaluate(block)?)
    }
}

impl fmt::Display for DivFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} / {:?}", self.left, self.right)
    }
}
