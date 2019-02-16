extern crate byteorder;
extern crate gpu_interp;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use gpu_interp::Buffer;
use std::io::{Read, Result, Write};

pub mod util {
    use gpu_interp::Buffer;
    pub fn slice(buffer: &mut Buffer, depth: u32, out: &mut Vec<f32>) {
        let width = buffer.width as usize;
        let height = buffer.width as usize;
        let buff = buffer.to_memory();
        let pos = |x: usize, y: usize| x + y * width + depth as usize * width * height;
        out.resize(width * height, 0.0);
        for y in 0..height {
            for x in 0..width {
                out[x + y * width] = buff[pos(x, y) as usize];
            }
        }
    }
}

pub fn read<R: Read>(mut reader: R) -> Result<Buffer> {
    let width = reader.read_u32::<LittleEndian>()?;
    let height = reader.read_u32::<LittleEndian>()?;
    let depth = reader.read_u32::<LittleEndian>()?;

    let capacity = width as usize * depth as usize * height as usize;
    let mut buffer = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        buffer.push(reader.read_f32::<LittleEndian>()?);
    }

    Ok(Buffer::from_memory(buffer, width, height, depth))
}

pub fn write<R: Write>(mut writer: R, buffer: &mut Buffer) -> Result<()> {
    writer.write_u32::<LittleEndian>(buffer.width)?;
    writer.write_u32::<LittleEndian>(buffer.height)?;
    writer.write_u32::<LittleEndian>(buffer.depth)?;

    for &v in buffer.to_memory() {
        writer.write_f32::<LittleEndian>(v)?;
    }

    Ok(())
}
