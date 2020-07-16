use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sincos(__x: libc::c_double, __sinx: *mut libc::c_double,
              __cosx: *mut libc::c_double);
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn rand() -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type bits_t = libc::c_uint;
/*
 * Calculate the next bit in the LFSR sequence
 */
#[inline]
unsafe extern "C" fn lfsr(mut code: bits_t, mut taps: bits_t) -> bits_t {
    let mut taken: bits_t = 0;
    let mut xrs: libc::c_int = 0;
    taken = code & taps;
    xrs = 0 as libc::c_int;
    while taken != 0 as libc::c_int as libc::c_uint {
        xrs =
            (xrs as
                 libc::c_uint).wrapping_add(taken &
                                                0x1 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int;
        taken >>= 1 as libc::c_int
    }
    return (xrs & 0x1 as libc::c_int) as bits_t;
}
/*
 * LFSR in the forward direction
 */
#[inline]
unsafe extern "C" fn fwd(mut current: bits_t, mut taps: bits_t,
                         mut nbits: libc::c_uint) -> bits_t {
    let mut l: bits_t = 0;
    /* New bits are added at the MSB; shift right by one */
    l = lfsr(current, taps | 0x1 as libc::c_int as libc::c_uint);
    return current >> 1 as libc::c_int |
               l << nbits.wrapping_sub(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn dither() -> libc::c_double {
    return (rand() % 32768 as libc::c_int) as libc::c_double / 32768.0f64 -
               0.5f64;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut s: libc::c_uint = 0;
    let mut b: bits_t = 0;
    let mut length: libc::c_int = 0;
    fputs(b"xwax timecode generator (C) Copyright 2018 Mark Hills <mark@xwax.org>\x00"
              as *const u8 as *const libc::c_char, stderr);
    fputc('\n' as i32, stderr);
    fprintf(stderr,
            b"Generating %d-bit %dHz timecode sampled at %dKhz\n\x00" as
                *const u8 as *const libc::c_char, 22 as libc::c_int,
            4000 as libc::c_int, 44100 as libc::c_int);
    b = 0x1 as libc::c_int as bits_t;
    length = 0 as libc::c_int;
    s = 0 as libc::c_int as libc::c_uint;
    loop  {
        let mut time: libc::c_double = 0.;
        let mut cycle: libc::c_double = 0.;
        let mut angle: libc::c_double = 0.;
        let mut modulate: libc::c_double = 0.;
        let mut x: libc::c_double = 0.;
        let mut y: libc::c_double = 0.;
        let mut c: [libc::c_short; 2] = [0; 2];
        time = s as libc::c_double / 44100 as libc::c_int as libc::c_double;
        cycle = time * 4000 as libc::c_int as libc::c_double;
        angle =
            cycle * 3.14159265358979323846f64 *
                2 as libc::c_int as libc::c_double;
        sincos(angle, &mut x, &mut y);
        /* Modulate the waveform according to the bitstream */
        modulate =
            1.0f64 -
                (-cos(angle) + 1.0f64) * 0.25f64 *
                    (b & 0x1 as libc::c_int as libc::c_uint ==
                         0 as libc::c_int as libc::c_uint) as libc::c_int as
                        libc::c_double;
        x *= modulate;
        y *= modulate;
        /* Write out 16-bit PCM data */
        c[0 as libc::c_int as usize] =
            (-y * 32767 as libc::c_int as libc::c_double * 0.5f64 + dither())
                as libc::c_short;
        c[1 as libc::c_int as usize] =
            (x * 32767 as libc::c_int as libc::c_double * 0.5f64 + dither())
                as libc::c_short;
        fwrite(c.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               2 as libc::c_int as size_t, stdout);
        /* Advance the bitstream if required */
        if cycle as libc::c_int > length {
            if cycle as libc::c_int - length == 1 as libc::c_int {
            } else {
                __assert_fail(b"(int)cycle - length == 1\x00" as *const u8 as
                                  *const libc::c_char,
                              b"mktimecode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              124 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 23],
                                                        &[libc::c_char; 23]>(b"int main(int, char **)\x00")).as_ptr());
            }
            b =
                fwd(b, 0x2 as libc::c_int as bits_t,
                    22 as libc::c_int as libc::c_uint);
            if b == 0x1 as libc::c_int as libc::c_uint { break ; }
            length = cycle as libc::c_int
        }
        s = s.wrapping_add(1)
    }
    fprintf(stderr,
            b"Generated %0.1f seconds of timecode\n\x00" as *const u8 as
                *const libc::c_char,
            length as libc::c_double / 4000 as libc::c_int as libc::c_double);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"    {\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"        .resolution = %d,\n\x00" as *const u8 as
                *const libc::c_char, 4000 as libc::c_int);
    fprintf(stderr,
            b"        .bits = %d,\n\x00" as *const u8 as *const libc::c_char,
            22 as libc::c_int);
    fprintf(stderr,
            b"        .seed = 0x%08x,\n\x00" as *const u8 as
                *const libc::c_char, 0x1 as libc::c_int);
    fprintf(stderr,
            b"        .taps = 0x%08x,\n\x00" as *const u8 as
                *const libc::c_char, 0x2 as libc::c_int);
    fprintf(stderr,
            b"        .length = %d,\n\x00" as *const u8 as
                *const libc::c_char, length);
    fprintf(stderr,
            b"        .safe = %d,\n\x00" as *const u8 as *const libc::c_char,
            if 0 as libc::c_int >
                   length - 4 as libc::c_int * 4000 as libc::c_int {
                0 as libc::c_int
            } else { (length) - 4 as libc::c_int * 4000 as libc::c_int });
    fprintf(stderr, b"    }\n\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
