use bytes::{BufMut, Buf};
use crate::buffers::{VarInts, VarIntsMut, Strings, StringsMut, Bools, BoolsMut};

pub struct Vari32 {
    pub val: i32
}

pub trait Sendable {
    fn read(buffer: &mut dyn Buf) -> Self;
    fn write(buffer: &mut dyn BufMut, data: &Self);
}

impl Sendable for Vari32 {
    fn read(mut buffer: &mut dyn Buf) -> Self {
        Vari32 { val: buffer.get_var_i32().0 }
    }

    fn write(mut buffer: &mut dyn BufMut, data: &Self) {
        buffer.put_var_i32(data.val)
    }
}

impl Sendable for i32 {
    fn read(buffer: &mut dyn Buf) -> Self {
        buffer.get_i32()
    }

    fn write(buffer: &mut dyn BufMut, data: &Self) {
        buffer.put_i32(*data)
    }
}

impl Sendable for u16 {
    fn read(buffer: &mut dyn Buf) -> Self {
        buffer.get_u16()
    }

    fn write(buffer: &mut dyn BufMut, data: &Self) {
        buffer.put_u16(*data)
    }
}

impl Sendable for u128 {
    fn read(buffer: &mut dyn Buf) -> Self {
        buffer.get_u128()
    }

    fn write(buffer: &mut dyn BufMut, data: &Self) {
        buffer.put_u128(*data)
    }
}

impl Sendable for i16 {
    fn read(buffer: &mut dyn Buf) -> Self {
        buffer.get_i16()
    }

    fn write(buffer: &mut dyn BufMut, data: &Self) {
        buffer.put_i16(*data)
    }
}

impl Sendable for bool {
    fn read(mut buffer: &mut dyn Buf) -> Self {
        buffer.get_bool()
    }

    fn write(mut buffer: &mut dyn BufMut, data: &Self) {
        buffer.put_bool(*data)
    }
}

impl Sendable for f64 {
    fn read(buffer: &mut dyn Buf) -> Self {
        buffer.get_f64()
    }

    fn write(buffer: &mut dyn BufMut, data: &Self) {
        buffer.put_f64(*data)
    }
}

impl Sendable for String {
    fn read(mut buffer: &mut dyn Buf) -> Self {
        buffer.get_string()
    }

    fn write(mut buffer: &mut dyn BufMut, data: &Self) {
        buffer.put_string(data)
    }
}