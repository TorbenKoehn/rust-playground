use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

use crate::{error::Error, resource::value::Value};

pub trait ResourceValueWriteExt: Write {
  fn write_resource_value(&mut self, value: Value, size: usize) -> Result<(), Error> {
    match value {
      Value::None => {}
      Value::Byte(byte_value) => {
        self.write_u8(byte_value)?;
      }
      Value::Short(short_value) => {
        self.write_i16::<LittleEndian>(short_value)?;
      }
      Value::UShort(ushort_value) => {
        self.write_u16::<LittleEndian>(ushort_value)?;
      }
      Value::Int(int_value) => {
        self.write_i32::<LittleEndian>(int_value)?;
      }
      Value::UInt(uint_value) => {
        self.write_u32::<LittleEndian>(uint_value)?;
      }
      Value::Float(float_value) => {
        self.write_f32::<LittleEndian>(float_value)?;
      }
      Value::Double(double_value) => {
        self.write_f64::<LittleEndian>(double_value)?;
      }
      Value::IVec2([ivec2_0, ivec2_1]) => {
        self.write_i32::<LittleEndian>(ivec2_0)?;
        self.write_i32::<LittleEndian>(ivec2_1)?;
      }
      Value::IVec3([ivec3_0, ivec3_1, ivec3_2]) => {
        self.write_i32::<LittleEndian>(ivec3_0)?;
        self.write_i32::<LittleEndian>(ivec3_1)?;
        self.write_i32::<LittleEndian>(ivec3_2)?;
      }
      Value::IVec4([ivec4_0, ivec4_1, ivec4_2, ivec4_3]) => {
        self.write_i32::<LittleEndian>(ivec4_0)?;
        self.write_i32::<LittleEndian>(ivec4_1)?;
        self.write_i32::<LittleEndian>(ivec4_2)?;
        self.write_i32::<LittleEndian>(ivec4_3)?;
      }
      Value::Vec2([vec2_0, vec2_1]) => {
        self.write_f32::<LittleEndian>(vec2_0)?;
        self.write_f32::<LittleEndian>(vec2_1)?;
      }
      Value::Vec3([vec3_0, vec3_1, vec3_2]) => {
        self.write_f32::<LittleEndian>(vec3_0)?;
        self.write_f32::<LittleEndian>(vec3_1)?;
        self.write_f32::<LittleEndian>(vec3_2)?;
      }
      Value::Vec4([vec4_0, vec4_1, vec4_2, vec4_3]) => {
        self.write_f32::<LittleEndian>(vec4_0)?;
        self.write_f32::<LittleEndian>(vec4_1)?;
        self.write_f32::<LittleEndian>(vec4_2)?;
        self.write_f32::<LittleEndian>(vec4_3)?;
      }
      Value::Mat2([[mat2_0_0, mat2_0_1], [mat2_1_0, mat2_1_1]]) => {
        self.write_f32::<LittleEndian>(mat2_0_0)?;
        self.write_f32::<LittleEndian>(mat2_0_1)?;
        self.write_f32::<LittleEndian>(mat2_1_0)?;
        self.write_f32::<LittleEndian>(mat2_1_1)?;
      }
      Value::Mat3(
        [[mat3_0_0, mat3_0_1, mat3_0_2], [mat3_1_0, mat3_1_1, mat3_1_2], [mat3_2_0, mat3_2_1, mat3_2_2]],
      ) => {
        self.write_f32::<LittleEndian>(mat3_0_0)?;
        self.write_f32::<LittleEndian>(mat3_0_1)?;
        self.write_f32::<LittleEndian>(mat3_0_2)?;
        self.write_f32::<LittleEndian>(mat3_1_0)?;
        self.write_f32::<LittleEndian>(mat3_1_1)?;
        self.write_f32::<LittleEndian>(mat3_1_2)?;
        self.write_f32::<LittleEndian>(mat3_2_0)?;
        self.write_f32::<LittleEndian>(mat3_2_1)?;
        self.write_f32::<LittleEndian>(mat3_2_2)?;
      }
      Value::Mat3x4(
        [[mat3x4_0_0, mat3x4_0_1, mat3x4_0_2], [mat3x4_1_0, mat3x4_1_1, mat3x4_1_2], [mat3x4_2_0, mat3x4_2_1, mat3x4_2_2], [mat3x4_3_0, mat3x4_3_1, mat3x4_3_2]],
      ) => {
        self.write_f32::<LittleEndian>(mat3x4_0_0)?;
        self.write_f32::<LittleEndian>(mat3x4_0_1)?;
        self.write_f32::<LittleEndian>(mat3x4_0_2)?;
        self.write_f32::<LittleEndian>(mat3x4_1_0)?;
        self.write_f32::<LittleEndian>(mat3x4_1_1)?;
        self.write_f32::<LittleEndian>(mat3x4_1_2)?;
        self.write_f32::<LittleEndian>(mat3x4_2_0)?;
        self.write_f32::<LittleEndian>(mat3x4_2_1)?;
        self.write_f32::<LittleEndian>(mat3x4_2_2)?;
        self.write_f32::<LittleEndian>(mat3x4_3_0)?;
        self.write_f32::<LittleEndian>(mat3x4_3_1)?;
        self.write_f32::<LittleEndian>(mat3x4_3_2)?;
      }
      Value::Mat4x3(
        [[mat4x3_0_0, mat4x3_0_1, mat4x3_0_2, mat4x3_0_3], [mat4x3_1_0, mat4x3_1_1, mat4x3_1_2, mat4x3_1_3], [mat4x3_2_0, mat4x3_2_1, mat4x3_2_2, mat4x3_2_3]],
      ) => {
        self.write_f32::<LittleEndian>(mat4x3_0_0)?;
        self.write_f32::<LittleEndian>(mat4x3_0_1)?;
        self.write_f32::<LittleEndian>(mat4x3_0_2)?;
        self.write_f32::<LittleEndian>(mat4x3_0_3)?;
        self.write_f32::<LittleEndian>(mat4x3_1_0)?;
        self.write_f32::<LittleEndian>(mat4x3_1_1)?;
        self.write_f32::<LittleEndian>(mat4x3_1_2)?;
        self.write_f32::<LittleEndian>(mat4x3_1_3)?;
        self.write_f32::<LittleEndian>(mat4x3_2_0)?;
        self.write_f32::<LittleEndian>(mat4x3_2_1)?;
        self.write_f32::<LittleEndian>(mat4x3_2_2)?;
        self.write_f32::<LittleEndian>(mat4x3_2_3)?;
      }
      Value::Mat4(
        [[mat4_0_0, mat4_0_1, mat4_0_2, mat4_0_3], [mat4_1_0, mat4_1_1, mat4_1_2, mat4_1_3], [mat4_2_0, mat4_2_1, mat4_2_2, mat4_2_3], [mat4_3_0, mat4_3_1, mat4_3_2, mat4_3_3]],
      ) => {
        self.write_f32::<LittleEndian>(mat4_0_0)?;
        self.write_f32::<LittleEndian>(mat4_0_1)?;
        self.write_f32::<LittleEndian>(mat4_0_2)?;
        self.write_f32::<LittleEndian>(mat4_0_3)?;
        self.write_f32::<LittleEndian>(mat4_1_0)?;
        self.write_f32::<LittleEndian>(mat4_1_1)?;
        self.write_f32::<LittleEndian>(mat4_1_2)?;
        self.write_f32::<LittleEndian>(mat4_1_3)?;
        self.write_f32::<LittleEndian>(mat4_2_0)?;
        self.write_f32::<LittleEndian>(mat4_2_1)?;
        self.write_f32::<LittleEndian>(mat4_2_2)?;
        self.write_f32::<LittleEndian>(mat4_2_3)?;
        self.write_f32::<LittleEndian>(mat4_3_0)?;
        self.write_f32::<LittleEndian>(mat4_3_1)?;
        self.write_f32::<LittleEndian>(mat4_3_2)?;
        self.write_f32::<LittleEndian>(mat4_3_3)?;
      }
      Value::Bool(bool_value) => {
        self.write_u8(bool_value as u8)?;
      }
      Value::ULongLong(ulong_long_value) => {
        self.write_u64::<LittleEndian>(ulong_long_value)?;
      }
      Value::Long(long_value) => {
        self.write_i64::<LittleEndian>(long_value)?;
      }
      Value::Int8(int8_value) => {
        self.write_i8(int8_value)?;
      }
      Value::Uuid(uuid_value) => {
        self.write_all(&uuid_value)?;
      }
      Value::Int64(int64_value) => {
        self.write_i64::<LittleEndian>(int64_value)?;
      }
      _ => unimplemented!(),
    };
    Ok(())
  }
}

impl<R: Write + ?Sized> ResourceValueWriteExt for R {}
