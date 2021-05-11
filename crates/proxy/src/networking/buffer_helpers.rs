use std::io::{ErrorKind, Read, Write};

use bytes::BufMut;
use mio::net::TcpStream;

use utils::contexts::ConnectionContext;
use utils::indexed_vec::IndexedVec;
use utils::buffers::VarInts;

pub fn read_socket(ctx: &mut ConnectionContext, packet: &mut IndexedVec<u8>) -> bool {
    let result = ctx.stream.read(packet.as_mut_write_slice());
    match result {
        Ok(read) => {
            packet.advance_writer_index(read);
            if read == 0 && packet.vec.len() > packet.get_writer_index() {
                println!("read 0");
                ctx.should_close = true;
                return true;
            }
            true
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::WouldBlock => {}
                _ => {
                    println!("unable to read socket: {:?}", e);
                    ctx.should_close = true;
                    return true;
                }
            }
            false
        }
    }
}

pub fn write_socket(ctx: &mut ConnectionContext, packet: &mut IndexedVec<u8>) {
    if ctx.is_writable {
        if !write_socket0(&mut ctx.stream, packet, &mut ctx.should_close) {
            buffer_write(ctx, packet);
            ctx.is_writable = false;
        }
    } else {
        buffer_write(ctx, packet);
    }
}

pub fn write_socket_slice(ctx: &mut ConnectionContext, packet: &[u8]) {
    if ctx.is_writable {
        let mut total_written = 0;
        loop {
            let result = ctx.stream.write(&packet[total_written..]);
            match result {
                Ok(written) => {
                    total_written += written;
                }
                Err(e) => {
                    match e.kind() {
                        ErrorKind::WouldBlock => {
                            buffer_write_slice(ctx, packet, total_written);
                            ctx.is_writable = false;
                            break;
                        }
                        _ => {
                            println!("unable to write socket: {:?}", e);
                            ctx.should_close = true;
                            return;
                        }
                    }
                }
            }
            if total_written >= packet.len() {
                break;
            }
        }
    } else {
        buffer_write_slice(ctx, packet, 0);
    }
}

pub fn write_socket0(stream: &mut TcpStream, packet: &mut IndexedVec<u8>, should_close: &mut bool) -> bool {
    loop {
        let range = packet.get_reader_index()..packet.get_writer_index();
        let result = stream.write(&mut packet.vec[range]);
        match result {
            Ok(written) => {
                packet.advance_reader_index(written);
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::WouldBlock => {
                        return false;
                    }
                    _ => {
                        println!("unable to write socket: {:?}", e);
                        *should_close = true;
                        return true;
                    }
                }
            }
        }
        if packet.get_reader_index() >= packet.get_writer_index() {
            break;
        }
    }
    true
}

/// store unread data
pub fn buffer_read(ctx: &mut ConnectionContext, buffering_buf: &mut IndexedVec<u8>) {
    ctx.read_buffering.put_slice(buffering_buf.as_slice());
}

/// recall unread data
pub fn unbuffer_read(ctx: &mut ConnectionContext, buffering_buf: &mut IndexedVec<u8>) {
    buffering_buf.put_slice(ctx.read_buffering.as_slice());
    ctx.read_buffering.reset();
}

/// store unwritten data
pub fn buffer_write(ctx: &mut ConnectionContext, buffering_buf: &mut IndexedVec<u8>) {
    ctx.write_buffering.put_slice(buffering_buf.as_slice());
}

/// store unwritten slice of data
pub fn buffer_write_slice(ctx: &mut ConnectionContext, buffering_buf: &[u8], start: usize) {
    let slice = &buffering_buf[start..];
    ctx.write_buffering.put_slice(slice);
}

/// recall unwritten data
pub fn unbuffer_write(ctx: &mut ConnectionContext, buffering_buf: &mut IndexedVec<u8>) {
    let slice = &ctx.write_buffering.vec[ctx.write_buffering.get_reader_index()..ctx.write_buffering.get_writer_index()];
    buffering_buf.put_slice(slice);
    ctx.write_buffering.reset();
}

/// copy data from slice to an IndexedVec
pub fn copy_slice_to(from: &[u8], to: &mut IndexedVec<u8>) {
    let slice = &from[..];
    to.put_slice(slice);
}

pub fn read_frame(buf: &mut IndexedVec<u8>, pointer: usize, len: usize) -> Option<(usize, usize)> {
    buf.set_reader_index(pointer);
    if len > pointer {
        if len - pointer >= 3 {
            return if let Some((len, bytes_read)) = buf.get_var_i32_limit(3) {
                Some((len as usize, bytes_read as usize))
            } else {
                None
            }
        } else {
            for index in pointer..len {
                if buf.vec[index] < 128 {
                    return if let Some((len, bytes_read)) = buf.get_var_i32_limit(3) {
                        Some((len as usize, bytes_read as usize))
                    } else {
                        None
                    }
                }
            }
        }
    }
    None
}
