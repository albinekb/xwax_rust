use ::libc;
extern "C" {
    pub type player;
    pub type timecoder;
    #[no_mangle]
    fn device_init(dv: *mut device, ops: *mut device_ops);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn sample_rate(mut d: *mut device) -> libc::c_uint {
    return 48000 as libc::c_int as libc::c_uint;
}
static mut dummy_ops: device_ops =
    unsafe {
        {
            let mut init =
                device_ops{pollfds: None,
                           handle: None,
                           sample_rate:
                               Some(sample_rate as
                                        unsafe extern "C" fn(_: *mut device)
                                            -> libc::c_uint),
                           start: None,
                           stop: None,
                           clear: None,};
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
#[no_mangle]
pub unsafe extern "C" fn dummy_init(mut d: *mut device) {
    device_init(d, &mut dummy_ops);
}
