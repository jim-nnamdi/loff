use std::{io::Read, mem::MaybeUninit, net::TcpStream};


pub unsafe fn _loff_buffer_initialized(slice: &mut [MaybeUninit<u8>]) -> &mut [u8] {
  let data_len = slice.len();
  let data_ptr = slice.as_mut_ptr() as *mut u8;
  std::slice::from_raw_parts_mut(data_ptr, data_len)
}

pub fn _custom_stream_from_loff_unsafe(buffer:&mut Vec<u8>,stream: &mut TcpStream) {
  buffer.reserve(4096);
  let old_length = buffer.len();
  let spare_cap = buffer.spare_capacity_mut();
  let _loff_initialized = unsafe { _loff_buffer_initialized(spare_cap) };
  let read_stream = stream.read(_loff_initialized).unwrap();
  unsafe{buffer.set_len(old_length + read_stream)}
}