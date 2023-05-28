use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{error::Error, resource::value::Value};

pub trait ResourceValueReadExt: Read {
  fn read_resource_value(&mut self, type_id: u32) -> Result<Value, Error> {
    match type_id {
      0 => Ok(Value::None),
      1 => Ok(Value::Byte(self.read_u8()?)),
      2 => Ok(Value::Short(self.read_i16::<LittleEndian>()?)),
      3 => Ok(Value::UShort(self.read_u16::<LittleEndian>()?)),
      4 => Ok(Value::Int(self.read_i32::<LittleEndian>()?)),
      5 => Ok(Value::UInt(self.read_u32::<LittleEndian>()?)),
      6 => Ok(Value::Float(self.read_f32::<LittleEndian>()?)),
      7 => Ok(Value::Double(self.read_f64::<LittleEndian>()?)),
      8 => Ok(Value::IVec2([
        self.read_i32::<LittleEndian>()?,
        self.read_i32::<LittleEndian>()?,
      ])),
      9 => Ok(Value::IVec3([
        self.read_i32::<LittleEndian>()?,
        self.read_i32::<LittleEndian>()?,
        self.read_i32::<LittleEndian>()?,
      ])),
      10 => Ok(Value::IVec4([
        self.read_i32::<LittleEndian>()?,
        self.read_i32::<LittleEndian>()?,
        self.read_i32::<LittleEndian>()?,
        self.read_i32::<LittleEndian>()?,
      ])),
      11 => Ok(Value::Vec2([
        self.read_f32::<LittleEndian>()?,
        self.read_f32::<LittleEndian>()?,
      ])),
      12 => Ok(Value::Vec3([
        self.read_f32::<LittleEndian>()?,
        self.read_f32::<LittleEndian>()?,
        self.read_f32::<LittleEndian>()?,
      ])),
      13 => Ok(Value::Vec4([
        self.read_f32::<LittleEndian>()?,
        self.read_f32::<LittleEndian>()?,
        self.read_f32::<LittleEndian>()?,
        self.read_f32::<LittleEndian>()?,
      ])),
      14 => Ok(Value::Mat2([
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
      ])),
      15 => Ok(Value::Mat3([
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
      ])),
      16 => Ok(Value::Mat3x4([
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
      ])),
      17 => Ok(Value::Mat4x3([
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
      ])),
      18 => Ok(Value::Mat4([
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
        [
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
          self.read_f32::<LittleEndian>()?,
        ],
      ])),
      19 => Ok(Value::Bool(self.read_u8()? != 0)),
      24 => Ok(Value::ULongLong(self.read_u64::<LittleEndian>()?)),
      26 => Ok(Value::Long(self.read_i64::<LittleEndian>()?)),
      27 => Ok(Value::Int8(self.read_i8()?)),
      31 => {
        let mut uuid_buffer = [0u8; 16];
        self.read_exact(&mut uuid_buffer)?;
        Ok(Value::Uuid(uuid_buffer))
      }
      32 => Ok(Value::Int64(self.read_i64::<LittleEndian>()?)),
      _ => Err(Error::InvalidTypeId(type_id)),
    }
  }
}

impl<R: Read + ?Sized> ResourceValueReadExt for R {}
