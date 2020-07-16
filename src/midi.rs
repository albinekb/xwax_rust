use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _snd_rawmidi;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn snd_strerror(errnum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn snd_rawmidi_open(in_rmidi: *mut *mut snd_rawmidi_t,
                        out_rmidi: *mut *mut snd_rawmidi_t,
                        name: *const libc::c_char, mode: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn snd_rawmidi_close(rmidi: *mut snd_rawmidi_t) -> libc::c_int;
    #[no_mangle]
    fn snd_rawmidi_poll_descriptors_count(rmidi: *mut snd_rawmidi_t)
     -> libc::c_int;
    #[no_mangle]
    fn snd_rawmidi_poll_descriptors(rmidi: *mut snd_rawmidi_t,
                                    pfds: *mut pollfd, space: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn snd_rawmidi_write(rmidi: *mut snd_rawmidi_t,
                         buffer: *const libc::c_void, size: size_t)
     -> ssize_t;
    #[no_mangle]
    fn snd_rawmidi_read(rmidi: *mut snd_rawmidi_t, buffer: *mut libc::c_void,
                        size: size_t) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type snd_rawmidi_t = _snd_rawmidi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct midi {
    pub in_0: *mut snd_rawmidi_t,
    pub out: *mut snd_rawmidi_t,
}
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
/*
 * Print error code from ALSA
 */
unsafe extern "C" fn alsa_error(mut msg: *const libc::c_char,
                                mut r: libc::c_int) {
    fprintf(stderr, b"ALSA %s: %s\n\x00" as *const u8 as *const libc::c_char,
            msg, snd_strerror(r));
}
#[no_mangle]
pub unsafe extern "C" fn midi_open(mut m: *mut midi,
                                   mut name: *const libc::c_char)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r =
        snd_rawmidi_open(&mut (*m).in_0, &mut (*m).out, name,
                         0x2 as libc::c_int);
    if r < 0 as libc::c_int {
        alsa_error(b"rawmidi_open\x00" as *const u8 as *const libc::c_char,
                   r);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn midi_close(mut m: *mut midi) {
    if snd_rawmidi_close((*m).in_0) < 0 as libc::c_int { abort(); }
    if snd_rawmidi_close((*m).out) < 0 as libc::c_int { abort(); };
}
/*
 * Get the poll descriptors for reading on this MIDI device
 *
 * Pre: len is maximum size of array pe
 * Return: -1 if len is not large enough, otherwise n on success
 * Post: on success, pe is filled with n entries
 */
#[no_mangle]
pub unsafe extern "C" fn midi_pollfds(mut m: *mut midi, mut pe: *mut pollfd,
                                      mut len: size_t) -> ssize_t {
    let mut r: libc::c_int = 0;
    if snd_rawmidi_poll_descriptors_count((*m).in_0) as libc::c_ulong > len {
        return -(1 as libc::c_int) as ssize_t
    }
    r = snd_rawmidi_poll_descriptors((*m).in_0, pe, len as libc::c_uint);
    if r >= 0 as libc::c_int {
    } else {
        __assert_fail(b"r >= 0\x00" as *const u8 as *const libc::c_char,
                      b"midi.c\x00" as *const u8 as *const libc::c_char,
                      70 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 61],
                                                &[libc::c_char; 61]>(b"ssize_t midi_pollfds(struct midi *, struct pollfd *, size_t)\x00")).as_ptr());
    }
    return r as ssize_t;
}
/*
 * Read raw bytes of input
 *
 * Pre: len is maximum size of buffer
 * Return: -1 on error, otherwise n on success
 * Post: on success, buf is filled with n bytes of data
 */
#[no_mangle]
pub unsafe extern "C" fn midi_read(mut m: *mut midi,
                                   mut buf: *mut libc::c_void,
                                   mut len: size_t) -> ssize_t {
    let mut r: libc::c_int = 0;
    r = snd_rawmidi_read((*m).in_0, buf, len) as libc::c_int;
    if r < 0 as libc::c_int {
        if r == -(11 as libc::c_int) { return 0 as libc::c_int as ssize_t }
        alsa_error(b"rawmidi_read\x00" as *const u8 as *const libc::c_char,
                   r);
        return -(1 as libc::c_int) as ssize_t
    }
    return r as ssize_t;
}
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
/*
 * State information for an open, non-blocking MIDI device
 */
#[no_mangle]
pub unsafe extern "C" fn midi_write(mut m: *mut midi,
                                    mut buf: *const libc::c_void,
                                    mut len: size_t) -> ssize_t {
    let mut r: libc::c_int = 0;
    r = snd_rawmidi_write((*m).out, buf, len) as libc::c_int;
    if r < 0 as libc::c_int {
        if r == -(11 as libc::c_int) { return 0 as libc::c_int as ssize_t }
        alsa_error(b"rawmidi_write\x00" as *const u8 as *const libc::c_char,
                   r);
        return -(1 as libc::c_int) as ssize_t
    }
    return r as ssize_t;
}
