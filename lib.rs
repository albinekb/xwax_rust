#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![feature(link_args)]
#![register_tool(c2rust)]


extern crate libc;

#[link(name="SDL")]
#[link(name="SDL_ttf")]
#[link(name="asound")]
#[link_args = "-lSDL_ttf"]
extern {}


pub mod src {
pub mod alsa;
pub mod controller;
pub mod cues;
pub mod deck;
pub mod device;
pub mod dicer;
pub mod dummy;
pub mod excrate;
pub mod external;
pub mod index;
pub mod interface;
pub mod library;
pub mod listbox;
pub mod lut;
pub mod midi;
pub mod mktimecode;
pub mod player;
pub mod realtime;
pub mod rig;
pub mod selector;
pub mod status;
pub mod thread;
pub mod timecoder;
pub mod track;
} // mod src

