use ::libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn _exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn vfork() -> libc::c_int;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
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
 * Utility functions for launching external processes
 */
/*
 * A handy read buffer; an equivalent of fread() but for
 * non-blocking file descriptors
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb {
    pub buf: [libc::c_char; 4096],
    pub len: size_t,
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
/* vfork() */
/*
 * Fork a child process, attaching stdout to the given pipe
 *
 * Return: -1 on error, or pid on success
 * Post: on success, *fd is file handle for reading
 */
unsafe extern "C" fn do_fork(mut pp: *mut libc::c_int,
                             mut path: *const libc::c_char,
                             mut argv: *mut *mut libc::c_char) -> pid_t {
    let mut pid: pid_t = 0;
    pid = vfork();
    if pid == -(1 as libc::c_int) {
        perror(b"vfork\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if pid == 0 as libc::c_int {
        /* child */
        if close(*pp.offset(0 as libc::c_int as isize)) != 0 as libc::c_int {
            abort();
        }
        if dup2(*pp.offset(1 as libc::c_int as isize), 1 as libc::c_int) ==
               -(1 as libc::c_int) {
            perror(b"dup2\x00" as *const u8 as *const libc::c_char);
            _exit(1 as libc::c_int);
            /* execv() does not return */
            /* vfork() was used */
        }
        if close(*pp.offset(1 as libc::c_int as isize)) != 0 as libc::c_int {
            abort();
        }
        if execv(path, argv as *const *mut libc::c_char) ==
               -(1 as libc::c_int) {
            perror(path);
            _exit(1 as libc::c_int);
            /* vfork() was used */
        }
        abort();
    }
    if close(*pp.offset(1 as libc::c_int as isize)) != 0 as libc::c_int {
        abort();
    }
    return pid;
}
/*
 * Wrapper on do_fork which uses va_list
 *
 * The caller passes in the pipe for use, rather us handing one
 * back. This is because if the caller wishes to have a non-blocking
 * pipe, then the cleanup is messy if the process has already been
 * forked.
 */
unsafe extern "C" fn vext(mut pp: *mut libc::c_int,
                          mut path: *const libc::c_char,
                          mut arg: *mut libc::c_char,
                          mut ap: ::std::ffi::VaList) -> pid_t {
    let mut args: [*mut libc::c_char; 16] = [0 as *mut libc::c_char; 16];
    let mut n: size_t = 0;
    args[0 as libc::c_int as usize] = arg;
    n = 1 as libc::c_int as size_t;
    loop 
         /* Convert to an array; there's no va_list variant of exec() */
         {
        let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
        x = ap.as_va_list().arg::<*mut libc::c_char>();
        if n <
               (::std::mem::size_of::<[*mut libc::c_char; 16]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong) {
        } else {
            // __assert_fail(b"n < ARRAY_SIZE(args)\x00" as *const u8 as
            //                   *const libc::c_char,
            //               b"external.c\x00" as *const u8 as
            //                   *const libc::c_char,
            //               101 as libc::c_int as libc::c_uint,
            //               (*::std::mem::transmute::<&[u8; 49],
            //                                         &[libc::c_char; 49]>(b"pid_t vext(int *, const char *, char *, va_list)\x00")).as_ptr());
        }
        let fresh0 = n;
        n = n.wrapping_add(1);
        args[fresh0 as usize] = x;
        if x.is_null() { break ; }
    }
    return do_fork(pp, path, args.as_mut_ptr());
}
/*
 * Fork a child process with stdout connected to this process
 * via a pipe
 *
 * Return: PID on success, otherwise -1
 * Post: on success, *fd is file descriptor for reading
 */
#[no_mangle]
pub unsafe extern "C" fn fork_pipe(mut fd: *mut libc::c_int,
                                   mut path: *const libc::c_char,
                                   mut arg: *mut libc::c_char, mut args: ...)
 -> pid_t {
    let mut pp: [libc::c_int; 2] = [0; 2];
    let mut r: pid_t = 0;
    let mut va: ::std::ffi::VaListImpl;
    if pipe(pp.as_mut_ptr()) == -(1 as libc::c_int) {
        perror(b"pipe\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    va = args.clone();
    r = vext(pp.as_mut_ptr(), path, arg, va.as_va_list());
    if r == -(1 as libc::c_int) {
        if close(pp[0 as libc::c_int as usize]) != 0 as libc::c_int {
            abort();
        }
        if close(pp[1 as libc::c_int as usize]) != 0 as libc::c_int {
            abort();
        }
    }
    *fd = pp[0 as libc::c_int as usize];
    return r;
}
/*
 * Make the given file descriptor non-blocking
 *
 * Return: 0 on success, otherwise -1
 * Post: if 0 is returned, file descriptor is non-blocking
 */
unsafe extern "C" fn make_non_blocking(mut fd: libc::c_int) -> libc::c_int {
    if fcntl(fd, 4 as libc::c_int, 0o4000 as libc::c_int) ==
           -(1 as libc::c_int) {
        perror(b"fcntl\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * Fork a child process with stdout connected to this process
 * via a non-blocking pipe
 *
 * Return: PID on success, otherwise -1
 * Post: on success, *fd is non-blocking file descriptor for reading
 */
#[no_mangle]
pub unsafe extern "C" fn fork_pipe_nb(mut fd: *mut libc::c_int,
                                      mut path: *const libc::c_char,
                                      mut arg: *mut libc::c_char,
                                      mut args: ...) -> pid_t {
    let mut pp: [libc::c_int; 2] = [0; 2];
    let mut r: pid_t = 0;
    let mut va: ::std::ffi::VaListImpl;
    if pipe(pp.as_mut_ptr()) == -(1 as libc::c_int) {
        perror(b"pipe\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if !(make_non_blocking(pp[0 as libc::c_int as usize]) ==
             -(1 as libc::c_int)) {
        va = args.clone();
        r = vext(pp.as_mut_ptr(), path, arg, va.as_va_list());
        if r != 0 as libc::c_int {
        } else {
            __assert_fail(b"r != 0\x00" as *const u8 as *const libc::c_char,
                          b"external.c\x00" as *const u8 as
                              *const libc::c_char,
                          188 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 53],
                                                    &[libc::c_char; 53]>(b"pid_t fork_pipe_nb(int *, const char *, char *, ...)\x00")).as_ptr());
        }
        if !(r < 0 as libc::c_int) {
            *fd = pp[0 as libc::c_int as usize];
            return r
        }
    }
    if close(pp[0 as libc::c_int as usize]) != 0 as libc::c_int { abort(); }
    if close(pp[1 as libc::c_int as usize]) != 0 as libc::c_int { abort(); }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rb_reset(mut rb: *mut rb) {
    (*rb).len = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn rb_is_full(mut rb: *const rb) -> bool {
    return (*rb).len ==
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong;
}
/*
 * Read, within reasonable limits (ie. memory or time)
 * from the fd into the buffer
 *
 * Return: -1 on error, 0 on EOF, otherwise the number of bytes added
 */
unsafe extern "C" fn top_up(mut rb: *mut rb, mut fd: libc::c_int) -> ssize_t {
    let mut remain: size_t = 0;
    let mut z: ssize_t = 0;
    if (*rb).len <
           ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong {
    } else {
        __assert_fail(b"rb->len < sizeof rb->buf\x00" as *const u8 as
                          *const libc::c_char,
                      b"external.c\x00" as *const u8 as *const libc::c_char,
                      226 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"ssize_t top_up(struct rb *, int)\x00")).as_ptr());
    }
    remain =
        (::std::mem::size_of::<[libc::c_char; 4096]>() as
             libc::c_ulong).wrapping_sub((*rb).len);
    z =
        read(fd,
             (*rb).buf.as_mut_ptr().offset((*rb).len as isize) as
                 *mut libc::c_void, remain);
    if z == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as ssize_t
    }
    (*rb).len =
        ((*rb).len as libc::c_ulong).wrapping_add(z as libc::c_ulong) as
            size_t as size_t;
    return z;
}
/*
 * Pop the front of the buffer to end-of-line
 *
 * Return: 0 if not found, -1 if not enough memory,
 *    otherwise string length (incl. terminator)
 * Post: if return is > 0, q points to alloc'd string
 */
unsafe extern "C" fn pop(mut rb: *mut rb, mut q: *mut *mut libc::c_char)
 -> ssize_t {
    let mut x: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    x =
        memchr((*rb).buf.as_mut_ptr() as *const libc::c_void, '\n' as i32,
               (*rb).len) as *const libc::c_char;
    if x.is_null() { return 0 as libc::c_int as ssize_t }
    len =
        x.wrapping_offset_from((*rb).buf.as_mut_ptr()) as libc::c_long as
            size_t;
    s = strndup((*rb).buf.as_mut_ptr(), len);
    if s.is_null() { return -(1 as libc::c_int) as ssize_t }
    *q = s;
    /* Simple compact of the buffer. If this is a bottleneck of any
     * kind (unlikely) then a circular buffer should be used */
    memmove((*rb).buf.as_mut_ptr() as *mut libc::c_void,
            x.offset(1 as libc::c_int as isize) as *const libc::c_void,
            (*rb).len.wrapping_sub(len).wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong));
    (*rb).len =
        (*rb).len.wrapping_sub(len).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong);
    return len.wrapping_add(1 as libc::c_int as libc::c_ulong) as ssize_t;
}
/*
 * Read a terminated string from the given file descriptor via
 * the buffer.
 *
 * Handles non-blocking file descriptors too. If fd is non-blocking,
 * then the semantics are the same as a non-blocking read() --
 * ie. EAGAIN may be returned as an error.
 *
 * Return: 0 on EOF, or -1 on error
 * Post: if -1 is returned, errno is set accordingly
 */
#[no_mangle]
pub unsafe extern "C" fn get_line(mut fd: libc::c_int, mut rb: *mut rb,
                                  mut string: *mut *mut libc::c_char)
 -> ssize_t {
    let mut y: ssize_t = 0; /* true EOF: no more data and empty buffer */
    let mut z: ssize_t = 0;
    y = top_up(rb, fd);
    if y < 0 as libc::c_int as libc::c_long { return y }
    z = pop(rb, string);
    if z != 0 as libc::c_int as libc::c_long { return z }
    if rb_is_full(rb) {
        *__errno_location() = 105 as libc::c_int
    } else if y > 0 as libc::c_int as libc::c_long {
        *__errno_location() = 11 as libc::c_int
    } else { return 0 as libc::c_int as ssize_t }
    return -(1 as libc::c_int) as ssize_t;
}
