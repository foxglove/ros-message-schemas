import { generateSchemaPrelude, generatePyclass } from "./generatePyclass";
import { exampleEnum, exampleMessage } from "./testFixtures";

describe("generatePyclass", () => {
  it("generates module header", () => {
    expect(generateSchemaPrelude()).toMatchInlineSnapshot(`
        "//! Definitions for well-known Foxglove schemas
        //! Generated by https://github.com/foxglove/schemas
        #![allow(clippy::too_many_arguments)]
        #![allow(clippy::enum_variant_names)]
        #![allow(non_snake_case)]
        use pyo3::prelude::*;

        "
        `);
  });

  it("generates an enum", () => {
    expect(generatePyclass(exampleEnum)).toMatchInlineSnapshot(`
        "/// An example enum
        #[pyclass(eq, eq_int)]
        #[derive(PartialEq, Clone)]
        pub(crate) enum ExampleMessageExampleEnum {
            A = 0,
            B = 1,
        }

        "
        `);
  });

  it("generates a struct from a message", () => {
    expect(generatePyclass(exampleMessage)).toMatchInlineSnapshot(`
        "/// An example type
        #[pyclass]
        #[derive(Clone)]
        pub(crate) struct ExampleMessage(pub(crate) foxglove::schemas::ExampleMessage);
        #[pymethods]
        impl ExampleMessage {
            #[new]
            fn new(
                field_duration: Duration,
                field_time: Timestamp,
                field_boolean: bool,
                field_bytes: Vec<u8>,
                field_float64: f64,
                field_uint32: u32,
                field_string: String,
                field_duration_array: Vec<Duration>,
                field_time_array: Vec<Timestamp>,
                field_boolean_array: Vec<bool>,
                field_bytes_array: Vec<Vec<u8>>,
                field_float64_array: Vec<f64>,
                field_uint32_array: Vec<u32>,
                field_string_array: Vec<String>,
                field_duration_fixed_array: Vec<Duration>,
                field_time_fixed_array: Vec<Timestamp>,
                field_boolean_fixed_array: Vec<bool>,
                field_bytes_fixed_array: Vec<Vec<u8>>,
                field_float64_fixed_array: Vec<f64>,
                field_uint32_fixed_array: Vec<u32>,
                field_string_fixed_array: Vec<String>,
                field_enum: ExampleMessageExampleEnum,
                field_enum_array: Vec<ExampleMessageExampleEnum>,
                field_nested: NestedMessage,
                field_nested_array: Vec<NestedMessage>,
            ) -> Self {
                Self(foxglove::schemas::ExampleMessage {
                    field_duration: Some(field_duration.into()),
                    field_time: Some(field_time.into()),
                    field_boolean,
                    field_bytes,
                    field_float64,
                    field_uint32,
                    field_string,
                    field_duration_array: Some(field_duration_array.into()),
                    field_time_array: Some(field_time_array.into()),
                    field_boolean_array,
                    field_bytes_array,
                    field_float64_array,
                    field_uint32_array,
                    field_string_array,
                    field_duration_fixed_array: Some(field_duration_fixed_array.into()),
                    field_time_fixed_array: Some(field_time_fixed_array.into()),
                    field_boolean_fixed_array,
                    field_bytes_fixed_array,
                    field_float64_fixed_array,
                    field_uint32_fixed_array,
                    field_string_fixed_array,
                    field_enum: field_enum as i32,
                    field_enum_array: field_enum_array as i32,
                    field_nested: Some(field_nested.into()),
                    field_nested_array: field_nested_array.into_iter().map(|x| x.into()).collect(),
                })
            }
        }


        impl From<ExampleMessage> for foxglove::schemas::ExampleMessage {
            fn from(value: ExampleMessage) -> Self {
                value.0
            }
        }

        "
        `);
  });
});
