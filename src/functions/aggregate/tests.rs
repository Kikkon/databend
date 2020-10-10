// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::*;

#[allow(dead_code)]
struct Test {
    name: &'static str,
    args: Vec<Function>,
    display: &'static str,
    nullable: bool,
    block: DataBlock,
    expect: DataArrayRef,
    error: &'static str,
    func: Function,
}

#[test]
fn test_cases() {
    use super::*;
    use crate::datatypes::{DataField, Int64Array, UInt64Array};
    use crate::functions::VariableFunction;

    use std::sync::Arc;

    let schema = DataSchema::new(vec![
        DataField::new("a", DataType::Int64, false),
        DataField::new("b", DataType::Int64, false),
    ]);

    let field_a = VariableFunction::create("a").unwrap();
    let field_b = VariableFunction::create("b").unwrap();

    let tests = vec![Test {
        name: "count-passed",
        args: vec![field_a.clone(), field_b.clone()],
        display: "CountAggregateFunction",
        nullable: false,
        func: CountAggregateFunction::create().unwrap(),
        block: DataBlock::new(
            schema.clone(),
            vec![
                Arc::new(Int64Array::from(vec![4, 3, 2, 1])),
                Arc::new(Int64Array::from(vec![1, 2, 3, 4])),
            ],
        ),
        expect: Arc::new(UInt64Array::from(vec![4])),
        error: "",
    }];

    for mut t in tests {
        (t.func).accumulate(&t.block).unwrap();
        let result = (t.func).aggregate();
        match result {
            Ok(ref v) => {
                // Display check.
                let expect_display = t.display.to_string();
                let actual_display = format!("{:?}", (t.func));
                assert_eq!(expect_display, actual_display);

                // Nullable check.
                let expect_null = t.nullable;
                let actual_null = (t.func).nullable(t.block.schema()).unwrap();
                assert_eq!(expect_null, actual_null);

                // Type check.
                let expect_type = &(t.func).return_type(t.block.schema()).unwrap();
                let actual_type = v.data_type();
                assert_eq!(expect_type, actual_type);

                // Result check.
                if !v.equals(&*t.expect) {
                    println!("expect:\n{:?} \nactual:\n{:?}", t.expect, v);
                    assert!(false);
                }
            }
            Err(e) => assert_eq!(t.error, e.to_string()),
        }
    }
}
