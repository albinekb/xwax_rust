use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    pub type deck;
    pub type player;
    pub type timecoder;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn controller_handle(c: *mut controller);
    #[no_mangle]
    fn controller_pollfds(c: *mut controller, pe: *mut pollfd, z: size_t)
     -> ssize_t;
    #[no_mangle]
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn device_start(dv: *mut device);
    #[no_mangle]
    fn device_stop(dv: *mut device);
    #[no_mangle]
    fn device_pollfds(dv: *mut device, pe: *mut pollfd, z: size_t) -> ssize_t;
    #[no_mangle]
    fn device_handle(dv: *mut device);
    #[no_mangle]
    fn sched_getparam(__pid: __pid_t, __param: *mut sched_param)
     -> libc::c_int;
    #[no_mangle]
    fn sched_setscheduler(__pid: __pid_t, __policy: libc::c_int,
                          __param: *const sched_param) -> libc::c_int;
    #[no_mangle]
    fn sched_get_priority_max(__algorithm: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn sem_post(__sem: *mut sem_t) -> libc::c_int;
    #[no_mangle]
    fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
    #[no_mangle]
    fn sem_destroy(__sem: *mut sem_t) -> libc::c_int;
    #[no_mangle]
    fn sem_init(__sem: *mut sem_t, __pshared: libc::c_int,
                __value: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn thread_to_realtime();
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 64],
    pub __align: libc::c_long,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rt {
    pub ph: pthread_t,
    pub sem: sem_t,
    pub finished: bool,
    pub priority: libc::c_int,
    pub ndv: size_t,
    pub dv: [*mut device; 3],
    pub nctl: size_t,
    pub ctl: [*mut controller; 3],
    pub npt: size_t,
    pub pt: [pollfd; 32],
}
/*
 * Base state of a 'controller', which is a MIDI controller or HID
 * device used to control the program
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct controller {
    pub fault: bool,
    pub local: *mut libc::c_void,
    pub ops: *mut controller_ops,
}
/*
 * Functions which must be implemented for a controller
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct controller_ops {
    pub add_deck: Option<unsafe extern "C" fn(_: *mut controller,
                                              _: *mut deck) -> libc::c_int>,
    pub pollfds: Option<unsafe extern "C" fn(_: *mut controller,
                                             _: *mut pollfd, _: size_t)
                            -> ssize_t>,
    pub realtime: Option<unsafe extern "C" fn(_: *mut controller)
                             -> libc::c_int>,
    pub clear: Option<unsafe extern "C" fn(_: *mut controller) -> ()>,
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
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
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
 * Raise the priority of the current thread
 *
 * Return: -1 if priority could not be satisfactorily raised, otherwise 0
 */
unsafe extern "C" fn raise_priority(mut priority: libc::c_int)
 -> libc::c_int {
    let mut max_pri: libc::c_int = 0;
    let mut sp: sched_param = sched_param{sched_priority: 0,};
    max_pri = sched_get_priority_max(1 as libc::c_int);
    if priority > max_pri {
        fprintf(stderr,
                b"Invalid scheduling priority (maximum %d).\n\x00" as
                    *const u8 as *const libc::c_char, max_pri);
        return -(1 as libc::c_int)
    }
    if sched_getparam(0 as libc::c_int, &mut sp) != 0 {
        perror(b"sched_getparam\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    sp.sched_priority = priority;
    if sched_setscheduler(0 as libc::c_int, 1 as libc::c_int, &mut sp) != 0 {
        perror(b"sched_setscheduler\x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"Failed to get realtime priorities\n\x00" as *const u8 as
                    *const libc::c_char);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * The realtime thread
 */
unsafe extern "C" fn rt_main(mut rt: *mut rt) {
    let mut r: libc::c_int = 0; /* under our control; see sem_post(3) */
    let mut n: size_t = 0;
    thread_to_realtime();
    if (*rt).priority != 0 as libc::c_int {
        if raise_priority((*rt).priority) == -(1 as libc::c_int) {
            (*rt).finished = 1 as libc::c_int != 0
        }
    }
    if sem_post(&mut (*rt).sem) == -(1 as libc::c_int) { abort(); }
    while !(*rt).finished {
        r = poll((*rt).pt.as_mut_ptr(), (*rt).npt, -(1 as libc::c_int));
        if r == -(1 as libc::c_int) {
            if *__errno_location() == 4 as libc::c_int { continue ; }
            perror(b"poll\x00" as *const u8 as *const libc::c_char);
            abort();
        } else {
            n = 0 as libc::c_int as size_t;
            while n < (*rt).nctl {
                controller_handle((*rt).ctl[n as usize]);
                n = n.wrapping_add(1)
            }
            n = 0 as libc::c_int as size_t;
            while n < (*rt).ndv {
                device_handle((*rt).dv[n as usize]);
                n = n.wrapping_add(1)
            }
        }
    };
}
unsafe extern "C" fn launch(mut p: *mut libc::c_void) -> *mut libc::c_void {
    rt_main(p as *mut rt);
    return 0 as *mut libc::c_void;
}
/*
 * Initialise state of realtime handler
 */
#[no_mangle]
pub unsafe extern "C" fn rt_init(mut rt: *mut rt) {
    (*rt).finished = 0 as libc::c_int != 0;
    (*rt).ndv = 0 as libc::c_int as size_t;
    (*rt).nctl = 0 as libc::c_int as size_t;
    (*rt).npt = 0 as libc::c_int as size_t;
}
/*
 * Clear resources associated with the realtime handler
 */
#[no_mangle]
pub unsafe extern "C" fn rt_clear(mut rt: *mut rt) { }
/*
 * Add a device to this realtime handler
 *
 * Return: -1 if the device could not be added, otherwise 0
 * Post: if 0 is returned the device is added
 */
#[no_mangle]
pub unsafe extern "C" fn rt_add_device(mut rt: *mut rt, mut dv: *mut device)
 -> libc::c_int {
    let mut z: ssize_t = 0;
    if (*rt).ndv ==
           (::std::mem::size_of::<[*mut device; 3]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut device>()
                                                as libc::c_ulong) {
        fprintf(stderr,
                b"Too many audio devices\n\x00" as *const u8 as
                    *const libc::c_char);
        return -(1 as libc::c_int)
    }
    /* The requested poll events never change, so populate the poll
     * entry table before entering the realtime thread */
    z =
        device_pollfds(dv,
                       &mut *(*rt).pt.as_mut_ptr().offset((*rt).npt as isize),
                       (::std::mem::size_of::<[pollfd; 32]>() as
                            libc::c_ulong).wrapping_sub((*rt).npt));
    if z == -(1 as libc::c_int) as libc::c_long {
        fprintf(stderr,
                b"Device failed to return file descriptors.\n\x00" as
                    *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    (*rt).npt =
        ((*rt).npt as libc::c_ulong).wrapping_add(z as libc::c_ulong) as
            size_t as size_t;
    (*rt).dv[(*rt).ndv as usize] = dv;
    (*rt).ndv = (*rt).ndv.wrapping_add(1);
    return 0 as libc::c_int;
}
/*
 * Add a controller to the realtime handler
 *
 * Return: -1 if the device could not be added, otherwise 0
 */
#[no_mangle]
pub unsafe extern "C" fn rt_add_controller(mut rt: *mut rt,
                                           mut c: *mut controller)
 -> libc::c_int {
    let mut z: ssize_t = 0;
    if (*rt).nctl ==
           (::std::mem::size_of::<[*mut controller; 3]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut controller>()
                                                as libc::c_ulong) {
        fprintf(stderr,
                b"Too many controllers\n\x00" as *const u8 as
                    *const libc::c_char);
        return -(1 as libc::c_int)
    }
    /* Similar to adding a PCM device */
    z =
        controller_pollfds(c,
                           &mut *(*rt).pt.as_mut_ptr().offset((*rt).npt as
                                                                  isize),
                           (::std::mem::size_of::<[pollfd; 32]>() as
                                libc::c_ulong).wrapping_sub((*rt).npt));
    if z == -(1 as libc::c_int) as libc::c_long {
        fprintf(stderr,
                b"Controller failed to return file descriptors.\n\x00" as
                    *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    (*rt).npt =
        ((*rt).npt as libc::c_ulong).wrapping_add(z as libc::c_ulong) as
            size_t as size_t;
    let fresh0 = (*rt).nctl;
    (*rt).nctl = (*rt).nctl.wrapping_add(1);
    (*rt).ctl[fresh0 as usize] = c;
    return 0 as libc::c_int;
}
/*
 * Start realtime handling of the given devices
 *
 * This forks the realtime thread if it is required (eg. ALSA). Some
 * devices (eg. JACK) start their own thread.
 *
 * Return: -1 on error, otherwise 0
 */
#[no_mangle]
pub unsafe extern "C" fn rt_start(mut rt: *mut rt, mut priority: libc::c_int)
 -> libc::c_int {
    let mut n: size_t = 0;
    if priority >= 0 as libc::c_int {
    } else {
        __assert_fail(b"priority >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"realtime.c\x00" as *const u8 as *const libc::c_char,
                      214 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"int rt_start(struct rt *, int)\x00")).as_ptr());
    }
    (*rt).priority = priority;
    /* If there are any devices which returned file descriptors for
     * poll() then launch the realtime thread to handle them */
    if (*rt).npt > 0 as libc::c_int as libc::c_ulong {
        let mut r: libc::c_int = 0;
        fprintf(stderr,
                b"Launching realtime thread to handle devices...\n\x00" as
                    *const u8 as *const libc::c_char);
        if sem_init(&mut (*rt).sem, 0 as libc::c_int,
                    0 as libc::c_int as libc::c_uint) == -(1 as libc::c_int) {
            perror(b"sem_init\x00" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        r =
            pthread_create(&mut (*rt).ph, 0 as *const pthread_attr_t,
                           Some(launch as
                                    unsafe extern "C" fn(_: *mut libc::c_void)
                                        -> *mut libc::c_void),
                           rt as *mut libc::c_void);
        if r != 0 as libc::c_int {
            *__errno_location() = r;
            perror(b"pthread_create\x00" as *const u8 as *const libc::c_char);
            if sem_destroy(&mut (*rt).sem) == -(1 as libc::c_int) { abort(); }
            return -(1 as libc::c_int)
        }
        /* Wait for the realtime thread to declare it is initialised */
        if sem_wait(&mut (*rt).sem) == -(1 as libc::c_int) { abort(); }
        if sem_destroy(&mut (*rt).sem) == -(1 as libc::c_int) { abort(); }
        if (*rt).finished {
            if pthread_join((*rt).ph, 0 as *mut *mut libc::c_void) !=
                   0 as libc::c_int {
                abort();
            }
            return -(1 as libc::c_int)
        }
    }
    n = 0 as libc::c_int as size_t;
    while n < (*rt).ndv {
        device_start((*rt).dv[n as usize]);
        n = n.wrapping_add(1)
    }
    return 0 as libc::c_int;
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
 * State data for the realtime thread, maintained during rt_start and
 * rt_stop
 */
/*
 * Stop realtime handling, which was previously started by rt_start()
 */
#[no_mangle]
pub unsafe extern "C" fn rt_stop(mut rt: *mut rt) {
    let mut n: size_t = 0;
    (*rt).finished = 1 as libc::c_int != 0;
    /* Stop audio rolling on devices */
    n = 0 as libc::c_int as size_t;
    while n < (*rt).ndv {
        device_stop((*rt).dv[n as usize]);
        n = n.wrapping_add(1)
    }
    if (*rt).npt > 0 as libc::c_int as libc::c_ulong {
        if pthread_join((*rt).ph, 0 as *mut *mut libc::c_void) !=
               0 as libc::c_int {
            abort();
        }
    };
}
