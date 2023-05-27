pub mod read;

use std::fmt::{Debug, Display};

use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TranslatedFsStringArgument {
  pub key: String,
  pub string: Value,
  pub value: String,
}

#[derive(Serialize, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Value {
  None,
  Byte(u8),
  Short(i16),
  UShort(u16),
  Int(i32),
  UInt(u32),
  Float(f32),
  Double(f64),
  IVec2([i32; 2]),
  IVec3([i32; 3]),
  IVec4([i32; 4]),
  Vec2([f32; 2]),
  Vec3([f32; 3]),
  Vec4([f32; 4]),
  Mat2([[f32; 2]; 2]),
  Mat3([[f32; 3]; 3]),
  Mat3x4([[f32; 3]; 4]),
  Mat4x3([[f32; 4]; 3]),
  Mat4([[f32; 4]; 4]),
  Bool(bool),
  String(String),
  Path(String),
  FixedString(String),
  LsString(String),
  ULongLong(u64),
  ScratchBuffer(Vec<u8>),
  Long(i64),
  Int8(i8),
  TranslatedString {
    version: u16,
    value: String,
    handle: String,
  },
  WString(String),
  LswString(String),
  Uuid([u8; 16]),
  Int64(i64),
  TranslatedFsString {
    version: u16,
    value: String,
    handle: String,
    arguments: Vec<TranslatedFsStringArgument>,
  },
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::None => write!(f, "None"),
      Self::Byte(value) => write!(f, "Byte({})", value),
      Self::Short(value) => write!(f, "Short({})", value),
      Self::UShort(value) => write!(f, "UShort({})", value),
      Self::Int(value) => write!(f, "Int({})", value),
      Self::UInt(value) => write!(f, "UInt({})", value),
      Self::Float(value) => write!(f, "Float({})", value),
      Self::Double(value) => write!(f, "Double({})", value),
      Self::IVec2(value) => write!(f, "IVec2({:?})", value),
      Self::IVec3(value) => write!(f, "IVec3({:?})", value),
      Self::IVec4(value) => write!(f, "IVec4({:?})", value),
      Self::Vec2(value) => write!(f, "Vec2({:?})", value),
      Self::Vec3(value) => write!(f, "Vec3({:?})", value),
      Self::Vec4(value) => write!(f, "Vec4({:?})", value),
      Self::Mat2(value) => write!(f, "Mat2({:?})", value),
      Self::Mat3(value) => write!(f, "Mat3({:?})", value),
      Self::Mat3x4(value) => write!(f, "Mat3x4({:?})", value),
      Self::Mat4x3(value) => write!(f, "Mat4x3({:?})", value),
      Self::Mat4(value) => write!(f, "Mat4({:?})", value),
      Self::Bool(value) => write!(f, "Bool({})", value),
      Self::String(value) => write!(f, "String({})", value),
      Self::Path(value) => write!(f, "Path({})", value),
      Self::FixedString(value) => write!(f, "FixedString({})", value),
      Self::LsString(value) => write!(f, "LsString({})", value),
      Self::ULongLong(value) => write!(f, "ULongLong({})", value),
      Self::ScratchBuffer(value) => write!(f, "ScratchBuffer({:?})", value),
      Self::Long(value) => write!(f, "Long({})", value),
      Self::Int8(value) => write!(f, "Int8({})", value),
      Self::TranslatedString {
        version,
        value,
        handle,
      } => write!(
        f,
        "TranslatedString {{ version: {}, value: {}, handle: {} }}",
        version, value, handle
      ),
      Self::WString(value) => write!(f, "WString({})", value),
      Self::LswString(value) => write!(f, "LswString({})", value),
      Self::Uuid(value) => write!(f, "Uuid({:?})", value),
      Self::Int64(value) => write!(f, "Int64({})", value),
      Self::TranslatedFsString {
        version,
        value,
        handle,
        arguments,
      } => write!(
        f,
        "TranslatedFsString {{ version: {}, value: {}, handle: {}, arguments: {:?} }}",
        version, value, handle, arguments
      ),
    }
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::None => write!(f, "None"),
      Self::Byte(value) => write!(f, "{}", value),
      Self::Short(value) => write!(f, "{}", value),
      Self::UShort(value) => write!(f, "{}", value),
      Self::Int(value) => write!(f, "{}", value),
      Self::UInt(value) => write!(f, "{}", value),
      Self::Float(value) => write!(f, "{}", value),
      Self::Double(value) => write!(f, "{}", value),
      Self::IVec2(value) => write!(f, "{:?}", value),
      Self::IVec3(value) => write!(f, "{:?}", value),
      Self::IVec4(value) => write!(f, "{:?}", value),
      Self::Vec2(value) => write!(f, "{:?}", value),
      Self::Vec3(value) => write!(f, "{:?}", value),
      Self::Vec4(value) => write!(f, "{:?}", value),
      Self::Mat2(value) => write!(f, "{:?}", value),
      Self::Mat3(value) => write!(f, "{:?}", value),
      Self::Mat3x4(value) => write!(f, "{:?}", value),
      Self::Mat4x3(value) => write!(f, "{:?}", value),
      Self::Mat4(value) => write!(f, "{:?}", value),
      Self::Bool(value) => write!(f, "{}", value),
      Self::String(value) => write!(f, "{}", value),
      Self::Path(value) => write!(f, "{}", value),
      Self::FixedString(value) => write!(f, "{}", value),
      Self::LsString(value) => write!(f, "{}", value),
      Self::ULongLong(value) => write!(f, "{}", value),
      Self::ScratchBuffer(value) => write!(f, "{:X?}", value),
      Self::Long(value) => write!(f, "{}", value),
      Self::Int8(value) => write!(f, "{}", value),
      Self::TranslatedString { value, .. } => write!(f, "{}", value),
      Self::WString(value) => write!(f, "{}", value),
      Self::LswString(value) => write!(f, "{}", value),
      Self::Uuid(value) => write!(f, "{:?}", value),
      Self::Int64(value) => write!(f, "{}", value),
      Self::TranslatedFsString { value, .. } => write!(f, "{}", value),
    }
  }
}
