use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _snd_pcm_hw_params;
    pub type _snd_pcm;
    pub type player;
    pub type timecoder;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn snd_pcm_open(pcm: *mut *mut snd_pcm_t, name: *const libc::c_char,
                    stream: snd_pcm_stream_t, mode: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_close(pcm: *mut snd_pcm_t) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_poll_descriptors_count(pcm: *mut snd_pcm_t) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_poll_descriptors(pcm: *mut snd_pcm_t, pfds: *mut pollfd,
                                space: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_poll_descriptors_revents(pcm: *mut snd_pcm_t,
                                        pfds: *mut pollfd, nfds: libc::c_uint,
                                        revents: *mut libc::c_ushort)
     -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_hw_params(pcm: *mut snd_pcm_t,
                         params: *mut snd_pcm_hw_params_t) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_prepare(pcm: *mut snd_pcm_t) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_start(pcm: *mut snd_pcm_t) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_writei(pcm: *mut snd_pcm_t, buffer: *const libc::c_void,
                      size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    #[no_mangle]
    fn snd_pcm_readi(pcm: *mut snd_pcm_t, buffer: *mut libc::c_void,
                     size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    #[no_mangle]
    fn snd_pcm_hw_params_any(pcm: *mut snd_pcm_t,
                             params: *mut snd_pcm_hw_params_t) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_hw_params_sizeof() -> size_t;
    #[no_mangle]
    fn snd_pcm_hw_params_set_access(pcm: *mut snd_pcm_t,
                                    params: *mut snd_pcm_hw_params_t,
                                    _access: snd_pcm_access_t) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_hw_params_set_format(pcm: *mut snd_pcm_t,
                                    params: *mut snd_pcm_hw_params_t,
                                    val: snd_pcm_format_t) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_hw_params_set_channels(pcm: *mut snd_pcm_t,
                                      params: *mut snd_pcm_hw_params_t,
                                      val: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_hw_params_set_rate(pcm: *mut snd_pcm_t,
                                  params: *mut snd_pcm_hw_params_t,
                                  val: libc::c_uint, dir: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_hw_params_get_period_size(params: *const snd_pcm_hw_params_t,
                                         frames: *mut snd_pcm_uframes_t,
                                         dir: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_hw_params_set_periods_min(pcm: *mut snd_pcm_t,
                                         params: *mut snd_pcm_hw_params_t,
                                         val: *mut libc::c_uint,
                                         dir: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn snd_pcm_hw_params_set_buffer_time_near(pcm: *mut snd_pcm_t,
                                              params:
                                                  *mut snd_pcm_hw_params_t,
                                              val: *mut libc::c_uint,
                                              dir: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn snd_strerror(errnum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn snd_config_update_free_global() -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn device_init(dv: *mut device, ops: *mut device_ops);
    #[no_mangle]
    fn device_submit(dv: *mut device, pcm: *mut libc::c_short, npcm: size_t);
    #[no_mangle]
    fn device_collect(dv: *mut device, pcm: *mut libc::c_short, npcm: size_t);
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
pub type snd_pcm_hw_params_t = _snd_pcm_hw_params;
pub type _snd_pcm_stream = libc::c_uint;
pub const SND_PCM_STREAM_LAST: _snd_pcm_stream = 1;
pub const SND_PCM_STREAM_CAPTURE: _snd_pcm_stream = 1;
pub const SND_PCM_STREAM_PLAYBACK: _snd_pcm_stream = 0;
pub type snd_pcm_stream_t = _snd_pcm_stream;
pub type _snd_pcm_access = libc::c_uint;
pub const SND_PCM_ACCESS_LAST: _snd_pcm_access = 4;
pub const SND_PCM_ACCESS_RW_NONINTERLEAVED: _snd_pcm_access = 4;
pub const SND_PCM_ACCESS_RW_INTERLEAVED: _snd_pcm_access = 3;
pub const SND_PCM_ACCESS_MMAP_COMPLEX: _snd_pcm_access = 2;
pub const SND_PCM_ACCESS_MMAP_NONINTERLEAVED: _snd_pcm_access = 1;
pub const SND_PCM_ACCESS_MMAP_INTERLEAVED: _snd_pcm_access = 0;
pub type snd_pcm_access_t = _snd_pcm_access;
pub type _snd_pcm_format = libc::c_int;
pub const SND_PCM_FORMAT_U20: _snd_pcm_format = 27;
pub const SND_PCM_FORMAT_S20: _snd_pcm_format = 25;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME: _snd_pcm_format = 18;
pub const SND_PCM_FORMAT_FLOAT64: _snd_pcm_format = 16;
pub const SND_PCM_FORMAT_FLOAT: _snd_pcm_format = 14;
pub const SND_PCM_FORMAT_U32: _snd_pcm_format = 12;
pub const SND_PCM_FORMAT_S32: _snd_pcm_format = 10;
pub const SND_PCM_FORMAT_U24: _snd_pcm_format = 8;
pub const SND_PCM_FORMAT_S24: _snd_pcm_format = 6;
pub const SND_PCM_FORMAT_U16: _snd_pcm_format = 4;
pub const SND_PCM_FORMAT_S16: _snd_pcm_format = 2;
pub const SND_PCM_FORMAT_LAST: _snd_pcm_format = 52;
pub const SND_PCM_FORMAT_DSD_U32_BE: _snd_pcm_format = 52;
pub const SND_PCM_FORMAT_DSD_U16_BE: _snd_pcm_format = 51;
pub const SND_PCM_FORMAT_DSD_U32_LE: _snd_pcm_format = 50;
pub const SND_PCM_FORMAT_DSD_U16_LE: _snd_pcm_format = 49;
pub const SND_PCM_FORMAT_DSD_U8: _snd_pcm_format = 48;
pub const SND_PCM_FORMAT_G723_40_1B: _snd_pcm_format = 47;
pub const SND_PCM_FORMAT_G723_40: _snd_pcm_format = 46;
pub const SND_PCM_FORMAT_G723_24_1B: _snd_pcm_format = 45;
pub const SND_PCM_FORMAT_G723_24: _snd_pcm_format = 44;
pub const SND_PCM_FORMAT_U18_3BE: _snd_pcm_format = 43;
pub const SND_PCM_FORMAT_U18_3LE: _snd_pcm_format = 42;
pub const SND_PCM_FORMAT_S18_3BE: _snd_pcm_format = 41;
pub const SND_PCM_FORMAT_S18_3LE: _snd_pcm_format = 40;
pub const SND_PCM_FORMAT_U20_3BE: _snd_pcm_format = 39;
pub const SND_PCM_FORMAT_U20_3LE: _snd_pcm_format = 38;
pub const SND_PCM_FORMAT_S20_3BE: _snd_pcm_format = 37;
pub const SND_PCM_FORMAT_S20_3LE: _snd_pcm_format = 36;
pub const SND_PCM_FORMAT_U24_3BE: _snd_pcm_format = 35;
pub const SND_PCM_FORMAT_U24_3LE: _snd_pcm_format = 34;
pub const SND_PCM_FORMAT_S24_3BE: _snd_pcm_format = 33;
pub const SND_PCM_FORMAT_S24_3LE: _snd_pcm_format = 32;
pub const SND_PCM_FORMAT_SPECIAL: _snd_pcm_format = 31;
pub const SND_PCM_FORMAT_U20_BE: _snd_pcm_format = 28;
pub const SND_PCM_FORMAT_U20_LE: _snd_pcm_format = 27;
pub const SND_PCM_FORMAT_S20_BE: _snd_pcm_format = 26;
pub const SND_PCM_FORMAT_S20_LE: _snd_pcm_format = 25;
pub const SND_PCM_FORMAT_GSM: _snd_pcm_format = 24;
pub const SND_PCM_FORMAT_MPEG: _snd_pcm_format = 23;
pub const SND_PCM_FORMAT_IMA_ADPCM: _snd_pcm_format = 22;
pub const SND_PCM_FORMAT_A_LAW: _snd_pcm_format = 21;
pub const SND_PCM_FORMAT_MU_LAW: _snd_pcm_format = 20;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME_BE: _snd_pcm_format = 19;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME_LE: _snd_pcm_format = 18;
pub const SND_PCM_FORMAT_FLOAT64_BE: _snd_pcm_format = 17;
pub const SND_PCM_FORMAT_FLOAT64_LE: _snd_pcm_format = 16;
pub const SND_PCM_FORMAT_FLOAT_BE: _snd_pcm_format = 15;
pub const SND_PCM_FORMAT_FLOAT_LE: _snd_pcm_format = 14;
pub const SND_PCM_FORMAT_U32_BE: _snd_pcm_format = 13;
pub const SND_PCM_FORMAT_U32_LE: _snd_pcm_format = 12;
pub const SND_PCM_FORMAT_S32_BE: _snd_pcm_format = 11;
pub const SND_PCM_FORMAT_S32_LE: _snd_pcm_format = 10;
pub const SND_PCM_FORMAT_U24_BE: _snd_pcm_format = 9;
pub const SND_PCM_FORMAT_U24_LE: _snd_pcm_format = 8;
pub const SND_PCM_FORMAT_S24_BE: _snd_pcm_format = 7;
pub const SND_PCM_FORMAT_S24_LE: _snd_pcm_format = 6;
pub const SND_PCM_FORMAT_U16_BE: _snd_pcm_format = 5;
pub const SND_PCM_FORMAT_U16_LE: _snd_pcm_format = 4;
pub const SND_PCM_FORMAT_S16_BE: _snd_pcm_format = 3;
pub const SND_PCM_FORMAT_S16_LE: _snd_pcm_format = 2;
pub const SND_PCM_FORMAT_U8: _snd_pcm_format = 1;
pub const SND_PCM_FORMAT_S8: _snd_pcm_format = 0;
pub const SND_PCM_FORMAT_UNKNOWN: _snd_pcm_format = -1;
pub type snd_pcm_format_t = _snd_pcm_format;
pub type snd_pcm_uframes_t = libc::c_ulong;
pub type snd_pcm_sframes_t = libc::c_long;
pub type snd_pcm_t = _snd_pcm;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub fault: bool,
    pub local: *mut libc::c_void,
    pub ops: *mut device_ops,
    pub timecoder: *mut timecoder,
    pub player: *mut player,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device_ops {
    pub pollfds: Option<unsafe extern "C" fn(_: *mut device, _: *mut pollfd,
                                             _: size_t) -> ssize_t>,
    pub handle: Option<unsafe extern "C" fn(_: *mut device) -> libc::c_int>,
    pub sample_rate: Option<unsafe extern "C" fn(_: *mut device)
                                -> libc::c_uint>,
    pub start: Option<unsafe extern "C" fn(_: *mut device) -> ()>,
    pub stop: Option<unsafe extern "C" fn(_: *mut device) -> ()>,
    pub clear: Option<unsafe extern "C" fn(_: *mut device) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alsa {
    pub capture: alsa_pcm,
    pub playback: alsa_pcm,
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
/* This structure doesn't have corresponding functions to be an
 * abstraction of the ALSA calls; it is merely a container for these
 * variables. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alsa_pcm {
    pub pcm: *mut snd_pcm_t,
    pub pe: *mut pollfd,
    pub pe_count: size_t,
    pub buf: *mut libc::c_short,
    pub period: snd_pcm_uframes_t,
    pub rate: libc::c_int,
}
unsafe extern "C" fn alsa_error(mut msg: *const libc::c_char,
                                mut r: libc::c_int) {
    fprintf(stderr, b"ALSA %s: %s\n\x00" as *const u8 as *const libc::c_char,
            msg, snd_strerror(r)); /* microseconds */
}
unsafe extern "C" fn chk(mut s: *const libc::c_char, mut r: libc::c_int)
 -> bool {
    if r < 0 as libc::c_int {
        alsa_error(s, r); /* double buffering */
        return 0 as libc::c_int != 0
    } else { return 1 as libc::c_int != 0 };
}
unsafe extern "C" fn pcm_open(mut alsa: *mut alsa_pcm,
                              mut device_name: *const libc::c_char,
                              mut stream: snd_pcm_stream_t,
                              mut rate: libc::c_int,
                              mut buffer_time: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut p: libc::c_uint = 0;
    let mut bytes: size_t = 0;
    let mut hw_params: *mut snd_pcm_hw_params_t =
        0 as *mut snd_pcm_hw_params_t;
    r =
        snd_pcm_open(&mut (*alsa).pcm, device_name, stream,
                     0x1 as libc::c_int);
    if !chk(b"open\x00" as *const u8 as *const libc::c_char, r) {
        return -(1 as libc::c_int)
    }
    let mut fresh0 =
        ::std::vec::from_elem(0, snd_pcm_hw_params_sizeof() as usize);
    hw_params = fresh0.as_mut_ptr() as *mut snd_pcm_hw_params_t;
    memset(hw_params as *mut libc::c_void, 0 as libc::c_int,
           snd_pcm_hw_params_sizeof());
    r = snd_pcm_hw_params_any((*alsa).pcm, hw_params);
    if !chk(b"hw_params_any\x00" as *const u8 as *const libc::c_char, r) {
        return -(1 as libc::c_int)
    }
    r =
        snd_pcm_hw_params_set_access((*alsa).pcm, hw_params,
                                     SND_PCM_ACCESS_RW_INTERLEAVED);
    if !chk(b"hw_params_set_access\x00" as *const u8 as *const libc::c_char,
            r) {
        return -(1 as libc::c_int)
    }
    r =
        snd_pcm_hw_params_set_format((*alsa).pcm, hw_params,
                                     SND_PCM_FORMAT_S16);
    if !chk(b"hw_params_set_format\x00" as *const u8 as *const libc::c_char,
            r) {
        fprintf(stderr,
                b"16-bit signed format is not available. You may need to use a \'plughw\' device.\n\x00"
                    as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    r =
        snd_pcm_hw_params_set_rate((*alsa).pcm, hw_params,
                                   rate as libc::c_uint, 0 as libc::c_int);
    if !chk(b"hw_params_set_rate\x00" as *const u8 as *const libc::c_char, r)
       {
        fprintf(stderr,
                b"%dHz sample rate not available. You may need to use a \'plughw\' device.\n\x00"
                    as *const u8 as *const libc::c_char, rate);
        return -(1 as libc::c_int)
    }
    (*alsa).rate = rate;
    r =
        snd_pcm_hw_params_set_channels((*alsa).pcm, hw_params,
                                       2 as libc::c_int as libc::c_uint);
    if !chk(b"hw_params_set_channels\x00" as *const u8 as *const libc::c_char,
            r) {
        fprintf(stderr,
                b"%d channel audio not available on this device.\n\x00" as
                    *const u8 as *const libc::c_char, 2 as libc::c_int);
        return -(1 as libc::c_int)
    }
    p = (buffer_time * 1000 as libc::c_int) as libc::c_uint;
    dir = -(1 as libc::c_int);
    r =
        snd_pcm_hw_params_set_buffer_time_near((*alsa).pcm, hw_params, &mut p,
                                               &mut dir);
    if !chk(b"hw_params_set_buffer_time_near\x00" as *const u8 as
                *const libc::c_char, r) {
        fprintf(stderr,
                b"Buffer of %dms may be too small for this hardware.\n\x00" as
                    *const u8 as *const libc::c_char, buffer_time);
        return -(1 as libc::c_int)
    }
    p = 2 as libc::c_int as libc::c_uint;
    dir = 1 as libc::c_int;
    r =
        snd_pcm_hw_params_set_periods_min((*alsa).pcm, hw_params, &mut p,
                                          &mut dir);
    if !chk(b"hw_params_set_periods_min\x00" as *const u8 as
                *const libc::c_char, r) {
        fprintf(stderr,
                b"Buffer of %dms may be too small for this hardware.\n\x00" as
                    *const u8 as *const libc::c_char, buffer_time);
        return -(1 as libc::c_int)
    }
    r = snd_pcm_hw_params((*alsa).pcm, hw_params);
    if !chk(b"hw_params\x00" as *const u8 as *const libc::c_char, r) {
        return -(1 as libc::c_int)
    }
    r =
        snd_pcm_hw_params_get_period_size(hw_params, &mut (*alsa).period,
                                          &mut dir);
    if !chk(b"get_period_size\x00" as *const u8 as *const libc::c_char, r) {
        return -(1 as libc::c_int)
    }
    bytes =
        (*alsa).period.wrapping_mul(2 as libc::c_int as
                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                                        as
                                                                        libc::c_ulong);
    (*alsa).buf = malloc(bytes) as *mut libc::c_short;
    if (*alsa).buf.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    /* snd_pcm_readi() returns uninitialised memory on first call,
     * possibly caused by premature POLLIN. Keep valgrind happy. */
    memset((*alsa).buf as *mut libc::c_void, 0 as libc::c_int, bytes);
    return 0 as libc::c_int;
}
unsafe extern "C" fn pcm_close(mut alsa: *mut alsa_pcm) {
    if snd_pcm_close((*alsa).pcm) < 0 as libc::c_int { abort(); }
    free((*alsa).buf as *mut libc::c_void);
}
unsafe extern "C" fn pcm_pollfds(mut alsa: *mut alsa_pcm, mut pe: *mut pollfd,
                                 mut z: size_t) -> ssize_t {
    let mut r: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    count = snd_pcm_poll_descriptors_count((*alsa).pcm);
    if count as libc::c_ulong > z { return -(1 as libc::c_int) as ssize_t }
    if count == 0 as libc::c_int {
        (*alsa).pe = 0 as *mut pollfd
    } else {
        r = snd_pcm_poll_descriptors((*alsa).pcm, pe, count as libc::c_uint);
        if r < 0 as libc::c_int {
            alsa_error(b"poll_descriptors\x00" as *const u8 as
                           *const libc::c_char, r);
            return -(1 as libc::c_int) as ssize_t
        }
        (*alsa).pe = pe
    }
    (*alsa).pe_count = count as size_t;
    return count as ssize_t;
}
unsafe extern "C" fn pcm_revents(mut alsa: *mut alsa_pcm,
                                 mut revents: *mut libc::c_ushort)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r =
        snd_pcm_poll_descriptors_revents((*alsa).pcm, (*alsa).pe,
                                         (*alsa).pe_count as libc::c_uint,
                                         revents);
    if r < 0 as libc::c_int {
        alsa_error(b"poll_descriptors_revents\x00" as *const u8 as
                       *const libc::c_char, r);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/* Start the audio device capture and playback */
unsafe extern "C" fn start(mut dv: *mut device) {
    let mut alsa: *mut alsa = (*dv).local as *mut alsa;
    if snd_pcm_start((*alsa).capture.pcm) < 0 as libc::c_int { abort(); };
}
/* Register this device's interest in a set of pollfd file
 * descriptors */
unsafe extern "C" fn pollfds(mut dv: *mut device, mut pe: *mut pollfd,
                             mut z: size_t) -> ssize_t {
    let mut total: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut alsa: *mut alsa = (*dv).local as *mut alsa;
    total = 0 as libc::c_int;
    r = pcm_pollfds(&mut (*alsa).capture, pe, z) as libc::c_int;
    if r < 0 as libc::c_int { return -(1 as libc::c_int) as ssize_t }
    pe = pe.offset(r as isize);
    z =
        (z as libc::c_ulong).wrapping_sub(r as libc::c_ulong) as size_t as
            size_t;
    total += r;
    r = pcm_pollfds(&mut (*alsa).playback, pe, z) as libc::c_int;
    if r < 0 as libc::c_int { return -(1 as libc::c_int) as ssize_t }
    total += r;
    return total as ssize_t;
}
/* Collect audio from the player and push it into the device's buffer,
 * for playback */
unsafe extern "C" fn playback(mut dv: *mut device) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut alsa: *mut alsa = (*dv).local as *mut alsa;
    device_collect(dv, (*alsa).playback.buf, (*alsa).playback.period);
    r =
        snd_pcm_writei((*alsa).playback.pcm,
                       (*alsa).playback.buf as *const libc::c_void,
                       (*alsa).playback.period) as libc::c_int;
    if r < 0 as libc::c_int { return r }
    if (r as libc::c_ulong) < (*alsa).playback.period {
        fprintf(stderr,
                b"alsa: playback underrun %d/%ld.\n\x00" as *const u8 as
                    *const libc::c_char, r, (*alsa).playback.period);
    }
    return 0 as libc::c_int;
}
/* Pull audio from the device's buffer for capture, and pass it
 * through to the timecoder */
unsafe extern "C" fn capture(mut dv: *mut device) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut alsa: *mut alsa = (*dv).local as *mut alsa;
    r =
        snd_pcm_readi((*alsa).capture.pcm,
                      (*alsa).capture.buf as *mut libc::c_void,
                      (*alsa).capture.period) as libc::c_int;
    if r < 0 as libc::c_int { return r }
    if (r as libc::c_ulong) < (*alsa).capture.period {
        fprintf(stderr,
                b"alsa: capture underrun %d/%ld.\n\x00" as *const u8 as
                    *const libc::c_char, r, (*alsa).capture.period);
    }
    device_submit(dv, (*alsa).capture.buf, r as size_t);
    return 0 as libc::c_int;
}
/* After poll() has returned, instruct a device to do all it can at
 * the present time. Return zero if success, otherwise -1 */
unsafe extern "C" fn handle(mut dv: *mut device) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut revents: libc::c_ushort = 0;
    let mut alsa: *mut alsa = (*dv).local as *mut alsa;
    /* Check input buffer for timecode capture */
    r = pcm_revents(&mut (*alsa).capture, &mut revents);
    if r < 0 as libc::c_int { return -(1 as libc::c_int) }
    if revents as libc::c_int & 0x1 as libc::c_int != 0 {
        r = capture(dv);
        if r < 0 as libc::c_int {
            if r == -(32 as libc::c_int) {
                fputs(b"ALSA: capture xrun.\n\x00" as *const u8 as
                          *const libc::c_char, stderr);
                r = snd_pcm_prepare((*alsa).capture.pcm);
                if r < 0 as libc::c_int {
                    alsa_error(b"prepare\x00" as *const u8 as
                                   *const libc::c_char, r);
                    return -(1 as libc::c_int)
                }
                r = snd_pcm_start((*alsa).capture.pcm);
                if r < 0 as libc::c_int {
                    alsa_error(b"start\x00" as *const u8 as
                                   *const libc::c_char, r);
                    return -(1 as libc::c_int)
                }
            } else {
                alsa_error(b"capture\x00" as *const u8 as *const libc::c_char,
                           r);
                return -(1 as libc::c_int)
            }
        }
    }
    /* Check the output buffer for playback */
    r = pcm_revents(&mut (*alsa).playback, &mut revents);
    if r < 0 as libc::c_int { return -(1 as libc::c_int) }
    if revents as libc::c_int & 0x4 as libc::c_int != 0 {
        r = playback(dv);
        if r < 0 as libc::c_int {
            if r == -(32 as libc::c_int) {
                fputs(b"ALSA: playback xrun.\n\x00" as *const u8 as
                          *const libc::c_char, stderr);
                r = snd_pcm_prepare((*alsa).playback.pcm);
                if r < 0 as libc::c_int {
                    alsa_error(b"prepare\x00" as *const u8 as
                                   *const libc::c_char, r);
                    return -(1 as libc::c_int)
                }
                /* The device starts when data is written. POLLOUT
                 * events are generated in prepared state. */
            } else {
                alsa_error(b"playback\x00" as *const u8 as
                               *const libc::c_char, r);
                return -(1 as libc::c_int)
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sample_rate(mut dv: *mut device) -> libc::c_uint {
    let mut alsa: *mut alsa = (*dv).local as *mut alsa;
    return (*alsa).capture.rate as libc::c_uint;
}
/* Close ALSA device and clear any allocations */
unsafe extern "C" fn clear(mut dv: *mut device) {
    let mut alsa: *mut alsa = (*dv).local as *mut alsa;
    pcm_close(&mut (*alsa).capture);
    pcm_close(&mut (*alsa).playback);
    free((*dv).local);
}
static mut alsa_ops: device_ops =
    unsafe {
        {
            let mut init =
                device_ops{pollfds:
                               Some(pollfds as
                                        unsafe extern "C" fn(_: *mut device,
                                                             _: *mut pollfd,
                                                             _: size_t)
                                            -> ssize_t),
                           handle:
                               Some(handle as
                                        unsafe extern "C" fn(_: *mut device)
                                            -> libc::c_int),
                           sample_rate:
                               Some(sample_rate as
                                        unsafe extern "C" fn(_: *mut device)
                                            -> libc::c_uint),
                           start:
                               Some(start as
                                        unsafe extern "C" fn(_: *mut device)
                                            -> ()),
                           stop: None,
                           clear:
                               Some(clear as
                                        unsafe extern "C" fn(_: *mut device)
                                            -> ()),};
            init
        }
    };
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
/* Open ALSA device. Do not operate on audio until device_start() */
#[no_mangle]
pub unsafe extern "C" fn alsa_init(mut dv: *mut device,
                                   mut device_name: *const libc::c_char,
                                   mut rate: libc::c_int,
                                   mut buffer_time: libc::c_int)
 -> libc::c_int {
    let mut alsa: *mut alsa = 0 as *mut alsa;
    alsa =
        malloc(::std::mem::size_of::<alsa>() as libc::c_ulong) as *mut alsa;
    if alsa.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if pcm_open(&mut (*alsa).capture, device_name, SND_PCM_STREAM_CAPTURE,
                rate, buffer_time) < 0 as libc::c_int {
        fputs(b"Failed to open device for capture.\n\x00" as *const u8 as
                  *const libc::c_char, stderr);
    } else if pcm_open(&mut (*alsa).playback, device_name,
                       SND_PCM_STREAM_PLAYBACK, rate, buffer_time) <
                  0 as libc::c_int {
        fputs(b"Failed to open device for playback.\n\x00" as *const u8 as
                  *const libc::c_char, stderr);
        pcm_close(&mut (*alsa).capture);
    } else {
        device_init(dv, &mut alsa_ops);
        (*dv).local = alsa as *mut libc::c_void;
        return 0 as libc::c_int
    }
    free(alsa as *mut libc::c_void);
    return -(1 as libc::c_int);
}
/* ALSA caches information when devices are open. Provide a call
 * to clear these caches so that valgrind output is clean. */
#[no_mangle]
pub unsafe extern "C" fn alsa_clear_config_cache() {
    let mut r: libc::c_int = 0;
    r = snd_config_update_free_global();
    if r < 0 as libc::c_int {
        alsa_error(b"config_update_free_global\x00" as *const u8 as
                       *const libc::c_char, r);
    };
}
