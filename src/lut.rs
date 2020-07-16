use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free
 * Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 */
pub type slot_no_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slot {
    pub timecode: libc::c_uint,
    pub next: slot_no_t,
}
/* next slot with the same hash */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lut {
    pub slot: *mut slot,
    pub table: *mut slot_no_t,
    pub avail: slot_no_t,
}
/* next available slot */
/* Initialise an empty hash lookup table to store the given number
 * of timecode -> position lookups */
#[no_mangle]
pub unsafe extern "C" fn lut_init(mut lut: *mut lut, mut nslots: libc::c_int)
 -> libc::c_int {
    let mut n: libc::c_int = 0; /* take the next available slot */
    let mut hashes: libc::c_int = 0;
    let mut bytes: size_t = 0;
    hashes = (1 as libc::c_int) << 16 as libc::c_int;
    bytes =
        (::std::mem::size_of::<slot>() as
             libc::c_ulong).wrapping_mul(nslots as
                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<slot_no_t>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(hashes
                                                                                                              as
                                                                                                              libc::c_ulong));
    fprintf(stderr,
            b"Lookup table has %d hashes to %d slots (%d slots per hash, %zuKb)\n\x00"
                as *const u8 as *const libc::c_char, hashes, nslots,
            nslots / hashes,
            bytes.wrapping_div(1024 as libc::c_int as libc::c_ulong));
    (*lut).slot =
        malloc((::std::mem::size_of::<slot>() as
                    libc::c_ulong).wrapping_mul(nslots as libc::c_ulong)) as
            *mut slot;
    if (*lut).slot.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    (*lut).table =
        malloc((::std::mem::size_of::<slot_no_t>() as
                    libc::c_ulong).wrapping_mul(hashes as libc::c_ulong)) as
            *mut slot_no_t;
    if (*lut).table.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    n = 0 as libc::c_int;
    while n < hashes {
        *(*lut).table.offset(n as isize) =
            -(1 as libc::c_int) as libc::c_uint;
        n += 1
    }
    (*lut).avail = 0 as libc::c_int as slot_no_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lut_clear(mut lut: *mut lut) {
    free((*lut).table as *mut libc::c_void);
    free((*lut).slot as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lut_push(mut lut: *mut lut,
                                  mut timecode: libc::c_uint) {
    let mut hash: libc::c_uint = 0;
    let mut slot_no: slot_no_t = 0;
    let mut slot: *mut slot = 0 as *mut slot;
    let fresh0 = (*lut).avail;
    (*lut).avail = (*lut).avail.wrapping_add(1);
    slot_no = fresh0;
    slot = &mut *(*lut).slot.offset(slot_no as isize) as *mut slot;
    (*slot).timecode = timecode;
    hash =
        timecode &
            (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as
                libc::c_uint;
    (*slot).next = *(*lut).table.offset(hash as isize);
    *(*lut).table.offset(hash as isize) = slot_no;
}
#[no_mangle]
pub unsafe extern "C" fn lut_lookup(mut lut: *mut lut,
                                    mut timecode: libc::c_uint)
 -> libc::c_uint {
    let mut hash: libc::c_uint = 0;
    let mut slot_no: slot_no_t = 0;
    let mut slot: *mut slot = 0 as *mut slot;
    hash =
        timecode &
            (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as
                libc::c_uint;
    slot_no = *(*lut).table.offset(hash as isize);
    while slot_no != -(1 as libc::c_int) as libc::c_uint {
        slot = &mut *(*lut).slot.offset(slot_no as isize) as *mut slot;
        if (*slot).timecode == timecode { return slot_no }
        slot_no = (*slot).next
    }
    return -(1 as libc::c_int) as libc::c_uint;
}
