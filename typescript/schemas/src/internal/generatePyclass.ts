import {
  FoxgloveEnumSchema,
  FoxgloveMessageField,
  FoxgloveMessageSchema,
  FoxglovePrimitive,
  FoxgloveSchema,
} from "./types";

/**
 * Generate the module header for schema pyclass definitions.
 */
export function generateSchemaPrelude(): string {
  const docs = [
    `//! Definitions for well-known Foxglove schemas`,
    `//! Generated by https://github.com/foxglove/schemas`,
    // Accept any number of arguments for Python constructors
    `#![allow(clippy::too_many_arguments)]`,
    // Match Foxglove enum conventions elsewhere (shared prefix)
    `#![allow(clippy::enum_variant_names)]`,
    // Allow capitalized single-letter enum variants
    `#![allow(non_snake_case)]`,
  ];

  const imports = ["use pyo3::prelude::*;"];

  const outputSections = [docs.join("\n"), imports.join("\n")];

  return outputSections.join("\n") + "\n\n";
}

/**
 * Generate a `pyclass`-annotated struct or enum definition for the given schema.
 */
export function generatePyclass(schema: FoxgloveSchema): string {
  return isMessageSchema(schema) ? generateMessageClass(schema) : generateEnumClass(schema);
}

/**
 * Generate a .pyi stub for the given schemas.
 */
export function generatePySchemaStub(schemas: FoxgloveSchema[]): string {
  const header = [
    "# Generated by https://github.com/foxglove/schemas",
    "from enum import Enum",
    "from typing import List",
  ].join("\n") + "\n";

  const timeTypes = generateTimeTypeStubs();

  const enums = schemas
    .filter((schema) => schema.type === "enum")
    .map((schema) => {
      const name = enumName(schema);
      const doc = ['    """', `    ${schema.description}`, '    """'];
      const values = schema.values.map((value) => {
        return `    ${constantToTitleCase(value.name)} = ${value.value}`;
      });
      return {
        name,
        source: [`class ${name}(Enum):`, ...doc, ...values].join("\n") + "\n\n",
      };
    });

  const classes = schemas.filter(isMessageSchema).map((schema) => {
    const name = structName(schema.name);
    const doc = ['    """', `    ${schema.description}`, '    """'];
    const params = schema.fields
      .map((field) => {
        return `        ${field.name}: "${pythonOutputType(field)}"`;
      })
      .join(",\n");

    return {
      name,
      source: [
        `class ${name}:`,
        ...doc,
        `    def __new__(`,
        "        cls,",
        "        *,",
        params,
        `    ) -> "${name}": ...`,
      ].join("\n") + "\n\n",
    };
  });

  const definitions = [...enums, ...classes, ...timeTypes]
    .sort((a, b) => a.name.localeCompare(b.name))
    .map(({ source }) => source);

  return [header, ...definitions].join("\n");
}

function rustDoc(str: string, opts: { indent?: number } = {}): string {
  const ws = " ".repeat(opts.indent ?? 0);
  return str
    .split("\n")
    .map((line) => `${ws}/// ${line}`)
    .join("\n");
}

function generateMessageClass(schema: FoxgloveMessageSchema): string {
  const className = structName(schema.name);
  const schemaFields = Array.from(schema.fields).map((field) => ({
    fieldName: safeName(field.name),
    argName: safeName(field.name),
    description: rustDoc(field.description, { indent: 4 }),
    field,
  }));
  const struct = [
    rustDoc(schema.description),
    "#[pyclass]",
    "#[derive(Clone)]",
    `pub(crate) struct ${className}(pub(crate) foxglove::schemas::${className});`,
  ];

  function fieldValue(field: FoxgloveMessageField): string {
    switch (field.type.type) {
      case "primitive":
        if (field.type.name === "time" || field.type.name === "duration") {
          return `${safeName(field.name)}.map(Into::into)`;
        }
        return safeName(field.name);
      case "nested":
        if (field.array != undefined) {
          return `${safeName(field.name)}.into_iter().map(|x| x.into()).collect()`;
        }
        return `${safeName(field.name)}.map(Into::into)`;
      case "enum":
        return `${safeName(field.name)} as i32`;
    }
  }

  function fieldAssignment(field: FoxgloveMessageField): string {
    const name = protoName(field.name);
    const value = fieldValue(field);
    if (name === value) {
      return name;
    }
    return `${name}: ${value}`;
  }

  const signature = schemaFields.map(({ argName, field }) => `${argName}=${defaultValue(field)}`).join(", ");

  const impl = [
    "#[pymethods]",
    `impl ${className} {`,
    `    #[new]`,
    `    #[pyo3(signature = (*, ${signature}) )]`,
    `    fn new(`,
    ...schemaFields.map(({ argName, field }) => `        ${argName}: ${rustOutputType(field)},`),
    `    ) -> Self {`,
    `        Self(foxglove::schemas::${className} {`,
    schemaFields.map(({ field }) => `            ${fieldAssignment(field)},`).join("\n"),
    "        })",
    "    }",
    "}\n\n",
  ];

  const fromTrait = [
    `impl From<${structName(schema.name)}> for foxglove::schemas::${structName(schema.name)} {`,
    `    fn from(value: ${structName(schema.name)}) -> Self {`,
    `        value.0`,
    `    }`,
    `}\n\n`,
  ];

  return [...struct, ...impl, ...fromTrait].join("\n");
}

function generateEnumClass(schema: FoxgloveEnumSchema): string {
  const enumLines = [
    rustDoc(schema.description),
    `#[pyclass(eq, eq_int)]`,
    `#[derive(PartialEq, Clone)]`,
    `pub(crate) enum ${enumName(schema)} {`,
    ...schema.values.map((value) => `    ${constantToTitleCase(value.name)} = ${value.value},`),
    "}\n\n",
  ];

  return enumLines.join("\n");
}

/**
 * For enums with parent schemas, prepend the parent schema name to the enum name,
 * removing duplicated prefixes.
 */
function enumName(schema: FoxgloveEnumSchema): string {
  const name = schema.name.replace(new RegExp("^" + schema.parentSchemaName), "");
  return `${schema.parentSchemaName}${name}`;
}

/**
 * Deal with reserved keywords in identifiers
 */
function safeName(name: string): string {
  if (name === "type") {
    return "r#type";
  }
  return name;
}

/**
 * A schema is either a message schema or an enum.
 */
function isMessageSchema(schema: FoxgloveSchema): schema is FoxgloveMessageSchema {
  return schema.type === "message";
}

/**
 * Get the rust type for a field.
 * Types are assumed to be owned, and wrapped in a `Vec` if the field is an array.
 * Nested types are optional, unless the field is an array.
 */
function rustOutputType(field: FoxgloveMessageField): string {
  const isVec = field.array != undefined;
  let type: string;
  switch (field.type.type) {
    case "primitive":
      type = rustType(field.type.name);
      break;
    case "nested":
      // Don't wrap in an optional if part of a Vec
      type = isVec ? field.type.schema.name : `Option<${field.type.schema.name}>`;
      break;
    case "enum":
      type = enumName(field.type.enum);
      break;
  }

  return isVec ? `Vec<${type}>` : type;
}

/**
 * Map Foxglove primitive types to Rust primitives, or structs in the case of `time` and `duration`.
 */
function rustType(foxglovePrimitive: FoxglovePrimitive): string {
  switch (foxglovePrimitive) {
    case "string":
      return "String";
    case "float64":
      return "f64";
    case "uint32":
      return "u32";
    case "boolean":
      return "bool";
    case "bytes":
      return "Vec<u8>";
    case "time":
      return "Option<Timestamp>";
    case "duration":
      return "Option<Duration>";
  }
}

/**
 * Get the Python type for a field.
 */
function pythonOutputType(field: FoxgloveMessageField): string {
  let type: string;
  switch (field.type.type) {
    case "primitive":
      type = pythonType(field.type.name);
      break;
    case "nested":
      type = field.type.schema.name;
      break;
    case "enum":
      type = enumName(field.type.enum);
      break;
  }
  return field.array != undefined ? `List[${type}]` : type;
}

/**
 * Get the Python default for a field; used in constructor signatures.
 */
function defaultValue(field: FoxgloveMessageField): string {
  if (field.array != undefined) {
    return "vec![]";
  }
  switch (field.type.type) {
    case "primitive":
      switch (field.type.name) {
        case "string":
          return `"".to_string()`;
        case "float64":
          return "0.0";
        case "uint32":
          return "0";
        case "boolean":
          return "false";
        case "bytes":
          return "vec![]";
        case "time":
        case "duration":
          return "None";
      }
    case "nested":
      return "None";
    case "enum":
      const value = constantToTitleCase(field.type.enum.values[0]!.name);
      return `${enumName(field.type.enum)}::${value}`;
  }
}

/**
 * Map Foxglove primitive types to Python primitives.
 */
function pythonType(foxglovePrimitive: FoxglovePrimitive): string {
  switch (foxglovePrimitive) {
    case "string":
      return "str";
    case "float64":
      return "float";
    case "uint32":
      return "int";
    case "boolean":
      return "bool";
    case "bytes":
      return "bytes";
    case "time":
      return "Timestamp";
    case "duration":
      return "Duration";
  }
}

function protoName(name: string): string {
  if (/^[A-Z]$/.exec(name)) {
    // Schemas may include single-letter capitals; generated proto structs use lowercase
    return name.toLowerCase();
  }
  return safeName(name);
}

function capitalize(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function constantToTitleCase(str: string): string {
  return str
    .split("_")
    .map((word) => word.toLowerCase())
    .map(capitalize)
    .join("");
}

function structName(name: string): string {
  // Match special case handling in protobuf gen
  if (name === "GeoJSON") {
    return "GeoJson";
  }
  return name;
}

/**
 * .pyi stubs for Timestamp and Duration.
 */
function generateTimeTypeStubs(): { name: string, source: string }[] {
  const timestamp = `
class Timestamp:
    """
    A timestamp in seconds and nanoseconds
    """
    def __new__(
        cls,
        seconds: int,
        nanos: int,
    ) -> "Timestamp": ...
`;

  const duration = `
class Duration:
    """
    A duration in seconds and nanoseconds
    """
    def __new__(
        cls,
        seconds: int,
        nanos: int,
    ) -> "Duration": ...
`;

  return [{ name: "Timestamp", source: timestamp }, { name: "Duration", source: duration }];
}

/**
 * Defines a struct for representing Timestamp and Duration.
 *
 * This also provides a `From` implementation into prost types for proto serialization.
 */
export function generateTimeTypes(): string {
  return `
#[pyclass]
#[derive(Clone)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}

#[pymethods]
impl Timestamp {
    #[new]
    #[pyo3(signature = (seconds=0, nanos=None))]
    fn new(seconds: i64, nanos: Option<i32>) -> Self {
        Self {
            seconds,
            nanos: nanos.unwrap_or_default(),
        }
    }
}

impl From<Timestamp> for prost_types::Timestamp {
    fn from(value: Timestamp) -> Self {
        Self {
            seconds: value.seconds,
            nanos: value.nanos,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Duration {
    pub seconds: u64,
    pub nanos: u32,
}

#[pymethods]
impl Duration {
    #[new]
    #[pyo3(signature = (seconds=0, nanos=None))]
    fn new(seconds: u64, nanos: Option<u32>) -> Self {
        Self {
            seconds,
            nanos: nanos.unwrap_or_default(),
        }
    }
}

impl From<Duration> for prost_types::Duration {
    fn from(value: Duration) -> Self {
        Self {
            seconds: value.seconds.try_into().unwrap_or_default(),
            nanos: value.nanos.try_into().unwrap_or_default(),
        }
    }
}
`;
}

/**
 * Generate a rust function to register the schemas in a submodule.
 * https://pyo3.rs/v0.23.4/module.html
 */
export function generateSchemaModuleRegistration(schemas: FoxgloveSchema[]): string {
  return `
pub fn register_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = PyModule::new(parent_module.py(), "schemas")?;

    module.add_class::<Duration>()?;
    module.add_class::<Timestamp>()?;
    ${schemas.map((schema) => `module.add_class::<${pyClassName(schema)}>()?;`).join("\n    ")}

    // Define as a package
    // https://github.com/PyO3/pyo3/issues/759
    let py = parent_module.py();
    py.import("sys")?
        .getattr("modules")?
        .set_item("foxglove._foxglove_py.schemas", &module)?;

    parent_module.add_submodule(&module)
}
`;
}

/**
 * Generate a rust function to register the channels in a submodule.
 * https://pyo3.rs/v0.23.4/module.html
 */
function generateChannelModuleRegistration(messageSchemas: FoxgloveMessageSchema[]): string {
  const schemas = messageSchemas.filter((schema) => !schema.name.endsWith("Primitive"));
  return `
pub fn register_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = PyModule::new(parent_module.py(), "channels")?;

    ${schemas.map((schema) => `module.add_class::<${structName(schema.name)}Channel>()?;`).join("\n    ")}

    // Define as a package
    // https://github.com/PyO3/pyo3/issues/759
    let py = parent_module.py();
    py.import("sys")?
        .getattr("modules")?
        .set_item("foxglove._foxglove_py.channels", &module)?;

    parent_module.add_submodule(&module)
}
`;
}

function pyClassName(schema: FoxgloveSchema): string {
  return isMessageSchema(schema) ? structName(schema.name) : enumName(schema);
}

/**
 * Generate a concrete `pyclass`-annotated Channel struct for each message schema, since generics
 * can't be exported directly to Python.
 */
export function generateChannelClasses(messageSchemas: FoxgloveMessageSchema[]): string {
  const schemas = messageSchemas.filter((schema) => !schema.name.endsWith("Primitive"));

  const imports = [
    `use foxglove::TypedChannel;`,
    `use pyo3::prelude::*;`,
    `use crate::{errors::PyFoxgloveError, PartialMetadata};`,
    `use super::schemas;`,
  ].join("\n");

  const channelModuleRegistration = generateChannelModuleRegistration(schemas);

  const classes = schemas.map((schema) => {
    const schemaClass = structName(schema.name);
    const channelClass = `${schemaClass}Channel`;
    return `
#[pyclass]
struct ${channelClass}(TypedChannel<foxglove::schemas::${schemaClass}>);

#[pymethods]
impl ${channelClass} {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::${schemaClass}) {
        self.0.log(&msg.0);
    }

    pub fn log_with_meta(
        &self,
        msg: &schemas::${schemaClass},
        opts: Bound<'_, PartialMetadata>,
    ) {
        let metadata = opts.extract::<PartialMetadata>().ok().unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }
}
`;
  });

  return [imports, channelModuleRegistration, ...classes].join("\n\n");
}

/**
 * Generate a .pyi stub for the given schemas.
 */
export function generatePyChannelStub(messageSchemas: FoxgloveMessageSchema[]): string {
  const schemas = messageSchemas.filter((schema) => !schema.name.endsWith("Primitive"));

  const imports = [
    `from . import PartialMetadata`,
    ...schemas.map((schema) =>  `from .schemas import ${structName(schema.name)}`),
  ];

  const classes = schemas.map((schema) => {
    const schemaClass = structName(schema.name);
    const channelClass = `${schemaClass}Channel`;
    const doc = ['    """', `    A channel for logging ${schemaClass} messages`, '    """'];

    return {
      name: channelClass,
      source: [
        `class ${channelClass}:`,
        ...doc,
        `    def __new__(`,
        `        cls,`,
        `        topic: str,`,
        `    ) -> "${channelClass}": ...\n`,
        `    def log(`,
        `        self,`,
        `        message: "${schemaClass}",`,
        `    ) -> "${channelClass}": ...\n`,
        `    def log_with_meta(`,
        `        self,`,
        `        message: "${schemaClass}",`,
        `        metadata: "PartialMetadata",`,
        `    ) -> "${channelClass}": ...\n`,
      ].join("\n"),
    };
  });

  const definitions = [...classes]
    .sort((a, b) => a.name.localeCompare(b.name))
    .map(({ source }) => source);

  return [...imports, ...definitions].join("\n");
}
