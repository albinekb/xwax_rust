use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type SDL_BlitMap;
    pub type private_hwdata;
    /* * If you want to use this event, you should include SDL_syswm.h */
    pub type SDL_SysWMmsg;
    /* * Definition of the timer ID type */
    pub type _SDL_TimerID;
    pub type _TTF_Font;
    pub type controller;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    #[no_mangle]
    fn iconv(__cd: iconv_t, __inbuf: *mut *mut libc::c_char,
             __inbytesleft: *mut size_t, __outbuf: *mut *mut libc::c_char,
             __outbytesleft: *mut size_t) -> size_t;
    #[no_mangle]
    fn iconv_open(__tocode: *const libc::c_char,
                  __fromcode: *const libc::c_char) -> iconv_t;
    #[no_mangle]
    fn atanf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn log2(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
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
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    /* *< Don't catch fatal signals */
    /* *< Not supported on all OS's */
    /*@}*/
    /* * This function loads the SDL dynamically linked library and initializes 
 *  the subsystems specified by 'flags' (and those satisfying dependencies)
 *  Unless the SDL_INIT_NOPARACHUTE flag is set, it will install cleanup
 *  signal handlers for some commonly ignored fatal signals (like SIGSEGV)
 */
    #[no_mangle]
    fn SDL_Init(flags: Uint32) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetError() -> *mut libc::c_char;
    /* *
 * Enable/Disable keyboard repeat.  Keyboard repeat defaults to off.
 *
 *  @param[in] delay
 *  'delay' is the initial delay in ms between the time when a key is
 *  pressed, and keyboard repeat begins.
 *
 *  @param[in] interval
 *  'interval' is the time in ms between keyboard repeat events.
 *
 *  If 'delay' is set to 0, keyboard repeat is disabled.
 */
    #[no_mangle]
    fn SDL_EnableKeyRepeat(delay: libc::c_int, interval: libc::c_int)
     -> libc::c_int;
    /* *
 * Set up a video mode with the specified width, height and bits-per-pixel.
 *
 * If 'bpp' is 0, it is treated as the current display bits per pixel.
 *
 * If SDL_ANYFORMAT is set in 'flags', the SDL library will try to set the
 * requested bits-per-pixel, but will return whatever video pixel format is
 * available.  The default is to emulate the requested pixel format if it
 * is not natively available.
 *
 * If SDL_HWSURFACE is set in 'flags', the video surface will be placed in
 * video memory, if possible, and you may have to call SDL_LockSurface()
 * in order to access the raw framebuffer.  Otherwise, the video surface
 * will be created in system memory.
 *
 * If SDL_ASYNCBLIT is set in 'flags', SDL will try to perform rectangle
 * updates asynchronously, but you must always lock before accessing pixels.
 * SDL will wait for updates to complete before returning from the lock.
 *
 * If SDL_HWPALETTE is set in 'flags', the SDL library will guarantee
 * that the colors set by SDL_SetColors() will be the colors you get.
 * Otherwise, in 8-bit mode, SDL_SetColors() may not be able to set all
 * of the colors exactly the way they are requested, and you should look
 * at the video surface structure to determine the actual palette.
 * If SDL cannot guarantee that the colors you request can be set, 
 * i.e. if the colormap is shared, then the video surface may be created
 * under emulation in system memory, overriding the SDL_HWSURFACE flag.
 *
 * If SDL_FULLSCREEN is set in 'flags', the SDL library will try to set
 * a fullscreen video mode.  The default is to create a windowed mode
 * if the current graphics system has a window manager.
 * If the SDL library is able to set a fullscreen video mode, this flag 
 * will be set in the surface that is returned.
 *
 * If SDL_DOUBLEBUF is set in 'flags', the SDL library will try to set up
 * two surfaces in video memory and swap between them when you call 
 * SDL_Flip().  This is usually slower than the normal single-buffering
 * scheme, but prevents "tearing" artifacts caused by modifying video 
 * memory while the monitor is refreshing.  It should only be used by 
 * applications that redraw the entire screen on every update.
 *
 * If SDL_RESIZABLE is set in 'flags', the SDL library will allow the
 * window manager, if any, to resize the window at runtime.  When this
 * occurs, SDL will send a SDL_VIDEORESIZE event to you application,
 * and you must respond to the event by re-calling SDL_SetVideoMode()
 * with the requested size (or another size that suits the application).
 *
 * If SDL_NOFRAME is set in 'flags', the SDL library will create a window
 * without any title bar or frame decoration.  Fullscreen video modes have
 * this flag set automatically.
 *
 * This function returns the video framebuffer surface, or NULL if it fails.
 *
 * If you rely on functionality provided by certain video flags, check the
 * flags of the returned surface to make sure that functionality is available.
 * SDL will fall back to reduced functionality if the exact flags you wanted
 * are not available.
 */
    #[no_mangle]
    fn SDL_SetVideoMode(width_0: libc::c_int, height_0: libc::c_int,
                        bpp: libc::c_int, flags: Uint32) -> *mut SDL_Surface;
    /* *
 * If 'x', 'y', 'w' and 'h' are all 0, SDL_UpdateRect will update the entire
 * screen.
 */
    #[no_mangle]
    fn SDL_UpdateRect(screen: *mut SDL_Surface, x: Sint32, y: Sint32,
                      w: Uint32, h: Uint32);
    /* *
 * Maps an RGB triple to an opaque pixel value for a given pixel format
 */
    #[no_mangle]
    fn SDL_MapRGB(format: *const SDL_PixelFormat, r: Uint8, g: Uint8,
                  b: Uint8) -> Uint32;
    #[no_mangle]
    fn SDL_FreeSurface(surface: *mut SDL_Surface);
    /* *
 * SDL_LockSurface() sets up a surface for directly accessing the pixels.
 * Between calls to SDL_LockSurface()/SDL_UnlockSurface(), you can write
 * to and read from 'surface->pixels', using the pixel format stored in 
 * 'surface->format'.  Once you are done accessing the surface, you should 
 * use SDL_UnlockSurface() to release it.
 *
 * Not all surfaces require locking.  If SDL_MUSTLOCK(surface) evaluates
 * to 0, then you can read and write to the surface at any time, and the
 * pixel format of the surface will not change.  In particular, if the
 * SDL_HWSURFACE flag is not given when calling SDL_SetVideoMode(), you
 * will not need to lock the display surface before accessing it.
 * 
 * No operating system or library calls should be made between lock/unlock
 * pairs, as critical system locks may be held during this time.
 *
 * SDL_LockSurface() returns 0, or -1 if the surface couldn't be locked.
 */
    #[no_mangle]
    fn SDL_LockSurface(surface: *mut SDL_Surface) -> libc::c_int;
    #[no_mangle]
    fn SDL_UnlockSurface(surface: *mut SDL_Surface);
    /* *
 * This performs a fast blit from the source surface to the destination
 * surface.  It assumes that the source and destination rectangles are
 * the same size.  If either 'srcrect' or 'dstrect' are NULL, the entire
 * surface (src or dst) is copied.  The final blit rectangles are saved
 * in 'srcrect' and 'dstrect' after all clipping is performed.
 * If the blit is successful, it returns 0, otherwise it returns -1.
 *
 * The blit function should not be called on a locked surface.
 *
 * The blit semantics for surfaces with and without alpha and colorkey
 * are defined as follows:
 *
 * RGBA->RGB:
 *     SDL_SRCALPHA set:
 * 	alpha-blend (using alpha-channel).
 * 	SDL_SRCCOLORKEY ignored.
 *     SDL_SRCALPHA not set:
 * 	copy RGB.
 * 	if SDL_SRCCOLORKEY set, only copy the pixels matching the
 * 	RGB values of the source colour key, ignoring alpha in the
 * 	comparison.
 * 
 * RGB->RGBA:
 *     SDL_SRCALPHA set:
 * 	alpha-blend (using the source per-surface alpha value);
 * 	set destination alpha to opaque.
 *     SDL_SRCALPHA not set:
 * 	copy RGB, set destination alpha to source per-surface alpha value.
 *     both:
 * 	if SDL_SRCCOLORKEY set, only copy the pixels matching the
 * 	source colour key.
 * 
 * RGBA->RGBA:
 *     SDL_SRCALPHA set:
 * 	alpha-blend (using the source alpha channel) the RGB values;
 * 	leave destination alpha untouched. [Note: is this correct?]
 * 	SDL_SRCCOLORKEY ignored.
 *     SDL_SRCALPHA not set:
 * 	copy all of RGBA to the destination.
 * 	if SDL_SRCCOLORKEY set, only copy the pixels matching the
 * 	RGB values of the source colour key, ignoring alpha in the
 * 	comparison.
 * 
 * RGB->RGB: 
 *     SDL_SRCALPHA set:
 * 	alpha-blend (using the source per-surface alpha value).
 *     SDL_SRCALPHA not set:
 * 	copy RGB.
 *     both:
 * 	if SDL_SRCCOLORKEY set, only copy the pixels matching the
 * 	source colour key.
 *
 * If either of the surfaces were in video memory, and the blit returns -2,
 * the video memory was lost, so it should be reloaded with artwork and 
 * re-blitted:
 * @code
 *	while ( SDL_BlitSurface(image, imgrect, screen, dstrect) == -2 ) {
 *		while ( SDL_LockSurface(image) < 0 )
 *			Sleep(10);
 *		-- Write image pixels to image->pixels --
 *		SDL_UnlockSurface(image);
 *	}
 * @endcode
 *
 * This happens under DirectX 5.0 when the system switches away from your
 * fullscreen application.  The lock will also fail until you have access
 * to the video memory again.
 *
 * You should call SDL_BlitSurface() unless you know exactly how SDL
 * blitting works internally and how to use the other blit functions.
 */
    /* * This is the public blit function, SDL_BlitSurface(), and it performs
 *  rectangle validation and clipping before passing it to SDL_LowerBlit()
 */
    #[no_mangle]
    fn SDL_UpperBlit(src: *mut SDL_Surface, srcrect: *mut SDL_Rect,
                     dst: *mut SDL_Surface, dstrect: *mut SDL_Rect)
     -> libc::c_int;
    /* *
 * This function performs a fast fill of the given rectangle with 'color'
 * The given rectangle is clipped to the destination surface clip area
 * and the final fill rectangle is saved in the passed in pointer.
 * If 'dstrect' is NULL, the whole surface will be filled with 'color'
 * The color should be a pixel of the format used by the surface, and 
 * can be generated by the SDL_MapRGB() function.
 * This function returns 0 on success, or -1 on error.
 */
    #[no_mangle]
    fn SDL_FillRect(dst: *mut SDL_Surface, dstrect: *mut SDL_Rect,
                    color: Uint32) -> libc::c_int;
    /*@}*/
    /*@}*/
    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
/* * @name Window Manager Functions                                           */
/* * These functions allow interaction with the window manager, if any.       */
    /*@{*/
    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    /* *
 * Sets the title and icon text of the display window (UTF-8 encoded)
 */
    #[no_mangle]
    fn SDL_WM_SetCaption(title: *const libc::c_char,
                         icon: *const libc::c_char);
    /* *
 *  Checks the event queue for messages and optionally returns them.
 *
 *  If 'action' is SDL_ADDEVENT, up to 'numevents' events will be added to
 *  the back of the event queue.
 *  If 'action' is SDL_PEEKEVENT, up to 'numevents' events at the front
 *  of the event queue, matching 'mask', will be returned and will not
 *  be removed from the queue.
 *  If 'action' is SDL_GETEVENT, up to 'numevents' events at the front 
 *  of the event queue, matching 'mask', will be returned and will be
 *  removed from the queue.
 *
 *  @return
 *  This function returns the number of events actually stored, or -1
 *  if there was an error.
 *
 *  This function is thread-safe.
 */
    #[no_mangle]
    fn SDL_PeepEvents(events: *mut SDL_Event, numevents: libc::c_int,
                      action: SDL_eventaction, mask: Uint32) -> libc::c_int;
    /* * Waits indefinitely for the next available event, returning 1, or 0 if there
 *  was an error while waiting for events.  If 'event' is not NULL, the next
 *  event is removed from the queue and stored in that area.
 */
    #[no_mangle]
    fn SDL_WaitEvent(event: *mut SDL_Event) -> libc::c_int;
    /* * Add an event to the event queue.
 *  This function returns 0 on success, or -1 if the event queue was full
 *  or there was some other error.
 */
    #[no_mangle]
    fn SDL_PushEvent(event: *mut SDL_Event) -> libc::c_int;
    /* * Add a new timer to the pool of timers already running.
 *  Returns a timer ID, or NULL when an error occurs.
 */
    #[no_mangle]
    fn SDL_AddTimer(interval: Uint32, callback: SDL_NewTimerCallback,
                    param: *mut libc::c_void) -> SDL_TimerID;
    /* *
 * Remove one of the multiple timers knowing its ID.
 * Returns a boolean value indicating success.
 */
    #[no_mangle]
    fn SDL_RemoveTimer(t: SDL_TimerID) -> SDL_bool;
    /* * This function cleans up all initialized subsystems and unloads the
 *  dynamically linked library.  You should call it upon all exit conditions.
 */
    #[no_mangle]
    fn SDL_Quit();
    /* Initialize the TTF engine - returns 0 if successful, -1 on error */
    #[no_mangle]
    fn TTF_Init() -> libc::c_int;
    /* Open a font file and create a font of the specified point size.
 * Some .fon fonts will have several sizes embedded in the file, so the
 * point size becomes the index of choosing which size.  If the value
 * is too high, the last indexed size will be the default. */
    #[no_mangle]
    fn TTF_OpenFont(file: *const libc::c_char, ptsize: libc::c_int)
     -> *mut TTF_Font;
    /* Get the offset from the baseline to the top of the font
   This is a positive value, relative to the baseline.
 */
    #[no_mangle]
    fn TTF_FontAscent(font_0: *const TTF_Font) -> libc::c_int;
    /* Create an 8-bit palettized surface and render the given text at
   high quality with the given font and colors.  The 0 pixel is background,
   while other pixels have varying degrees of the foreground color.
   This function returns the new surface, or NULL if there was an error.
*/
    #[no_mangle]
    fn TTF_RenderText_Shaded(font_0: *mut TTF_Font, text: *const libc::c_char,
                             fg: SDL_Color, bg: SDL_Color)
     -> *mut SDL_Surface;
    #[no_mangle]
    fn TTF_RenderUTF8_Shaded(font_0: *mut TTF_Font, text: *const libc::c_char,
                             fg: SDL_Color, bg: SDL_Color)
     -> *mut SDL_Surface;
    /* For compatibility with previous versions, here are the old functions */
    /* Close an opened font file */
    #[no_mangle]
    fn TTF_CloseFont(font_0: *mut TTF_Font);
    /* De-initialize the TTF engine */
    #[no_mangle]
    fn TTF_Quit();
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
    /* Punch */
    /* A controller adds itself here */
    #[no_mangle]
    fn deck_recue(deck_0: *mut deck);
    #[no_mangle]
    fn player_toggle_timecode_control(pl: *mut player) -> bool;
    #[no_mangle]
    fn player_set_internal_playback(pl: *mut player);
    #[no_mangle]
    fn player_get_position(pl: *mut player) -> libc::c_double;
    #[no_mangle]
    fn player_get_elapsed(pl: *mut player) -> libc::c_double;
    #[no_mangle]
    fn player_get_remain(pl: *mut player) -> libc::c_double;
    #[no_mangle]
    fn timecoder_monitor_init(tc: *mut timecoder, size: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn timecoder_monitor_clear(tc: *mut timecoder);
    #[no_mangle]
    fn timecoder_cycle_definition(tc: *mut timecoder);
    #[no_mangle]
    fn timecoder_get_position(tc: *mut timecoder, when: *mut libc::c_double)
     -> libc::c_int;
    #[no_mangle]
    fn deck_is_locked(deck_0: *const deck) -> bool;
    #[no_mangle]
    fn deck_load(deck_0: *mut deck, record: *mut record);
    #[no_mangle]
    fn deck_clone(deck_0: *mut deck, from: *const deck);
    #[no_mangle]
    fn rig_unlock();
    #[no_mangle]
    fn rig_quit() -> libc::c_int;
    #[no_mangle]
    fn rig_lock();
    /*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>,
 *                    Yves Adler <yves.adler@googlemail.com>
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
    /* base_index + search filter applied */
    /* used to swap between a and b indexes */
    /* the compiled search, kept in-sync */
    #[no_mangle]
    fn selector_clear(sel: *mut selector);
    #[no_mangle]
    fn listbox_current(s: *const listbox) -> libc::c_int;
    #[no_mangle]
    fn selector_init(sel: *mut selector, lib: *mut library);
    #[no_mangle]
    fn listbox_map(s: *const listbox, row: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn selector_set_lines(sel: *mut selector, lines: libc::c_uint);
    #[no_mangle]
    fn selector_up(sel: *mut selector);
    #[no_mangle]
    fn selector_down(sel: *mut selector);
    #[no_mangle]
    fn selector_page_up(sel: *mut selector);
    #[no_mangle]
    fn selector_page_down(sel: *mut selector);
    #[no_mangle]
    fn selector_top(sel: *mut selector);
    #[no_mangle]
    fn selector_bottom(sel: *mut selector);
    #[no_mangle]
    fn selector_current(sel: *mut selector) -> *mut record;
    #[no_mangle]
    fn selector_prev(sel: *mut selector);
    #[no_mangle]
    fn selector_next(sel: *mut selector);
    #[no_mangle]
    fn selector_toggle(sel: *mut selector);
    #[no_mangle]
    fn selector_toggle_order(sel: *mut selector);
    #[no_mangle]
    fn selector_rescan(sel: *mut selector);
    #[no_mangle]
    fn selector_search_expand(sel: *mut selector);
    #[no_mangle]
    fn selector_search_refine(sel: *mut selector, key: libc::c_char);
    #[no_mangle]
    static mut status_changed: event;
    #[no_mangle]
    fn status() -> *const libc::c_char;
    #[no_mangle]
    fn status_level() -> libc::c_int;
    #[no_mangle]
    fn status_set(level: libc::c_int, s: *const libc::c_char);
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
    static mut banner: *mut libc::c_char;
    #[no_mangle]
    static mut ndeck: size_t;
    #[no_mangle]
    static mut deck: [deck; 0];
}
pub type size_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pid_t = __pid_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 64],
    pub __align: libc::c_long,
}
pub type pthread_spinlock_t = libc::c_int;
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
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
/*
    SDL - Simple DirectMedia Layer
    Copyright (C) 1997-2012 Sam Lantinga

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

    Sam Lantinga
    slouken@libsdl.org
*/
/* * @file SDL_stdinc.h
 *  This is a general header that includes C language support
 */
/* * The number of elements in an array */
/* Use proper C++ casts when compiled as C++ to be compatible with the option
 -Wold-style-cast of GCC (and -Werror=old-style-cast in GCC 4.2 and above. */
/* * @name Basic data types */
/*@{*/
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Sint32 = int32_t;
pub type Uint32 = uint32_t;
/*
    SDL - Simple DirectMedia Layer
    Copyright (C) 1997-2012 Sam Lantinga

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

    Sam Lantinga
    slouken@libsdl.org
*/
/* * What we really want is a mapping of every raw key on the keyboard.
 *  To support international keyboards, we use the range 0xA1 - 0xFF
 *  as international virtual keycodes.  We'll follow in the footsteps of X11...
 *  @brief The names of the keys
 */
pub type SDLKey = libc::c_uint;
/* *< Atari keyboard has Undo */
/*@}*/
/* Add any other keys here */
pub const SDLK_LAST: SDLKey = 323;
/* *< Some european keyboards */
pub const SDLK_UNDO: SDLKey = 322;
/* *< Power Macintosh power key */
pub const SDLK_EURO: SDLKey = 321;
pub const SDLK_POWER: SDLKey = 320;
pub const SDLK_MENU: SDLKey = 319;
pub const SDLK_BREAK: SDLKey = 318;
pub const SDLK_SYSREQ: SDLKey = 317;
pub const SDLK_PRINT: SDLKey = 316;
/* *< Multi-key compose key */
/*@}*/
/* * @name Miscellaneous function keys */
        /*@{*/
pub const SDLK_HELP: SDLKey = 315;
/* *< "Alt Gr" key */
pub const SDLK_COMPOSE: SDLKey = 314;
/* *< Right "Windows" key */
pub const SDLK_MODE: SDLKey = 313;
/* *< Left "Windows" key */
pub const SDLK_RSUPER: SDLKey = 312;
pub const SDLK_LSUPER: SDLKey = 311;
pub const SDLK_LMETA: SDLKey = 310;
pub const SDLK_RMETA: SDLKey = 309;
pub const SDLK_LALT: SDLKey = 308;
pub const SDLK_RALT: SDLKey = 307;
pub const SDLK_LCTRL: SDLKey = 306;
pub const SDLK_RCTRL: SDLKey = 305;
pub const SDLK_LSHIFT: SDLKey = 304;
pub const SDLK_RSHIFT: SDLKey = 303;
pub const SDLK_SCROLLOCK: SDLKey = 302;
pub const SDLK_CAPSLOCK: SDLKey = 301;
/*@}*/
/* * @name Key state modifier keys */
        /*@{*/
pub const SDLK_NUMLOCK: SDLKey = 300;
pub const SDLK_F15: SDLKey = 296;
pub const SDLK_F14: SDLKey = 295;
pub const SDLK_F13: SDLKey = 294;
pub const SDLK_F12: SDLKey = 293;
pub const SDLK_F11: SDLKey = 292;
pub const SDLK_F10: SDLKey = 291;
pub const SDLK_F9: SDLKey = 290;
pub const SDLK_F8: SDLKey = 289;
pub const SDLK_F7: SDLKey = 288;
pub const SDLK_F6: SDLKey = 287;
pub const SDLK_F5: SDLKey = 286;
pub const SDLK_F4: SDLKey = 285;
pub const SDLK_F3: SDLKey = 284;
pub const SDLK_F2: SDLKey = 283;
/*@}*/
/* * @name Function keys */
        /*@{*/
pub const SDLK_F1: SDLKey = 282;
pub const SDLK_PAGEDOWN: SDLKey = 281;
pub const SDLK_PAGEUP: SDLKey = 280;
pub const SDLK_END: SDLKey = 279;
pub const SDLK_HOME: SDLKey = 278;
pub const SDLK_INSERT: SDLKey = 277;
pub const SDLK_LEFT: SDLKey = 276;
pub const SDLK_RIGHT: SDLKey = 275;
pub const SDLK_DOWN: SDLKey = 274;
/*@}*/
/* * @name Arrows + Home/End pad */
        /*@{*/
pub const SDLK_UP: SDLKey = 273;
pub const SDLK_KP_EQUALS: SDLKey = 272;
pub const SDLK_KP_ENTER: SDLKey = 271;
pub const SDLK_KP_PLUS: SDLKey = 270;
pub const SDLK_KP_MINUS: SDLKey = 269;
pub const SDLK_KP_MULTIPLY: SDLKey = 268;
pub const SDLK_KP_DIVIDE: SDLKey = 267;
pub const SDLK_KP_PERIOD: SDLKey = 266;
pub const SDLK_KP9: SDLKey = 265;
pub const SDLK_KP8: SDLKey = 264;
pub const SDLK_KP7: SDLKey = 263;
pub const SDLK_KP6: SDLKey = 262;
pub const SDLK_KP5: SDLKey = 261;
pub const SDLK_KP4: SDLKey = 260;
pub const SDLK_KP3: SDLKey = 259;
pub const SDLK_KP2: SDLKey = 258;
pub const SDLK_KP1: SDLKey = 257;
/* 0xFF */
/*@}*/
/* * @name Numeric keypad */
        /*@{*/
pub const SDLK_KP0: SDLKey = 256;
pub const SDLK_WORLD_95: SDLKey = 255;
pub const SDLK_WORLD_94: SDLKey = 254;
pub const SDLK_WORLD_93: SDLKey = 253;
pub const SDLK_WORLD_92: SDLKey = 252;
pub const SDLK_WORLD_91: SDLKey = 251;
pub const SDLK_WORLD_90: SDLKey = 250;
pub const SDLK_WORLD_89: SDLKey = 249;
pub const SDLK_WORLD_88: SDLKey = 248;
pub const SDLK_WORLD_87: SDLKey = 247;
pub const SDLK_WORLD_86: SDLKey = 246;
pub const SDLK_WORLD_85: SDLKey = 245;
pub const SDLK_WORLD_84: SDLKey = 244;
pub const SDLK_WORLD_83: SDLKey = 243;
pub const SDLK_WORLD_82: SDLKey = 242;
pub const SDLK_WORLD_81: SDLKey = 241;
pub const SDLK_WORLD_80: SDLKey = 240;
pub const SDLK_WORLD_79: SDLKey = 239;
pub const SDLK_WORLD_78: SDLKey = 238;
pub const SDLK_WORLD_77: SDLKey = 237;
pub const SDLK_WORLD_76: SDLKey = 236;
pub const SDLK_WORLD_75: SDLKey = 235;
pub const SDLK_WORLD_74: SDLKey = 234;
pub const SDLK_WORLD_73: SDLKey = 233;
pub const SDLK_WORLD_72: SDLKey = 232;
pub const SDLK_WORLD_71: SDLKey = 231;
pub const SDLK_WORLD_70: SDLKey = 230;
pub const SDLK_WORLD_69: SDLKey = 229;
pub const SDLK_WORLD_68: SDLKey = 228;
pub const SDLK_WORLD_67: SDLKey = 227;
pub const SDLK_WORLD_66: SDLKey = 226;
pub const SDLK_WORLD_65: SDLKey = 225;
pub const SDLK_WORLD_64: SDLKey = 224;
pub const SDLK_WORLD_63: SDLKey = 223;
pub const SDLK_WORLD_62: SDLKey = 222;
pub const SDLK_WORLD_61: SDLKey = 221;
pub const SDLK_WORLD_60: SDLKey = 220;
pub const SDLK_WORLD_59: SDLKey = 219;
pub const SDLK_WORLD_58: SDLKey = 218;
pub const SDLK_WORLD_57: SDLKey = 217;
pub const SDLK_WORLD_56: SDLKey = 216;
pub const SDLK_WORLD_55: SDLKey = 215;
pub const SDLK_WORLD_54: SDLKey = 214;
pub const SDLK_WORLD_53: SDLKey = 213;
pub const SDLK_WORLD_52: SDLKey = 212;
pub const SDLK_WORLD_51: SDLKey = 211;
pub const SDLK_WORLD_50: SDLKey = 210;
pub const SDLK_WORLD_49: SDLKey = 209;
pub const SDLK_WORLD_48: SDLKey = 208;
pub const SDLK_WORLD_47: SDLKey = 207;
pub const SDLK_WORLD_46: SDLKey = 206;
pub const SDLK_WORLD_45: SDLKey = 205;
pub const SDLK_WORLD_44: SDLKey = 204;
pub const SDLK_WORLD_43: SDLKey = 203;
pub const SDLK_WORLD_42: SDLKey = 202;
pub const SDLK_WORLD_41: SDLKey = 201;
pub const SDLK_WORLD_40: SDLKey = 200;
pub const SDLK_WORLD_39: SDLKey = 199;
pub const SDLK_WORLD_38: SDLKey = 198;
pub const SDLK_WORLD_37: SDLKey = 197;
pub const SDLK_WORLD_36: SDLKey = 196;
pub const SDLK_WORLD_35: SDLKey = 195;
pub const SDLK_WORLD_34: SDLKey = 194;
pub const SDLK_WORLD_33: SDLKey = 193;
pub const SDLK_WORLD_32: SDLKey = 192;
pub const SDLK_WORLD_31: SDLKey = 191;
pub const SDLK_WORLD_30: SDLKey = 190;
pub const SDLK_WORLD_29: SDLKey = 189;
pub const SDLK_WORLD_28: SDLKey = 188;
pub const SDLK_WORLD_27: SDLKey = 187;
pub const SDLK_WORLD_26: SDLKey = 186;
pub const SDLK_WORLD_25: SDLKey = 185;
pub const SDLK_WORLD_24: SDLKey = 184;
pub const SDLK_WORLD_23: SDLKey = 183;
pub const SDLK_WORLD_22: SDLKey = 182;
pub const SDLK_WORLD_21: SDLKey = 181;
pub const SDLK_WORLD_20: SDLKey = 180;
pub const SDLK_WORLD_19: SDLKey = 179;
pub const SDLK_WORLD_18: SDLKey = 178;
pub const SDLK_WORLD_17: SDLKey = 177;
pub const SDLK_WORLD_16: SDLKey = 176;
pub const SDLK_WORLD_15: SDLKey = 175;
pub const SDLK_WORLD_14: SDLKey = 174;
pub const SDLK_WORLD_13: SDLKey = 173;
pub const SDLK_WORLD_12: SDLKey = 172;
pub const SDLK_WORLD_11: SDLKey = 171;
pub const SDLK_WORLD_10: SDLKey = 170;
pub const SDLK_WORLD_9: SDLKey = 169;
pub const SDLK_WORLD_8: SDLKey = 168;
pub const SDLK_WORLD_7: SDLKey = 167;
pub const SDLK_WORLD_6: SDLKey = 166;
pub const SDLK_WORLD_5: SDLKey = 165;
pub const SDLK_WORLD_4: SDLKey = 164;
pub const SDLK_WORLD_3: SDLKey = 163;
pub const SDLK_WORLD_2: SDLKey = 162;
/* 0xA0 */
pub const SDLK_WORLD_1: SDLKey = 161;
/* End of ASCII mapped keysyms */
        /*@}*/
/* * @name International keyboard syms */
        /*@{*/
pub const SDLK_WORLD_0: SDLKey = 160;
pub const SDLK_DELETE: SDLKey = 127;
pub const SDLK_z: SDLKey = 122;
pub const SDLK_y: SDLKey = 121;
pub const SDLK_x: SDLKey = 120;
pub const SDLK_w: SDLKey = 119;
pub const SDLK_v: SDLKey = 118;
pub const SDLK_u: SDLKey = 117;
pub const SDLK_t: SDLKey = 116;
pub const SDLK_s: SDLKey = 115;
pub const SDLK_r: SDLKey = 114;
pub const SDLK_q: SDLKey = 113;
pub const SDLK_p: SDLKey = 112;
pub const SDLK_o: SDLKey = 111;
pub const SDLK_n: SDLKey = 110;
pub const SDLK_m: SDLKey = 109;
pub const SDLK_l: SDLKey = 108;
pub const SDLK_k: SDLKey = 107;
pub const SDLK_j: SDLKey = 106;
pub const SDLK_i: SDLKey = 105;
pub const SDLK_h: SDLKey = 104;
pub const SDLK_g: SDLKey = 103;
pub const SDLK_f: SDLKey = 102;
pub const SDLK_e: SDLKey = 101;
pub const SDLK_d: SDLKey = 100;
pub const SDLK_c: SDLKey = 99;
pub const SDLK_b: SDLKey = 98;
pub const SDLK_a: SDLKey = 97;
pub const SDLK_BACKQUOTE: SDLKey = 96;
pub const SDLK_UNDERSCORE: SDLKey = 95;
pub const SDLK_CARET: SDLKey = 94;
pub const SDLK_RIGHTBRACKET: SDLKey = 93;
pub const SDLK_BACKSLASH: SDLKey = 92;
/* 
	   Skip uppercase letters
	 */
pub const SDLK_LEFTBRACKET: SDLKey = 91;
pub const SDLK_AT: SDLKey = 64;
pub const SDLK_QUESTION: SDLKey = 63;
pub const SDLK_GREATER: SDLKey = 62;
pub const SDLK_EQUALS: SDLKey = 61;
pub const SDLK_LESS: SDLKey = 60;
pub const SDLK_SEMICOLON: SDLKey = 59;
pub const SDLK_COLON: SDLKey = 58;
pub const SDLK_9: SDLKey = 57;
pub const SDLK_8: SDLKey = 56;
pub const SDLK_7: SDLKey = 55;
pub const SDLK_6: SDLKey = 54;
pub const SDLK_5: SDLKey = 53;
pub const SDLK_4: SDLKey = 52;
pub const SDLK_3: SDLKey = 51;
pub const SDLK_2: SDLKey = 50;
pub const SDLK_1: SDLKey = 49;
pub const SDLK_0: SDLKey = 48;
pub const SDLK_SLASH: SDLKey = 47;
pub const SDLK_PERIOD: SDLKey = 46;
pub const SDLK_MINUS: SDLKey = 45;
pub const SDLK_COMMA: SDLKey = 44;
pub const SDLK_PLUS: SDLKey = 43;
pub const SDLK_ASTERISK: SDLKey = 42;
pub const SDLK_RIGHTPAREN: SDLKey = 41;
pub const SDLK_LEFTPAREN: SDLKey = 40;
pub const SDLK_QUOTE: SDLKey = 39;
pub const SDLK_AMPERSAND: SDLKey = 38;
pub const SDLK_DOLLAR: SDLKey = 36;
pub const SDLK_HASH: SDLKey = 35;
pub const SDLK_QUOTEDBL: SDLKey = 34;
pub const SDLK_EXCLAIM: SDLKey = 33;
pub const SDLK_SPACE: SDLKey = 32;
pub const SDLK_ESCAPE: SDLKey = 27;
pub const SDLK_PAUSE: SDLKey = 19;
pub const SDLK_RETURN: SDLKey = 13;
pub const SDLK_CLEAR: SDLKey = 12;
pub const SDLK_TAB: SDLKey = 9;
pub const SDLK_BACKSPACE: SDLKey = 8;
pub const SDLK_FIRST: SDLKey = 0;
/* * @name ASCII mapped keysyms
         *  The keyboard syms have been cleverly chosen to map to ASCII
         */
        /*@{*/
pub const SDLK_UNKNOWN: SDLKey = 0;
/* * Enumeration of valid key mods (possibly OR'd together) */
pub type SDLMod = libc::c_uint;
pub const KMOD_RESERVED: SDLMod = 32768;
pub const KMOD_MODE: SDLMod = 16384;
pub const KMOD_CAPS: SDLMod = 8192;
pub const KMOD_NUM: SDLMod = 4096;
pub const KMOD_RMETA: SDLMod = 2048;
pub const KMOD_LMETA: SDLMod = 1024;
pub const KMOD_RALT: SDLMod = 512;
pub const KMOD_LALT: SDLMod = 256;
pub const KMOD_RCTRL: SDLMod = 128;
pub const KMOD_LCTRL: SDLMod = 64;
pub const KMOD_RSHIFT: SDLMod = 2;
pub const KMOD_LSHIFT: SDLMod = 1;
pub const KMOD_NONE: SDLMod = 0;
/*
    SDL - Simple DirectMedia Layer
    Copyright (C) 1997-2012 Sam Lantinga

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

    Sam Lantinga
    slouken@libsdl.org
*/
/* * @file SDL_keyboard.h
 *  Include file for SDL keyboard event handling
 */
/* Set up for C function definitions, even when using C++ */
/* * Keysym structure
 *
 *  - The scancode is hardware dependent, and should not be used by general
 *    applications.  If no hardware scancode is available, it will be 0.
 *
 *  - The 'unicode' translated character is only available when character
 *    translation is enabled by the SDL_EnableUNICODE() API.  If non-zero,
 *    this is a UNICODE character corresponding to the keypress.  If the
 *    high 9 bits of the character are 0, then this maps to the equivalent
 *    ASCII character:
 *      @code
 *	char ch;
 *	if ( (keysym.unicode & 0xFF80) == 0 ) {
 *		ch = keysym.unicode & 0x7F;
 *	} else {
 *		An international character..
 *	}
 *      @endcode
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_keysym {
    pub scancode: Uint8,
    pub sym: SDLKey,
    pub mod_0: SDLMod,
    pub unicode: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: Sint16,
    pub y: Sint16,
    pub w: Uint16,
    pub h: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub unused: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: libc::c_int,
    pub colors: *mut SDL_Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_PixelFormat {
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub colorkey: Uint32,
    pub alpha: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: Uint32,
    pub format: *mut SDL_PixelFormat,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub pitch: Uint16,
    pub pixels: *mut libc::c_void,
    pub offset: libc::c_int,
    pub hwdata: *mut private_hwdata,
    pub clip_rect: SDL_Rect,
    pub unused1: Uint32,
    pub locked: Uint32,
    pub map: *mut SDL_BlitMap,
    pub format_version: libc::c_uint,
    pub refcount: libc::c_int,
}
/*
    SDL - Simple DirectMedia Layer
    Copyright (C) 1997-2012 Sam Lantinga

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

    Sam Lantinga
    slouken@libsdl.org
*/
/* *
 *  @file SDL_events.h
 *  Include file for SDL event handling
 */
/* Set up for C function definitions, even when using C++ */
/* * @name General keyboard/mouse state definitions */
/*@{*/
/*@}*/
/* * Event enumerations */
pub type C2RustUnnamed = libc::c_uint;
/* * This last event is only for bounding internal arrays
	*  It is the number of bits in the event mask datatype -- Uint32
        */
pub const SDL_NUMEVENTS: C2RustUnnamed = 32;
/* *< Reserved for future use.. */
/* * Events SDL_USEREVENT through SDL_MAXEVENTS-1 are for your use */
pub const SDL_USEREVENT: C2RustUnnamed = 24;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED7: C2RustUnnamed = 23;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED6: C2RustUnnamed = 22;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED5: C2RustUnnamed = 21;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED4: C2RustUnnamed = 20;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVED3: C2RustUnnamed = 19;
/* *< Screen needs to be redrawn */
pub const SDL_EVENT_RESERVED2: C2RustUnnamed = 18;
/* *< User resized video mode */
pub const SDL_VIDEOEXPOSE: C2RustUnnamed = 17;
/* *< Reserved for future use.. */
pub const SDL_VIDEORESIZE: C2RustUnnamed = 16;
/* *< Reserved for future use.. */
pub const SDL_EVENT_RESERVEDB: C2RustUnnamed = 15;
/* *< System specific event */
pub const SDL_EVENT_RESERVEDA: C2RustUnnamed = 14;
/* *< User-requested quit */
pub const SDL_SYSWMEVENT: C2RustUnnamed = 13;
/* *< Joystick button released */
pub const SDL_QUIT: C2RustUnnamed = 12;
/* *< Joystick button pressed */
pub const SDL_JOYBUTTONUP: C2RustUnnamed = 11;
/* *< Joystick hat position change */
pub const SDL_JOYBUTTONDOWN: C2RustUnnamed = 10;
/* *< Joystick trackball motion */
pub const SDL_JOYHATMOTION: C2RustUnnamed = 9;
/* *< Joystick axis motion */
pub const SDL_JOYBALLMOTION: C2RustUnnamed = 8;
/* *< Mouse button released */
pub const SDL_JOYAXISMOTION: C2RustUnnamed = 7;
/* *< Mouse button pressed */
pub const SDL_MOUSEBUTTONUP: C2RustUnnamed = 6;
/* *< Mouse moved */
pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed = 5;
/* *< Keys released */
pub const SDL_MOUSEMOTION: C2RustUnnamed = 4;
/* *< Keys pressed */
pub const SDL_KEYUP: C2RustUnnamed = 3;
/* *< Application loses/gains visibility */
pub const SDL_KEYDOWN: C2RustUnnamed = 2;
/* *< Unused (do not remove) */
pub const SDL_ACTIVEEVENT: C2RustUnnamed = 1;
pub const SDL_NOEVENT: C2RustUnnamed = 0;
/*@}*/
/* * Application visibility event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ActiveEvent {
    pub type_0: Uint8,
    pub gain: Uint8,
    pub state: Uint8,
}
/* * Keyboard event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub state: Uint8,
    pub keysym: SDL_keysym,
}
/* * Mouse motion event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub state: Uint8,
    pub x: Uint16,
    pub y: Uint16,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
/* * Mouse button event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub button: Uint8,
    pub state: Uint8,
    pub x: Uint16,
    pub y: Uint16,
}
/* * Joystick axis motion event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub axis: Uint8,
    pub value: Sint16,
}
/* * Joystick trackball motion event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub ball: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
/* * Joystick hat position change event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub hat: Uint8,
    pub value: Uint8,
}
/* * Joystick button event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub button: Uint8,
    pub state: Uint8,
}
/* * The "window resized" event
 *  When you get this event, you are responsible for setting a new video
 *  mode with the new width and height.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ResizeEvent {
    pub type_0: Uint8,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
/* * The "screen redraw" event */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ExposeEvent {
    pub type_0: Uint8,
}
/* * The "quit requested" event */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: Uint8,
}
/* * A user-defined event type */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub type_0: Uint8,
    pub code: libc::c_int,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub type_0: Uint8,
    pub msg: *mut SDL_SysWMmsg,
}
/* * General event structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub type_0: Uint8,
    pub active: SDL_ActiveEvent,
    pub key: SDL_KeyboardEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub resize: SDL_ResizeEvent,
    pub expose: SDL_ExposeEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
}
pub type SDL_eventaction = libc::c_uint;
pub const SDL_GETEVENT: SDL_eventaction = 2;
pub const SDL_PEEKEVENT: SDL_eventaction = 1;
pub const SDL_ADDEVENT: SDL_eventaction = 0;
/* * @name New timer API
 * New timer API, supports multiple timers
 * Written by Stephane Peter <megastep@lokigames.com>
 */
/*@{*/
/* *
 * Function prototype for the new timer callback function.
 * The callback function is passed the current timer interval and returns
 * the next timer interval.  If the returned value is the same as the one
 * passed in, the periodic alarm continues, otherwise a new alarm is
 * scheduled.  If the callback returns 0, the periodic alarm is cancelled.
 */
pub type SDL_NewTimerCallback
    =
    Option<unsafe extern "C" fn(_: Uint32, _: *mut libc::c_void) -> Uint32>;
pub type SDL_TimerID = *mut _SDL_TimerID;
/* The internal structure containing font information */
pub type TTF_Font = _TTF_Font;
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
 * A set of cue points
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cues {
    pub position: [libc::c_double; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub fault: bool,
    pub local: *mut libc::c_void,
    pub ops: *mut device_ops,
    pub timecoder: *mut timecoder,
    pub player: *mut player,
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
pub struct player {
    pub sample_dt: libc::c_double,
    pub lock: spin,
    pub track: *mut track,
    pub position: libc::c_double,
    pub target_position: libc::c_double,
    pub offset: libc::c_double,
    pub last_difference: libc::c_double,
    pub pitch: libc::c_double,
    pub sync_pitch: libc::c_double,
    pub volume: libc::c_double,
    pub timecoder: *mut timecoder,
    pub timecode_control: bool,
    pub recalibrate: bool,
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
/* number of bits in string */
/* wave cycles per second */
/* LFSR value at timecode zero */
/* central LFSR taps, excluding end taps */
/* in cycles */
/* last 'safe' timecode number (for auto disconnect) */
/* true if lut has been generated */
/* wave is in positive part of cycle */
/* wave recently swapped polarity */
/* samples since we last crossed zero */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timecoder {
    pub def: *mut timecode_def,
    pub speed: libc::c_double,
    pub dt: libc::c_double,
    pub zero_alpha: libc::c_double,
    pub threshold: libc::c_int,
    pub forwards: bool,
    pub primary: timecoder_channel,
    pub secondary: timecoder_channel,
    pub pitch: pitch,
    pub ref_level: libc::c_int,
    pub bitstream: bits_t,
    pub timecode: bits_t,
    pub valid_counter: libc::c_uint,
    pub timecode_ticker: libc::c_uint,
    pub mon: *mut libc::c_uchar,
    pub mon_size: libc::c_int,
    pub mon_counter: libc::c_int,
}
pub type bits_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pitch {
    pub dt: libc::c_double,
    pub x: libc::c_double,
    pub v: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timecoder_channel {
    pub positive: bool,
    pub swapped: bool,
    pub zero: libc::c_int,
    pub crossing_ticker: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timecode_def {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub bits: libc::c_int,
    pub resolution: libc::c_int,
    pub flags: libc::c_int,
    pub seed: bits_t,
    pub taps: bits_t,
    pub length: libc::c_uint,
    pub safe: libc::c_uint,
    pub lookup: bool,
    pub lut: lut,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lut {
    pub slot: *mut slot,
    pub table: *mut slot_no_t,
    pub avail: slot_no_t,
}
pub type slot_no_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slot {
    pub timecode: libc::c_uint,
    pub next: slot_no_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct track {
    pub tracks: list,
    pub refcount: libc::c_uint,
    pub rate: libc::c_int,
    pub importer: *const libc::c_char,
    pub path: *const libc::c_char,
    pub bytes: size_t,
    pub length: libc::c_uint,
    pub blocks: libc::c_uint,
    pub block: [*mut track_block; 64],
    pub rig: list,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub pe: *mut pollfd,
    pub terminated: bool,
    pub ppm: libc::c_ushort,
    pub overview: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub prev: *mut list,
    pub next: *mut list,
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
pub struct track_block {
    pub pcm: [libc::c_short; 4194304],
    pub ppm: [libc::c_uchar; 32768],
    pub overview: [libc::c_uchar; 1024],
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
 * Spinlock routines for synchronising with the realtime thread
 */
pub type spin = pthread_spinlock_t;
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
/* A single music track in our listings */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct record {
    pub pathname: *mut libc::c_char,
    pub artist: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub match_0: *mut libc::c_char,
    pub bpm: libc::c_double,
}
/* or 0.0 if not known */
/* Index points to records, but does not manage those pointers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index {
    pub record: *mut *mut record,
    pub size: size_t,
    pub entries: size_t,
}
/* A 'compiled' search criteria, so we can repeat searches and
 * matches efficiently */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_0 {
    pub buf: [libc::c_char; 512],
    pub words: [*mut libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deck {
    pub device: device,
    pub timecoder: timecoder,
    pub importer: *const libc::c_char,
    pub protect: bool,
    pub player: player,
    pub record: *const record,
    pub cues: cues,
    pub punch: libc::c_double,
    pub ncontrol: size_t,
    pub control: [*mut controller; 4],
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
 * Implementation of "observer pattern"
 *
 * There are several cases in the code where we need to notify
 * when something changes (eg. to update a UI.)
 *
 * The use of simple function calls is problematic because it creates
 * cyclical dependencies in header files, and is not sufficiently
 * modular to allow the code to be re-used in a self-contained test.
 *
 * So, reluctantly introduce a slots and signals concept; xwax is
 * getting to be quite a lot of code and structure now.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub observers: list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observer {
    pub event: list,
    pub func: Option<unsafe extern "C" fn(_: *mut observer,
                                          _: *mut libc::c_void) -> ()>,
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
/* A set of records, with several optimised indexes */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listing {
    pub by_artist: index,
    pub by_bpm: index,
    pub by_order: index,
    pub addition: event,
}
/* A single crate of records */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crate_0 {
    pub is_fixed: bool,
    pub is_busy: bool,
    pub name: *mut libc::c_char,
    pub listing: *mut listing,
    pub on_addition: observer,
    pub on_completion: observer,
    pub activity: event,
    pub refresh: event,
    pub addition: event,
    pub scan: *const libc::c_char,
    pub path: *const libc::c_char,
    pub excrate: *mut excrate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct excrate {
    pub excrates: list,
    pub refcount: libc::c_uint,
    pub search: *const libc::c_char,
    pub listing: listing,
    pub storage: *mut listing,
    pub completion: event,
    pub rig: list,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub pe: *mut pollfd,
    pub terminated: bool,
    pub rb: rb,
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
/* The complete music library, which consists of multiple crates */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct library {
    pub storage: listing,
    pub all: crate_0,
    pub crate_0: *mut *mut crate_0,
    pub crates: size_t,
}
/*
 * A rectangle defines an on-screen area, and other attributes
 * for anything that gets into the area
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub x: pix_t,
    pub y: pix_t,
    pub w: pix_t,
    pub h: pix_t,
    pub scale: libc::c_float,
}
/* default is our own screen units */
pub type pix_t = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout {
    pub flags: libc::c_uchar,
    pub portion: libc::c_float,
    pub distance: libc::c_uint,
    pub space: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct selector {
    pub library: *mut library,
    pub view_index: *mut index,
    pub swap_index: *mut index,
    pub index_a: index,
    pub index_b: index,
    pub records: listbox,
    pub crates: listbox,
    pub toggled: bool,
    pub toggle_back: libc::c_int,
    pub sort: libc::c_int,
    pub target: *mut record,
    pub on_activity: observer,
    pub on_refresh: observer,
    pub on_addition: observer,
    pub search_len: size_t,
    pub search: [libc::c_char; 256],
    pub match_0: match_0,
    pub changed: event,
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
 * Generic UI listbox widget internals
 */
/* Managed context of a scrolling window, of a number of fixed-height
 * lines, backed by a list of a known number of entries */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listbox {
    pub entries: libc::c_int,
    pub lines: libc::c_int,
    pub offset: libc::c_int,
    pub selected: libc::c_int,
}
/*
 * A callback function for drawing a row. Included here for
 * readability where it is used.
 */
pub type draw_row_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *mut SDL_Surface,
                                _: rect, _: libc::c_uint, _: bool) -> ()>;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(0 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn __list_add(mut new: *mut list, mut prev: *mut list,
                                mut next: *mut list) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}
/*
 * Insert a new list entry after the given head
 */
#[inline]
unsafe extern "C" fn list_add(mut new: *mut list, mut head: *mut list) {
    __list_add(new, head, (*head).next);
}
#[inline]
unsafe extern "C" fn list_del(mut entry: *mut list) {
    let mut next: *mut list = 0 as *mut list;
    let mut prev: *mut list = 0 as *mut list;
    next = (*entry).next;
    prev = (*entry).prev;
    (*next).prev = prev;
    (*prev).next = next;
}
/* Return true if the track importer is running, otherwise false */
#[inline]
unsafe extern "C" fn track_is_importing(mut tr: *mut track) -> bool {
    return (*tr).pid != 0 as libc::c_int;
}
/* Return the pseudo-PPM meter value for the given sample */
#[inline]
unsafe extern "C" fn track_get_ppm(mut tr: *mut track, mut s: libc::c_int)
 -> libc::c_uchar {
    let mut b: *mut track_block = 0 as *mut track_block;
    b =
        (*tr).block[(s / (2048 as libc::c_int * 1024 as libc::c_int)) as
                        usize];
    return (*b).ppm[(s % (2048 as libc::c_int * 1024 as libc::c_int) /
                         64 as libc::c_int) as usize];
}
/* Return the overview meter value for the given sample */
#[inline]
unsafe extern "C" fn track_get_overview(mut tr: *mut track,
                                        mut s: libc::c_int) -> libc::c_uchar {
    let mut b: *mut track_block = 0 as *mut track_block;
    b =
        (*tr).block[(s / (2048 as libc::c_int * 1024 as libc::c_int)) as
                        usize];
    return (*b).overview[(s % (2048 as libc::c_int * 1024 as libc::c_int) /
                              2048 as libc::c_int) as usize];
}
/*
 * The number of revolutions per second of the timecode vinyl,
 * used only for visual display
 */
#[inline]
unsafe extern "C" fn timecoder_revs_per_sec(mut tc: *mut timecoder)
 -> libc::c_double {
    return (33.0f64 + 1.0f64 / 3 as libc::c_int as libc::c_double) *
               (*tc).speed / 60 as libc::c_int as libc::c_double;
}
/*
 * Pre: observer is not watching anything
 * Post: observer is watching the given event
 */
#[inline]
unsafe extern "C" fn watch(mut observer: *mut observer, mut sig: *mut event,
                           mut func:
                               Option<unsafe extern "C" fn(_: *mut observer,
                                                           _:
                                                               *mut libc::c_void)
                                          -> ()>) {
    list_add(&mut (*observer).event, &mut (*sig).observers);
    (*observer).func = func;
}
#[inline]
unsafe extern "C" fn ignore(mut observer: *mut observer) {
    list_del(&mut (*observer).event);
}
/* may be pixels, or units */
/*
 * Helper function to make layout specs
 */
#[inline]
unsafe extern "C" fn absolute(mut flags: libc::c_uchar,
                              mut distance: libc::c_uint,
                              mut space: libc::c_uint) -> layout {
    let mut l: layout = layout{flags: 0, portion: 0., distance: 0, space: 0,};
    l.flags = flags;
    l.portion = 0.0f64 as libc::c_float;
    l.distance = distance;
    l.space = space;
    return l;
}
#[inline]
unsafe extern "C" fn from_left(mut distance: libc::c_uint,
                               mut space: libc::c_uint) -> layout {
    return absolute(0 as libc::c_int as libc::c_uchar, distance, space);
}
#[inline]
unsafe extern "C" fn from_right(mut distance: libc::c_uint,
                                mut space: libc::c_uint) -> layout {
    return absolute(0x2 as libc::c_int as libc::c_uchar, distance, space);
}
#[inline]
unsafe extern "C" fn from_top(mut distance: libc::c_uint,
                              mut space: libc::c_uint) -> layout {
    return absolute(0x1 as libc::c_int as libc::c_uchar, distance, space);
}
#[inline]
unsafe extern "C" fn from_bottom(mut distance: libc::c_uint,
                                 mut space: libc::c_uint) -> layout {
    return absolute((0x1 as libc::c_int | 0x2 as libc::c_int) as
                        libc::c_uchar, distance, space);
}
#[inline]
unsafe extern "C" fn portion(mut flags: libc::c_uchar, mut f: libc::c_double,
                             mut space: libc::c_uint) -> layout {
    let mut l: layout = layout{flags: 0, portion: 0., distance: 0, space: 0,};
    l.flags = flags;
    l.portion = f as libc::c_float;
    l.distance = 0 as libc::c_int as libc::c_uint;
    l.space = space;
    return l;
}
#[inline]
unsafe extern "C" fn columns(mut n: libc::c_uint, mut total: libc::c_uint,
                             mut space: libc::c_uint) -> layout {
    if n < total {
    } else {
        __assert_fail(b"n < total\x00" as *const u8 as *const libc::c_char,
                      b"./layout.h\x00" as *const u8 as *const libc::c_char,
                      106 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 64],
                                                &[libc::c_char; 64]>(b"struct layout columns(unsigned int, unsigned int, unsigned int)\x00")).as_ptr());
    }
    return portion(0 as libc::c_int as libc::c_uchar,
                   1.0f64 / total.wrapping_sub(n) as libc::c_double, space);
}
/*
 * Take an existing layout spec and request that it be in pixels
 *
 * Most dimensions are done in terms of 'screen units' but sometimes
 * we need to apply layout based on a pixel measurement (eg.  returned
 * to us when drawing text)
 */
#[inline]
unsafe extern "C" fn pixels(mut j: layout) -> layout {
    let mut r: layout = layout{flags: 0, portion: 0., distance: 0, space: 0,};
    r = j;
    r.flags = (r.flags as libc::c_int | 0x4 as libc::c_int) as libc::c_uchar;
    return r;
}
/*
 * Create a new rectangle from pixels
 */
#[inline]
unsafe extern "C" fn rect(mut x: pix_t, mut y: pix_t, mut w: pix_t,
                          mut h: pix_t, mut scale_0: libc::c_float) -> rect {
    let mut r: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    r.x = x;
    r.y = y;
    r.w = w;
    r.h = h;
    r.scale = scale_0;
    return r;
}
/*
 * Apply a layout request to split two rectangles
 *
 * The caller is allowed to use the same rectangle for output
 * as is the input.
 */
#[inline]
unsafe extern "C" fn split(x: rect, spec: layout, mut a: *mut rect,
                           mut b: *mut rect) {
    let mut flags: libc::c_uchar =
        0; /* allow caller to re-use x as an output */
    let mut p: libc::c_short = 0;
    let mut q: libc::c_short = 0;
    let mut full: libc::c_short = 0;
    let mut distance: libc::c_short = 0;
    let mut space: libc::c_short = 0;
    let mut discard: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut in_0: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    in_0 = x;
    if a.is_null() { a = &mut discard }
    if b.is_null() { b = &mut discard }
    flags = spec.flags;
    if flags as libc::c_int & 0x1 as libc::c_int != 0 {
        full = in_0.h
    } else { full = in_0.w }
    if flags as libc::c_int & 0x4 as libc::c_int != 0 {
        space = spec.space as libc::c_short;
        distance = spec.distance as libc::c_short
    } else {
        space = (spec.space as libc::c_float * in_0.scale) as libc::c_short;
        distance =
            (spec.distance as libc::c_float * in_0.scale) as libc::c_short
    }
    if spec.portion as libc::c_double != 0.0f64 {
        distance =
            (spec.portion * full as libc::c_int as libc::c_float -
                 (space as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_short
    }
    if flags as libc::c_int & 0x2 as libc::c_int != 0 {
        p =
            (full as libc::c_int - distance as libc::c_int -
                 space as libc::c_int) as libc::c_short;
        q = (full as libc::c_int - distance as libc::c_int) as libc::c_short
    } else {
        p = distance;
        q = (distance as libc::c_int + space as libc::c_int) as libc::c_short
    }
    if flags as libc::c_int & 0x1 as libc::c_int != 0 {
        *a = rect(in_0.x, in_0.y, in_0.w, p, in_0.scale);
        *b =
            rect(in_0.x, (in_0.y as libc::c_int + q as libc::c_int) as pix_t,
                 in_0.w, (in_0.h as libc::c_int - q as libc::c_int) as pix_t,
                 in_0.scale)
    } else {
        *a = rect(in_0.x, in_0.y, p, in_0.h, in_0.scale);
        *b =
            rect((in_0.x as libc::c_int + q as libc::c_int) as pix_t, in_0.y,
                 (in_0.w as libc::c_int - q as libc::c_int) as pix_t, in_0.h,
                 in_0.scale)
    };
}
/*
 * Shrink a rectangle to leave a border on all sides
 */
#[inline]
unsafe extern "C" fn shrink(in_0: rect, mut distance: libc::c_int) -> rect {
    let mut out: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    out = in_0;
    distance = (distance as libc::c_float * in_0.scale) as libc::c_int;
    if (distance * 2 as libc::c_int) < in_0.w as libc::c_int {
        out.x = (in_0.x as libc::c_int + distance) as pix_t;
        out.w = (in_0.w as libc::c_int - distance * 2 as libc::c_int) as pix_t
    }
    if (distance * 2 as libc::c_int) < in_0.h as libc::c_int {
        out.y = (in_0.y as libc::c_int + distance) as pix_t;
        out.h = (in_0.h as libc::c_int - distance * 2 as libc::c_int) as pix_t
    }
    return out;
}
/*
 * Calculate the number of lines we can expect to fit if we
 * do splits of the given row height
 */
#[inline]
unsafe extern "C" fn count_rows(mut in_0: rect, mut row_height: libc::c_uint)
 -> libc::c_uint {
    let mut pixels_0: libc::c_uint = 0;
    pixels_0 = (row_height as libc::c_float * in_0.scale) as libc::c_uint;
    return (in_0.h as libc::c_uint).wrapping_div(pixels_0);
}
/* Macro functions */
/* List of directories to use as search path for fonts. */
static mut font_dirs: [*const libc::c_char; 8] =
    [b"/usr/X11R6/lib/X11/fonts/TTF\x00" as *const u8 as *const libc::c_char,
     b"/usr/share/fonts/truetype/ttf-dejavu/\x00" as *const u8 as
         *const libc::c_char,
     b"/usr/share/fonts/ttf-dejavu\x00" as *const u8 as *const libc::c_char,
     b"/usr/share/fonts/dejavu\x00" as *const u8 as *const libc::c_char,
     b"/usr/share/fonts/TTF\x00" as *const u8 as *const libc::c_char,
     b"/usr/share/fonts/truetype/dejavu\x00" as *const u8 as
         *const libc::c_char,
     b"/usr/share/fonts/truetype/ttf-dejavu\x00" as *const u8 as
         *const libc::c_char, 0 as *const libc::c_char];
static mut clock_font: *mut TTF_Font = 0 as *const TTF_Font as *mut TTF_Font;
static mut deci_font: *mut TTF_Font = 0 as *const TTF_Font as *mut TTF_Font;
static mut detail_font: *mut TTF_Font = 0 as *const TTF_Font as *mut TTF_Font;
static mut font: *mut TTF_Font = 0 as *const TTF_Font as *mut TTF_Font;
static mut em_font: *mut TTF_Font = 0 as *const TTF_Font as *mut TTF_Font;
static mut big_font: *mut TTF_Font = 0 as *const TTF_Font as *mut TTF_Font;
static mut background_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 0 as libc::c_int as Uint8,
                      g: 0 as libc::c_int as Uint8,
                      b: 0 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut text_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 224 as libc::c_int as Uint8,
                      g: 224 as libc::c_int as Uint8,
                      b: 224 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut alert_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 192 as libc::c_int as Uint8,
                      g: 64 as libc::c_int as Uint8,
                      b: 0 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut ok_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 32 as libc::c_int as Uint8,
                      g: 128 as libc::c_int as Uint8,
                      b: 3 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut elapsed_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 0 as libc::c_int as Uint8,
                      g: 32 as libc::c_int as Uint8,
                      b: 255 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut cursor_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 192 as libc::c_int as Uint8,
                      g: 0 as libc::c_int as Uint8,
                      b: 0 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut selected_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 0 as libc::c_int as Uint8,
                      g: 48 as libc::c_int as Uint8,
                      b: 64 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut detail_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 128 as libc::c_int as Uint8,
                      g: 128 as libc::c_int as Uint8,
                      b: 128 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut needle_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 255 as libc::c_int as Uint8,
                      g: 255 as libc::c_int as Uint8,
                      b: 255 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut artist_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 16 as libc::c_int as Uint8,
                      g: 64 as libc::c_int as Uint8,
                      b: 0 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut bpm_col: SDL_Color =
    {
        let mut init =
            SDL_Color{r: 64 as libc::c_int as Uint8,
                      g: 16 as libc::c_int as Uint8,
                      b: 0 as libc::c_int as Uint8,
                      unused: 255 as libc::c_int as Uint8,};
        init
    };
static mut spinner_angle: *mut libc::c_ushort =
    0 as *const libc::c_ushort as *mut libc::c_ushort;
static mut spinner_size: libc::c_ushort = 0;
static mut width: libc::c_int = 960 as libc::c_int;
static mut height: libc::c_int = 720 as libc::c_int;
static mut meter_scale: libc::c_int = 8 as libc::c_int;
static mut video_flags: Uint32 = 0x10 as libc::c_int as Uint32;
static mut scale: libc::c_float = 1.0f64 as libc::c_float;
static mut utf: iconv_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut ph: pthread_t = 0;
static mut selector: selector =
    selector{library: 0 as *const library as *mut library,
             view_index: 0 as *const index as *mut index,
             swap_index: 0 as *const index as *mut index,
             index_a:
                 index{record: 0 as *const *mut record as *mut *mut record,
                       size: 0,
                       entries: 0,},
             index_b:
                 index{record: 0 as *const *mut record as *mut *mut record,
                       size: 0,
                       entries: 0,},
             records: listbox{entries: 0, lines: 0, offset: 0, selected: 0,},
             crates: listbox{entries: 0, lines: 0, offset: 0, selected: 0,},
             toggled: false,
             toggle_back: 0,
             sort: 0,
             target: 0 as *const record as *mut record,
             on_activity:
                 observer{event:
                              list{prev: 0 as *const list as *mut list,
                                   next: 0 as *const list as *mut list,},
                          func: None,},
             on_refresh:
                 observer{event:
                              list{prev: 0 as *const list as *mut list,
                                   next: 0 as *const list as *mut list,},
                          func: None,},
             on_addition:
                 observer{event:
                              list{prev: 0 as *const list as *mut list,
                                   next: 0 as *const list as *mut list,},
                          func: None,},
             search_len: 0,
             search: [0; 256],
             match_0:
                 match_0{buf: [0; 512],
                         words:
                             [0 as *const libc::c_char as *mut libc::c_char;
                                 32],},
             changed:
                 event{observers:
                           list{prev: 0 as *const list as *mut list,
                                next: 0 as *const list as *mut list,},},};
static mut on_status: observer =
    observer{event:
                 list{prev: 0 as *const list as *mut list,
                      next: 0 as *const list as *mut list,},
             func: None,};
static mut on_selector: observer =
    observer{event:
                 list{prev: 0 as *const list as *mut list,
                      next: 0 as *const list as *mut list,},
             func: None,};
/*
 * Scale a dimension according to the current zoom level
 *
 * FIXME: This function is used where a rendering does not
 * acknowledge the scale given in the local rectangle.
 * These cases should be removed.
 */
unsafe extern "C" fn zoom(mut d: libc::c_int) -> libc::c_int {
    return (d as libc::c_float * scale) as libc::c_int;
}
/*
 * Convert the given time (in milliseconds) to displayable time
 */
unsafe extern "C" fn time_to_clock(mut buf: *mut libc::c_char,
                                   mut deci: *mut libc::c_char,
                                   mut t: libc::c_int) {
    let mut minutes: libc::c_int = 0;
    let mut seconds: libc::c_int = 0;
    let mut frac: libc::c_int = 0;
    let mut neg: bool = false;
    if t < 0 as libc::c_int {
        t = abs(t);
        neg = 1 as libc::c_int != 0
    } else { neg = 0 as libc::c_int != 0 }
    minutes =
        t / 60 as libc::c_int / 1000 as libc::c_int %
            (60 as libc::c_int * 60 as libc::c_int);
    seconds = t / 1000 as libc::c_int % 60 as libc::c_int;
    frac = t % 1000 as libc::c_int;
    if neg {
        let fresh0 = buf;
        buf = buf.offset(1);
        *fresh0 = '-' as i32 as libc::c_char
    }
    sprintf(buf, b"%02d:%02d.\x00" as *const u8 as *const libc::c_char,
            minutes, seconds);
    sprintf(deci, b"%03d\x00" as *const u8 as *const libc::c_char, frac);
}
/*
 * Calculate a lookup which maps a position on screen to an angle,
 * relative to the centre of the spinner
 */
unsafe extern "C" fn calculate_angle_lut(mut lut: *mut libc::c_ushort,
                                         mut size: libc::c_int) {
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut nr: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut theta: libc::c_float = 0.;
    let mut rat: libc::c_float = 0.;
    r = 0 as libc::c_int;
    while r < size {
        nr = r - size / 2 as libc::c_int;
        c = 0 as libc::c_int;
        while c < size {
            nc = c - size / 2 as libc::c_int;
            if nr == 0 as libc::c_int {
                theta = 1.57079632679489661923f64 as libc::c_float
            } else if nc == 0 as libc::c_int {
                theta = 0 as libc::c_int as libc::c_float;
                if nr < 0 as libc::c_int {
                    theta = 3.14159265358979323846f64 as libc::c_float
                }
            } else {
                rat = nc as libc::c_float / -nr as libc::c_float;
                theta = atanf(rat);
                if rat < 0 as libc::c_int as libc::c_float {
                    theta =
                        (theta as libc::c_double + 3.14159265358979323846f64)
                            as libc::c_float
                }
            }
            if nc <= 0 as libc::c_int {
                theta =
                    (theta as libc::c_double + 3.14159265358979323846f64) as
                        libc::c_float
            }
            /* The angles stored in the lookup table range from 0 to
             * 1023 (where 1024 is 360 degrees) */
            *lut.offset((r * size + c) as isize) =
                ((((theta * 1024 as libc::c_int as libc::c_float) as
                       libc::c_double /
                       (3.14159265358979323846f64 *
                            2 as libc::c_int as libc::c_double)) as
                      libc::c_int + 1024 as libc::c_int) %
                     1024 as libc::c_int) as libc::c_ushort;
            c += 1
        }
        r += 1
    };
}
unsafe extern "C" fn init_spinner(mut size: libc::c_int) -> libc::c_int {
    spinner_angle =
        malloc(((size * size) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                    as libc::c_ulong)) as
            *mut libc::c_ushort;
    if spinner_angle.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    calculate_angle_lut(spinner_angle, size);
    spinner_size = size as libc::c_ushort;
    return 0 as libc::c_int;
}
unsafe extern "C" fn clear_spinner() {
    free(spinner_angle as *mut libc::c_void);
}
/*
 * Open a font, given the leafname
 *
 * This scans the available font directories for the file, to account
 * for different software distributions.
 *
 * As this is an SDL (it is not an X11 app) we prefer to avoid the use
 * of fontconfig to select fonts.
 */
unsafe extern "C" fn open_font(mut name: *const libc::c_char,
                               mut size: libc::c_int) -> *mut TTF_Font {
    let mut r: libc::c_int = 0;
    let mut pt: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut dir: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_mode: 0,
             st_nlink: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             __pad1: 0,
             st_size: 0,
             st_blksize: 0,
             __pad2: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 2],};
    let mut font_0: *mut TTF_Font = 0 as *mut TTF_Font;
    pt = zoom(size);
    dir =
        &mut *font_dirs.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut *const libc::c_char;
    while !(*dir).is_null() {
        sprintf(buf.as_mut_ptr(),
                b"%s/%s\x00" as *const u8 as *const libc::c_char, *dir, name);
        r = stat(buf.as_mut_ptr(), &mut st);
        if r != -(1 as libc::c_int) {
            /* something exists at this path */
            fprintf(stderr,
                    b"Loading font \'%s\', %dpt...\n\x00" as *const u8 as
                        *const libc::c_char, buf.as_mut_ptr(), pt);
            font_0 = TTF_OpenFont(buf.as_mut_ptr(), pt);
            if font_0.is_null() {
                fprintf(stderr,
                        b"Font error: %s\n\x00" as *const u8 as
                            *const libc::c_char, SDL_GetError());
            }
            return font_0
            /* or NULL */
        }
        if *__errno_location() != 2 as libc::c_int {
            perror(b"stat\x00" as *const u8 as *const libc::c_char);
            return 0 as *mut TTF_Font
        }
        dir = dir.offset(1)
    }
    fprintf(stderr,
            b"Font \'%s\' cannot be found in\x00" as *const u8 as
                *const libc::c_char, name);
    dir =
        &mut *font_dirs.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut *const libc::c_char;
    while !(*dir).is_null() {
        fputc(' ' as i32, stderr);
        fputs(*dir, stderr);
        dir = dir.offset(1)
    }
    fputc('.' as i32, stderr);
    fputc('\n' as i32, stderr);
    return 0 as *mut TTF_Font;
}
/*
 * Load all fonts
 */
unsafe extern "C" fn load_fonts() -> libc::c_int {
    clock_font =
        open_font(b"DejaVuSans.ttf\x00" as *const u8 as *const libc::c_char,
                  32 as libc::c_int);
    if clock_font.is_null() { return -(1 as libc::c_int) }
    deci_font =
        open_font(b"DejaVuSans.ttf\x00" as *const u8 as *const libc::c_char,
                  20 as libc::c_int);
    if deci_font.is_null() { return -(1 as libc::c_int) }
    font =
        open_font(b"DejaVuSans.ttf\x00" as *const u8 as *const libc::c_char,
                  10 as libc::c_int);
    if font.is_null() { return -(1 as libc::c_int) }
    em_font =
        open_font(b"DejaVuSans-Oblique.ttf\x00" as *const u8 as
                      *const libc::c_char, 10 as libc::c_int);
    if em_font.is_null() { return -(1 as libc::c_int) }
    big_font =
        open_font(b"DejaVuSans-Bold.ttf\x00" as *const u8 as
                      *const libc::c_char, 14 as libc::c_int);
    if big_font.is_null() { return -(1 as libc::c_int) }
    detail_font =
        open_font(b"DejaVuSansMono-Bold.ttf\x00" as *const u8 as
                      *const libc::c_char, 9 as libc::c_int);
    if detail_font.is_null() { return -(1 as libc::c_int) }
    return 0 as libc::c_int;
}
/*
 * Free resources associated with fonts
 */
unsafe extern "C" fn clear_fonts() {
    TTF_CloseFont(clock_font);
    TTF_CloseFont(deci_font);
    TTF_CloseFont(font);
    TTF_CloseFont(em_font);
    TTF_CloseFont(big_font);
    TTF_CloseFont(detail_font);
}
unsafe extern "C" fn palette(mut sf: *mut SDL_Surface,
                             mut col: *mut SDL_Color) -> Uint32 {
    return SDL_MapRGB((*sf).format, (*col).r, (*col).g, (*col).b);
}
/*
 * Draw text
 *
 * Render the string "buf" text inside the given "rect".  If "locale"
 * is set then a conversion from the system locale is done.
 *
 * Return: width of text drawn
 */
unsafe extern "C" fn do_draw_text(mut sf: *mut SDL_Surface,
                                  mut rect_0: *const rect,
                                  mut buf: *const libc::c_char,
                                  mut font_0: *mut TTF_Font,
                                  mut fg: SDL_Color, mut bg: SDL_Color,
                                  mut locale: bool) -> libc::c_int {
    let mut rendered: *mut SDL_Surface = 0 as *mut SDL_Surface;
    let mut dst: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
    let mut src: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
    let mut fill: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
    if buf.is_null() {
        src.w = 0 as libc::c_int as Uint16;
        src.h = 0 as libc::c_int as Uint16
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  '\u{0}' as i32 {
        /* SDL_ttf fails for empty string */
        src.w = 0 as libc::c_int as Uint16; /* always leave space for \0 */
        src.h = 0 as libc::c_int as Uint16
    } else {
        if !locale {
            rendered = TTF_RenderText_Shaded(font_0, buf, fg, bg)
        } else {
            let mut ubuf: [libc::c_char; 256] = [0; 256];
            let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: size_t = 0;
            let mut fill_0: size_t = 0;
            out = ubuf.as_mut_ptr();
            fill_0 =
                (::std::mem::size_of::<[libc::c_char; 256]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong);
            if iconv(utf, 0 as *mut *mut libc::c_char, 0 as *mut size_t,
                     &mut out, &mut fill_0) ==
                   -(1 as libc::c_int) as libc::c_ulong {
                abort();
            }
            in_0 =
                ({
                     let mut __old: *const libc::c_char = buf;
                     let mut __len: size_t =
                         strlen(__old).wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong);
                     let mut fresh1 =
                         ::std::vec::from_elem(0, __len as usize);
                     let mut __new: *mut libc::c_char =
                         fresh1.as_mut_ptr() as *mut libc::c_char;
                     memcpy(__new as *mut libc::c_void,
                            __old as *const libc::c_void, __len) as
                         *mut libc::c_char
                 });
            len = strlen(in_0);
            iconv(utf, &mut in_0, &mut len, &mut out, &mut fill_0);
            *out = '\u{0}' as i32 as libc::c_char;
            rendered =
                TTF_RenderUTF8_Shaded(font_0, ubuf.as_mut_ptr(), fg, bg)
        }
        src.x = 0 as libc::c_int as Sint16;
        src.y = 0 as libc::c_int as Sint16;
        src.w =
            if ((*rect_0).w as libc::c_int) < (*rendered).w {
                (*rect_0).w as libc::c_int
            } else { (*rendered).w } as Uint16;
        src.h =
            if ((*rect_0).h as libc::c_int) < (*rendered).h {
                (*rect_0).h as libc::c_int
            } else { (*rendered).h } as Uint16;
        dst.x = (*rect_0).x;
        dst.y = (*rect_0).y;
        SDL_UpperBlit(rendered, &mut src, sf, &mut dst);
        SDL_FreeSurface(rendered);
    }
    /* Complete the remaining space with a blank rectangle */
    if (src.w as libc::c_int) < (*rect_0).w as libc::c_int {
        fill.x =
            ((*rect_0).x as libc::c_int + src.w as libc::c_int) as
                Sint16; /* the x-fill rectangle does the corner */
        fill.y = (*rect_0).y;
        fill.w =
            ((*rect_0).w as libc::c_int - src.w as libc::c_int) as Uint16;
        fill.h = (*rect_0).h as Uint16;
        SDL_FillRect(sf, &mut fill, palette(sf, &mut bg));
    }
    if (src.h as libc::c_int) < (*rect_0).h as libc::c_int {
        fill.x = (*rect_0).x;
        fill.y =
            ((*rect_0).y as libc::c_int + src.h as libc::c_int) as Sint16;
        fill.w = src.w;
        fill.h =
            ((*rect_0).h as libc::c_int - src.h as libc::c_int) as Uint16;
        SDL_FillRect(sf, &mut fill, palette(sf, &mut bg));
    }
    return src.w as libc::c_int;
}
unsafe extern "C" fn draw_text(mut sf: *mut SDL_Surface,
                               mut rect_0: *const rect,
                               mut buf: *const libc::c_char,
                               mut font_0: *mut TTF_Font, mut fg: SDL_Color,
                               mut bg: SDL_Color) -> libc::c_int {
    return do_draw_text(sf, rect_0, buf, font_0, fg, bg,
                        0 as libc::c_int != 0);
}
unsafe extern "C" fn draw_text_in_locale(mut sf: *mut SDL_Surface,
                                         mut rect_0: *const rect,
                                         mut buf: *const libc::c_char,
                                         mut font_0: *mut TTF_Font,
                                         mut fg: SDL_Color, mut bg: SDL_Color)
 -> libc::c_int {
    return do_draw_text(sf, rect_0, buf, font_0, fg, bg,
                        1 as libc::c_int != 0);
}
/*
 * Given a rectangle and font, calculate rendering bounds
 * for another font so that the baseline matches.
 */
unsafe extern "C" fn track_baseline(mut rect_0: *const rect,
                                    mut a: *const TTF_Font,
                                    mut aligned: *mut rect,
                                    mut b: *const TTF_Font) {
    split(*rect_0,
          pixels(from_top((TTF_FontAscent(a) - TTF_FontAscent(b)) as
                              libc::c_uint,
                          0 as libc::c_int as libc::c_uint)), 0 as *mut rect,
          aligned);
}
/*
 * Draw a coloured rectangle
 */
unsafe extern "C" fn draw_rect(mut surface: *mut SDL_Surface,
                               mut rect_0: *const rect, mut col: SDL_Color) {
    let mut b: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
    b.x = (*rect_0).x;
    b.y = (*rect_0).y;
    b.w = (*rect_0).w as Uint16;
    b.h = (*rect_0).h as Uint16;
    SDL_FillRect(surface, &mut b, palette(surface, &mut col));
}
/*
 * Draw some text in a box
 */
unsafe extern "C" fn draw_token(mut surface: *mut SDL_Surface,
                                mut rect_0: *const rect,
                                mut buf: *const libc::c_char,
                                mut text_col_0: SDL_Color, mut col: SDL_Color,
                                mut bg_col: SDL_Color) {
    let mut b: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    draw_rect(surface, rect_0, bg_col);
    b = shrink(*rect_0, 2 as libc::c_int);
    draw_text(surface, &mut b, buf, detail_font, text_col_0, col);
}
/*
 * Dim a colour for display
 */
unsafe extern "C" fn dim(x: SDL_Color, mut n: libc::c_int) -> SDL_Color {
    let mut c: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    c.r = (x.r as libc::c_int >> n) as Uint8;
    c.g = (x.g as libc::c_int >> n) as Uint8;
    c.b = (x.b as libc::c_int >> n) as Uint8;
    return c;
}
/*
 * Get a colour from RGB values
 */
unsafe extern "C" fn rgb(mut r: libc::c_double, mut g: libc::c_double,
                         mut b: libc::c_double) -> SDL_Color {
    let mut c: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    c.r = (r * 255 as libc::c_int as libc::c_double) as Uint8;
    c.g = (g * 255 as libc::c_int as libc::c_double) as Uint8;
    c.b = (b * 255 as libc::c_int as libc::c_double) as Uint8;
    return c;
}
/*
 * Get a colour from HSV values
 *
 * Pre: h is in degrees, in the range 0.0 to 360.0
 */
unsafe extern "C" fn hsv(mut h: libc::c_double, mut s: libc::c_double,
                         mut v: libc::c_double) -> SDL_Color {
    let mut i: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    if s == 0.0f64 { return rgb(v, v, v) }
    h /= 60 as libc::c_int as libc::c_double;
    i = floor(h) as libc::c_int;
    f = h - i as libc::c_double;
    p = v * (1 as libc::c_int as libc::c_double - s);
    q = v * (1 as libc::c_int as libc::c_double - s * f);
    t =
        v *
            (1 as libc::c_int as libc::c_double -
                 s * (1 as libc::c_int as libc::c_double - f));
    match i {
        0 => { return rgb(v, t, p) }
        1 => { return rgb(q, v, p) }
        2 => { return rgb(p, v, t) }
        3 => { return rgb(p, q, v) }
        4 => { return rgb(t, p, v) }
        5 | 6 => { return rgb(v, p, q) }
        _ => { abort(); }
    };
}
unsafe extern "C" fn show_bpm(mut bpm: libc::c_double) -> bool {
    return bpm > 20.0f64 && bpm < 400.0f64;
}
/*
 * Draw the beats-per-minute indicator
 */
unsafe extern "C" fn draw_bpm(mut surface: *mut SDL_Surface,
                              mut rect_0: *const rect,
                              mut bpm: libc::c_double,
                              mut bg_col: SDL_Color) {
    static mut min: libc::c_double = 60.0f64;
    static mut max: libc::c_double = 240.0f64;
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut f: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    sprintf(buf.as_mut_ptr(),
            b"%5.1f\x00" as *const u8 as *const libc::c_char, bpm);
    /* Safety catch against bad BPM values, NaN, infinity etc. */
    if bpm < min || bpm > max {
        draw_token(surface, rect_0, buf.as_mut_ptr(), detail_col, bg_col,
                   bg_col);
        return
    }
    /* Colour compatible BPMs the same; cycle 360 degrees
     * every time the BPM doubles */
    f = log2(bpm); /* degrees */
    f -= floor(f);
    h = f * 360.0f64;
    draw_token(surface, rect_0, buf.as_mut_ptr(), text_col,
               hsv(h, 1.0f64, 0.3f64), bg_col);
}
/*
 * Draw the BPM field, or a gap
 */
unsafe extern "C" fn draw_bpm_field(mut surface: *mut SDL_Surface,
                                    mut rect_0: *const rect,
                                    mut bpm: libc::c_double,
                                    mut bg_col: SDL_Color) {
    if show_bpm(bpm) {
        draw_bpm(surface, rect_0, bpm, bg_col);
    } else { draw_rect(surface, rect_0, bg_col); };
}
/*
 * Draw the record information in the deck
 */
unsafe extern "C" fn draw_record(mut surface: *mut SDL_Surface,
                                 mut rect_0: *const rect,
                                 mut record: *const record) {
    let mut artist: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut title: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut left: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut right: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    split(*rect_0,
          from_top(19 as libc::c_int as libc::c_uint,
                   0 as libc::c_int as libc::c_uint), &mut artist,
          &mut title);
    draw_text_in_locale(surface, &mut artist, (*record).artist, big_font,
                        text_col, background_col);
    /* Layout changes slightly if BPM is known */
    if show_bpm((*record).bpm) {
        split(title,
              from_left(32 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint), &mut left,
              &mut right);
        draw_bpm(surface, &mut left, (*record).bpm, background_col);
        split(right,
              from_left(4 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint), &mut left,
              &mut title);
        draw_rect(surface, &mut left, background_col);
    }
    draw_text_in_locale(surface, &mut title, (*record).title, font, text_col,
                        background_col);
}
/*
 * Draw a single time in milliseconds in hours:minutes.seconds format
 */
unsafe extern "C" fn draw_clock(mut surface: *mut SDL_Surface,
                                mut rect_0: *const rect, mut t: libc::c_int,
                                mut col: SDL_Color) {
    let mut hms: [libc::c_char; 8] = [0; 8];
    let mut deci: [libc::c_char; 8] = [0; 8];
    let mut v: libc::c_short = 0;
    let mut sr: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    time_to_clock(hms.as_mut_ptr(), deci.as_mut_ptr(), t);
    v =
        draw_text(surface, rect_0, hms.as_mut_ptr(), clock_font, col,
                  background_col) as libc::c_short;
    split(*rect_0,
          pixels(from_left(v as libc::c_uint,
                           0 as libc::c_int as libc::c_uint)), 0 as *mut rect,
          &mut sr);
    track_baseline(&mut sr, clock_font, &mut sr, deci_font);
    draw_text(surface, &mut sr, deci.as_mut_ptr(), deci_font, col,
              background_col);
}
/*
 * Draw the visual monitor of the input audio to the timecoder
 */
unsafe extern "C" fn draw_scope(mut surface: *mut SDL_Surface,
                                mut rect_0: *const rect,
                                mut tc: *mut timecoder) {
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    let mut p: *mut Uint8 = 0 as *mut Uint8;
    mid = (*tc).mon_size / 2 as libc::c_int;
    r = 0 as libc::c_int;
    while r < (*tc).mon_size {
        c = 0 as libc::c_int;
        while c < (*tc).mon_size {
            p =
                (*surface).pixels.offset((((*rect_0).y as libc::c_int + r) *
                                              (*surface).pitch as libc::c_int)
                                             as
                                             isize).offset((((*rect_0).x as
                                                                 libc::c_int +
                                                                 c) *
                                                                (*(*surface).format).BytesPerPixel
                                                                    as
                                                                    libc::c_int)
                                                               as isize) as
                    *mut Uint8;
            v =
                *(*tc).mon.offset((r * (*tc).mon_size + c) as isize) as
                    libc::c_int;
            if (r == mid || c == mid) && v < 64 as libc::c_int {
                v = 64 as libc::c_int
            }
            *p.offset(0 as libc::c_int as isize) = v as Uint8;
            *p.offset(1 as libc::c_int as isize) =
                *p.offset(0 as libc::c_int as isize);
            *p.offset(2 as libc::c_int as isize) =
                *p.offset(1 as libc::c_int as isize);
            c += 1
        }
        r += 1
    };
}
/*
 * Draw the spinner
 *
 * The spinner shows the rotational position of the record, and
 * matches the physical rotation of the vinyl record.
 */
unsafe extern "C" fn draw_spinner(mut surface: *mut SDL_Surface,
                                  mut rect_0: *const rect,
                                  mut pl: *mut player) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut rangle: libc::c_int = 0;
    let mut pangle: libc::c_int = 0;
    let mut elapsed: libc::c_double = 0.;
    let mut remain: libc::c_double = 0.;
    let mut rps: libc::c_double = 0.;
    let mut rp: *mut Uint8 = 0 as *mut Uint8;
    let mut p: *mut Uint8 = 0 as *mut Uint8;
    let mut col: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    x = (*rect_0).x as libc::c_int;
    y = (*rect_0).y as libc::c_int;
    elapsed = player_get_elapsed(pl);
    remain = player_get_remain(pl);
    rps = timecoder_revs_per_sec((*pl).timecoder);
    rangle =
        (player_get_position(pl) * 1024 as libc::c_int as libc::c_double *
             rps) as libc::c_int % 1024 as libc::c_int;
    if elapsed < 0 as libc::c_int as libc::c_double ||
           remain < 0 as libc::c_int as libc::c_double {
        col = alert_col
    } else { col = ok_col }
    r = 0 as libc::c_int;
    while r < spinner_size as libc::c_int {
        /* Store a pointer to this row of the framebuffer */
        rp =
            (*surface).pixels.offset(((y + r) *
                                          (*surface).pitch as libc::c_int) as
                                         isize) as *mut Uint8;
        c = 0 as libc::c_int;
        while c < spinner_size as libc::c_int {
            /* Use the lookup table to provide the angle at each
             * pixel */
            pangle =
                *spinner_angle.offset((r * spinner_size as libc::c_int + c) as
                                          isize) as libc::c_int;
            /* Calculate the final pixel location and set it */
            p =
                rp.offset(((x + c) *
                               (*(*surface).format).BytesPerPixel as
                                   libc::c_int) as isize);
            if ((rangle - pangle + 1024 as libc::c_int) % 1024 as libc::c_int)
                   < 512 as libc::c_int {
                *p.offset(0 as libc::c_int as isize) =
                    (col.b as libc::c_int >> 2 as libc::c_int) as Uint8;
                *p.offset(1 as libc::c_int as isize) =
                    (col.g as libc::c_int >> 2 as libc::c_int) as Uint8;
                *p.offset(2 as libc::c_int as isize) =
                    (col.r as libc::c_int >> 2 as libc::c_int) as Uint8
            } else {
                *p.offset(0 as libc::c_int as isize) = col.b;
                *p.offset(1 as libc::c_int as isize) = col.g;
                *p.offset(2 as libc::c_int as isize) = col.r
            }
            c += 1
        }
        r += 1
    };
}
/*
 * Draw the clocks which show time elapsed and time remaining
 */
unsafe extern "C" fn draw_deck_clocks(mut surface: *mut SDL_Surface,
                                      mut rect_0: *const rect,
                                      mut pl: *mut player,
                                      mut track: *mut track) {
    let mut elapse: libc::c_int = 0;
    let mut remain: libc::c_int = 0;
    let mut upper: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut lower: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut col: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    split(*rect_0,
          from_top(32 as libc::c_int as libc::c_uint,
                   0 as libc::c_int as libc::c_uint), &mut upper, &mut lower);
    elapse =
        (player_get_elapsed(pl) * 1000 as libc::c_int as libc::c_double) as
            libc::c_int;
    remain =
        (player_get_remain(pl) * 1000 as libc::c_int as libc::c_double) as
            libc::c_int;
    if elapse < 0 as libc::c_int {
        col = alert_col
    } else if remain > 0 as libc::c_int {
        col = ok_col
    } else { col = text_col }
    draw_clock(surface, &mut upper, elapse, col);
    if remain <= 0 as libc::c_int { col = alert_col } else { col = text_col }
    if track_is_importing(track) { col = dim(col, 2 as libc::c_int) }
    draw_clock(surface, &mut lower, -remain, col);
}
/*
 * Draw the high-level overview meter which shows the whole length
 * of the track
 */
unsafe extern "C" fn draw_overview(mut surface: *mut SDL_Surface,
                                   mut rect_0: *const rect,
                                   mut tr: *mut track,
                                   mut position: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut sp: libc::c_int = 0;
    let mut fade: libc::c_int = 0;
    let mut bytes_per_pixel: libc::c_int = 0;
    let mut pitch: libc::c_int = 0;
    let mut height_0: libc::c_int = 0;
    let mut current_position: libc::c_int = 0;
    let mut pixels_0: *mut Uint8 = 0 as *mut Uint8;
    let mut p: *mut Uint8 = 0 as *mut Uint8;
    let mut col: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    x = (*rect_0).x as libc::c_int;
    y = (*rect_0).y as libc::c_int;
    w = (*rect_0).w as libc::c_int;
    h = (*rect_0).h as libc::c_int;
    pixels_0 = (*surface).pixels as *mut Uint8;
    bytes_per_pixel = (*(*surface).format).BytesPerPixel as libc::c_int;
    pitch = (*surface).pitch as libc::c_int;
    if (*tr).length != 0 {
        current_position =
            (position as libc::c_longlong * w as libc::c_longlong /
                 (*tr).length as libc::c_longlong) as libc::c_int
    } else { current_position = 0 as libc::c_int }
    c = 0 as libc::c_int;
    while c < w {
        /* Collect the correct meter value for this column */
        sp =
            ((*tr).length as libc::c_longlong * c as libc::c_longlong /
                 w as libc::c_longlong) as libc::c_int;
        if (sp as libc::c_uint) < (*tr).length {
            /* account for rounding */
            height_0 =
                track_get_overview(tr, sp) as libc::c_int * h /
                    256 as libc::c_int
        } else { height_0 = 0 as libc::c_int }
        /* Choose a base colour to display in */
        if (*tr).length == 0 {
            col = background_col;
            fade = 0 as libc::c_int
        } else if c == current_position {
            col = needle_col;
            fade = 1 as libc::c_int
        } else if position as libc::c_uint >
                      (*tr).length.wrapping_sub(((*tr).rate *
                                                     20 as libc::c_int) as
                                                    libc::c_uint) {
            col = alert_col;
            fade = 3 as libc::c_int
        } else { col = elapsed_col; fade = 3 as libc::c_int }
        if track_is_importing(tr) { col = dim(col, 1 as libc::c_int) }
        if c < current_position { col = dim(col, 1 as libc::c_int) }
        /* Store a pointer to this column of the framebuffer */
        p =
            pixels_0.offset((y * pitch) as
                                isize).offset(((x + c) * bytes_per_pixel) as
                                                  isize);
        r = h;
        while r > height_0 {
            *p.offset(0 as libc::c_int as isize) =
                (col.b as libc::c_int >> fade) as Uint8;
            *p.offset(1 as libc::c_int as isize) =
                (col.g as libc::c_int >> fade) as Uint8;
            *p.offset(2 as libc::c_int as isize) =
                (col.r as libc::c_int >> fade) as Uint8;
            p = p.offset(pitch as isize);
            r -= 1
        }
        while r != 0 {
            *p.offset(0 as libc::c_int as isize) = col.b;
            *p.offset(1 as libc::c_int as isize) = col.g;
            *p.offset(2 as libc::c_int as isize) = col.r;
            p = p.offset(pitch as isize);
            r -= 1
        }
        c += 1
    };
}
/*
 * Draw the close-up meter, which can be zoomed to a level set by
 * 'scale'
 */
unsafe extern "C" fn draw_closeup(mut surface: *mut SDL_Surface,
                                  mut rect_0: *const rect, mut tr: *mut track,
                                  mut position: libc::c_int,
                                  mut scale_0: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut bytes_per_pixel: size_t = 0;
    let mut pitch: size_t = 0;
    let mut pixels_0: *mut Uint8 = 0 as *mut Uint8;
    x = (*rect_0).x as libc::c_int;
    y = (*rect_0).y as libc::c_int;
    w = (*rect_0).w as libc::c_int;
    h = (*rect_0).h as libc::c_int;
    pixels_0 = (*surface).pixels as *mut Uint8;
    bytes_per_pixel = (*(*surface).format).BytesPerPixel as size_t;
    pitch = (*surface).pitch as size_t;
    /* Draw in columns. This may seem like a performance hit,
     * but oprofile shows it makes no difference */
    c = 0 as libc::c_int;
    while c < w {
        let mut r: libc::c_int = 0;
        let mut sp: libc::c_int = 0;
        let mut height_0: libc::c_int = 0;
        let mut fade: libc::c_int = 0;
        let mut p: *mut Uint8 = 0 as *mut Uint8;
        let mut col: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
        /* Work out the meter height in pixels for this column */
        sp =
            position - position % ((1 as libc::c_int) << scale_0) +
                ((c - w / 2 as libc::c_int) << scale_0);
        if (sp as libc::c_uint) < (*tr).length && sp > 0 as libc::c_int {
            height_0 =
                track_get_ppm(tr, sp) as libc::c_int * h / 256 as libc::c_int
        } else { height_0 = 0 as libc::c_int }
        /* Select the appropriate colour */
        if c == w / 2 as libc::c_int {
            col = needle_col;
            fade = 1 as libc::c_int
        } else { col = elapsed_col; fade = 3 as libc::c_int }
        /* Get a pointer to the top of the column, and increment
         * it for each row */
        p =
            pixels_0.offset((y as libc::c_ulong).wrapping_mul(pitch) as
                                isize).offset(((x + c) as
                                                   libc::c_ulong).wrapping_mul(bytes_per_pixel)
                                                  as isize);
        r = h;
        while r > height_0 {
            *p.offset(0 as libc::c_int as isize) =
                (col.b as libc::c_int >> fade) as Uint8;
            *p.offset(1 as libc::c_int as isize) =
                (col.g as libc::c_int >> fade) as Uint8;
            *p.offset(2 as libc::c_int as isize) =
                (col.r as libc::c_int >> fade) as Uint8;
            p = p.offset(pitch as isize);
            r -= 1
        }
        while r != 0 {
            *p.offset(0 as libc::c_int as isize) = col.b;
            *p.offset(1 as libc::c_int as isize) = col.g;
            *p.offset(2 as libc::c_int as isize) = col.r;
            p = p.offset(pitch as isize);
            r -= 1
        }
        c += 1
    };
}
/*
 * Draw the audio meters for a deck
 */
unsafe extern "C" fn draw_meters(mut surface: *mut SDL_Surface,
                                 mut rect_0: *const rect, mut tr: *mut track,
                                 mut position: libc::c_int,
                                 mut scale_0: libc::c_int) {
    let mut overview: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut closeup: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    split(*rect_0,
          from_top(16 as libc::c_int as libc::c_uint,
                   8 as libc::c_int as libc::c_uint), &mut overview,
          &mut closeup);
    if closeup.h as libc::c_int > 16 as libc::c_int {
        draw_overview(surface, &mut overview, tr, position);
    } else { closeup = *rect_0 }
    draw_closeup(surface, &mut closeup, tr, position, scale_0);
}
/*
 * Draw the current playback status -- clocks, spinner and scope
 */
unsafe extern "C" fn draw_deck_top(mut surface: *mut SDL_Surface,
                                   mut rect_0: *const rect,
                                   mut pl: *mut player,
                                   mut track: *mut track) {
    let mut clocks: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut left: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut right: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut spinner: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut scope: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    split(*rect_0,
          from_left(160 as libc::c_int as libc::c_uint,
                    8 as libc::c_int as libc::c_uint), &mut clocks,
          &mut right);
    /* If there is no timecoder to display information on, or not enough
     * available space, just draw clocks which span the overall space */
    if !(*pl).timecode_control || (right.w as libc::c_int) < 0 as libc::c_int
       {
        draw_deck_clocks(surface, rect_0, pl, track);
        return
    }
    draw_deck_clocks(surface, &mut clocks, pl, track);
    split(right,
          from_right((32 as libc::c_int * 2 as libc::c_int - 6 as libc::c_int)
                         as libc::c_uint, 8 as libc::c_int as libc::c_uint),
          &mut left, &mut spinner);
    if (left.w as libc::c_int) < 0 as libc::c_int { return }
    split(spinner,
          from_bottom((32 as libc::c_int * 2 as libc::c_int -
                           6 as libc::c_int) as libc::c_uint,
                      0 as libc::c_int as libc::c_uint), 0 as *mut rect,
          &mut spinner);
    draw_spinner(surface, &mut spinner, pl);
    split(left,
          from_right((32 as libc::c_int * 2 as libc::c_int - 6 as libc::c_int)
                         as libc::c_uint, 8 as libc::c_int as libc::c_uint),
          &mut clocks, &mut scope);
    if (clocks.w as libc::c_int) < 0 as libc::c_int { return }
    split(scope,
          from_bottom((32 as libc::c_int * 2 as libc::c_int -
                           6 as libc::c_int) as libc::c_uint,
                      0 as libc::c_int as libc::c_uint), 0 as *mut rect,
          &mut scope);
    draw_scope(surface, &mut scope, (*pl).timecoder);
}
/*
 * Draw the textual description of playback status, which includes
 * information on the timecode
 */
unsafe extern "C" fn draw_deck_status(mut surface: *mut SDL_Surface,
                                      mut rect_0: *const rect,
                                      mut deck_0: *const deck) {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tc: libc::c_int = 0;
    let mut pl: *const player = &(*deck_0).player;
    c = buf.as_mut_ptr();
    c =
        c.offset(sprintf(c, b"%s: \x00" as *const u8 as *const libc::c_char,
                         (*(*(*pl).timecoder).def).name) as isize);
    tc = timecoder_get_position((*pl).timecoder, 0 as *mut libc::c_double);
    if (*pl).timecode_control as libc::c_int != 0 && tc != -(1 as libc::c_int)
       {
        c =
            c.offset(sprintf(c,
                             b"%7d \x00" as *const u8 as *const libc::c_char,
                             tc) as isize)
    } else {
        c =
            c.offset(sprintf(c,
                             b"        \x00" as *const u8 as
                                 *const libc::c_char) as isize)
    }
    sprintf(c,
            b"pitch:%+0.2f (sync %0.2f %+.5fs = %+0.2f)  %s%s\x00" as
                *const u8 as *const libc::c_char, (*pl).pitch,
            (*pl).sync_pitch, (*pl).last_difference,
            (*pl).pitch * (*pl).sync_pitch,
            if (*pl).recalibrate as libc::c_int != 0 {
                b"RCAL  \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if deck_is_locked(deck_0) as libc::c_int != 0 {
                b"LOCK  \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char });
    draw_text(surface, rect_0, buf.as_mut_ptr(), detail_font, detail_col,
              background_col);
}
/*
 * Draw a single deck
 */
unsafe extern "C" fn draw_deck(mut surface: *mut SDL_Surface,
                               mut rect_0: *const rect, mut deck_0: *mut deck,
                               mut meter_scale_0: libc::c_int) {
    let mut position: libc::c_int = 0;
    let mut track: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut top: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut meters: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut status_0: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rest: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut lower: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut pl: *mut player = 0 as *mut player;
    let mut t: *mut track = 0 as *mut track;
    pl = &mut (*deck_0).player;
    t = (*pl).track;
    position =
        (player_get_elapsed(pl) * (*t).rate as libc::c_double) as libc::c_int;
    split(*rect_0,
          from_top((15 as libc::c_int + 19 as libc::c_int) as libc::c_uint,
                   0 as libc::c_int as libc::c_uint), &mut track, &mut rest);
    if (rest.h as libc::c_int) < 160 as libc::c_int {
        rest = *rect_0
    } else { draw_record(surface, &mut track, (*deck_0).record); }
    split(rest,
          from_top((32 as libc::c_int * 2 as libc::c_int) as libc::c_uint,
                   8 as libc::c_int as libc::c_uint), &mut top, &mut lower);
    if (lower.h as libc::c_int) < 64 as libc::c_int {
        lower = rest
    } else { draw_deck_top(surface, &mut top, pl, t); }
    split(lower,
          from_bottom(15 as libc::c_int as libc::c_uint,
                      8 as libc::c_int as libc::c_uint), &mut meters,
          &mut status_0);
    if (meters.h as libc::c_int) < 64 as libc::c_int {
        meters = lower
    } else { draw_deck_status(surface, &mut status_0, deck_0); }
    draw_meters(surface, &mut meters, t, position, meter_scale_0);
}
/*
 * Draw all the decks in the system left to right
 */
unsafe extern "C" fn draw_decks(mut surface: *mut SDL_Surface,
                                mut rect_0: *const rect,
                                mut deck_0: *mut deck, mut ndecks: size_t,
                                mut meter_scale_0: libc::c_int) {
    let mut d: libc::c_int = 0;
    let mut left: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut right: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    right = *rect_0;
    d = 0 as libc::c_int;
    while (d as libc::c_ulong) < ndecks {
        split(right,
              columns(d as libc::c_uint, ndecks as libc::c_uint,
                      12 as libc::c_int as libc::c_uint), &mut left,
              &mut right);
        draw_deck(surface, &mut left, &mut *deck_0.offset(d as isize),
                  meter_scale_0);
        d += 1
    };
}
/*
 * Draw the status bar
 */
unsafe extern "C" fn draw_status(mut sf: *mut SDL_Surface,
                                 mut rect_0: *const rect) {
    let mut fg: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    let mut bg: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    match status_level() {
        3 | 2 => { fg = text_col; bg = dim(alert_col, 2 as libc::c_int) }
        _ => { fg = detail_col; bg = background_col }
    }
    draw_text_in_locale(sf, rect_0, status(), detail_font, fg, bg);
}
/*
 * Draw the search field which the user types into
 */
unsafe extern "C" fn draw_search(mut surface: *mut SDL_Surface,
                                 mut rect_0: *const rect,
                                 mut sel: *mut selector) {
    let mut s: libc::c_int = 0; /* FIXME: use proper UI funcs */
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut cm: [libc::c_char; 32] = [0; 32];
    let mut cursor: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
    let mut rtext: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    split(*rect_0,
          from_left(10 as libc::c_int as libc::c_uint,
                    8 as libc::c_int as libc::c_uint), 0 as *mut rect,
          &mut rtext);
    if (*sel).search[0 as libc::c_int as usize] as libc::c_int !=
           '\u{0}' as i32 {
        buf = (*sel).search.as_mut_ptr()
    } else { buf = 0 as *const libc::c_char }
    s = draw_text(surface, &mut rtext, buf, font, text_col, background_col);
    cursor.x = (rtext.x as libc::c_int + s) as Sint16;
    cursor.y = rtext.y;
    cursor.w =
        (4 as libc::c_int as libc::c_float * (*rect_0).scale) as Uint16;
    cursor.h = rtext.h as Uint16;
    SDL_FillRect(surface, &mut cursor, palette(surface, &mut cursor_col));
    if (*(*sel).view_index).entries > 1 as libc::c_int as libc::c_ulong {
        sprintf(cm.as_mut_ptr(),
                b"%zd matches\x00" as *const u8 as *const libc::c_char,
                (*(*sel).view_index).entries);
    } else if (*(*sel).view_index).entries > 0 as libc::c_int as libc::c_ulong
     {
        sprintf(cm.as_mut_ptr(),
                b"1 match\x00" as *const u8 as *const libc::c_char);
    } else {
        sprintf(cm.as_mut_ptr(),
                b"no matches\x00" as *const u8 as *const libc::c_char);
    }
    rtext.x =
        (rtext.x as libc::c_int + (s + 4 as libc::c_int + 8 as libc::c_int))
            as pix_t;
    rtext.w =
        (rtext.w as libc::c_int - (s + 4 as libc::c_int + 8 as libc::c_int))
            as pix_t;
    draw_text(surface, &mut rtext, cm.as_mut_ptr(), em_font, detail_col,
              background_col);
}
/*
 * Draw a vertical scroll bar representing our view on a list of the
 * given number of entries
 */
unsafe extern "C" fn draw_scroll_bar(mut surface: *mut SDL_Surface,
                                     mut rect_0: *const rect,
                                     mut scroll: *const listbox) {
    let mut box_0: SDL_Rect = SDL_Rect{x: 0, y: 0, w: 0, h: 0,};
    let mut bg: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    bg = dim(selected_col, 1 as libc::c_int);
    box_0.x = (*rect_0).x;
    box_0.y = (*rect_0).y;
    box_0.w = (*rect_0).w as Uint16;
    box_0.h = (*rect_0).h as Uint16;
    SDL_FillRect(surface, &mut box_0, palette(surface, &mut bg));
    if (*scroll).entries > 0 as libc::c_int {
        box_0.x = (*rect_0).x;
        box_0.y =
            ((*rect_0).y as libc::c_int +
                 (*rect_0).h as libc::c_int * (*scroll).offset /
                     (*scroll).entries) as Sint16;
        box_0.w = (*rect_0).w as Uint16;
        box_0.h =
            ((*rect_0).h as libc::c_int *
                 (if (*scroll).lines < (*scroll).entries {
                      (*scroll).lines
                  } else { (*scroll).entries }) / (*scroll).entries) as
                Uint16;
        SDL_FillRect(surface, &mut box_0,
                     palette(surface, &mut selected_col));
    };
}
/*
 * Draw a listbox, using the given function to draw each row
 */
unsafe extern "C" fn draw_listbox(mut lb: *const listbox,
                                  mut surface: *mut SDL_Surface, rect_0: rect,
                                  mut context: *const libc::c_void,
                                  mut draw: draw_row_t) {
    let mut left: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut remain: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut row: libc::c_uint = 0;
    split(rect_0,
          from_left(10 as libc::c_int as libc::c_uint,
                    8 as libc::c_int as libc::c_uint), &mut left,
          &mut remain);
    draw_scroll_bar(surface, &mut left, lb);
    row = 0 as libc::c_int as libc::c_uint;
    row = 0 as libc::c_int as libc::c_uint;
    loop  {
        let mut entry: libc::c_int = 0;
        let mut selected: bool = false;
        let mut line: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
        entry = listbox_map(lb, row as libc::c_int);
        if entry == -(1 as libc::c_int) { break ; }
        if entry == listbox_current(lb) {
            selected = 1 as libc::c_int != 0
        } else { selected = 0 as libc::c_int != 0 }
        split(remain,
              from_top(15 as libc::c_int as libc::c_uint,
                       0 as libc::c_int as libc::c_uint), &mut line,
              &mut remain);
        draw.expect("non-null function pointer")(context, surface, line,
                                                 entry as libc::c_uint,
                                                 selected);
        row = row.wrapping_add(1)
    }
    draw_rect(surface, &mut remain, background_col);
}
unsafe extern "C" fn draw_crate_row(mut context: *const libc::c_void,
                                    mut surface: *mut SDL_Surface,
                                    rect_0: rect, mut entry: libc::c_uint,
                                    mut selected: bool) {
    let mut selector_0: *const selector = context as *const selector;
    let mut crate_0: *const crate_0 = 0 as *const crate_0;
    let mut left: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut right: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut col: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    crate_0 = *(*(*selector_0).library).crate_0.offset(entry as isize);
    if (*crate_0).is_fixed { col = detail_col } else { col = text_col }
    if !selected {
        draw_text_in_locale(surface, &rect_0, (*crate_0).name, font, col,
                            background_col);
        return
    }
    split(rect_0,
          from_right(21 as libc::c_int as libc::c_uint,
                     0 as libc::c_int as libc::c_uint), &mut left,
          &mut right);
    match (*selector_0).sort {
        0 => {
            draw_token(surface, &mut right,
                       b"ART\x00" as *const u8 as *const libc::c_char,
                       text_col, artist_col, selected_col);
        }
        1 => {
            draw_token(surface, &mut right,
                       b"BPM\x00" as *const u8 as *const libc::c_char,
                       text_col, bpm_col, selected_col);
        }
        2 => {
            draw_token(surface, &mut right,
                       b"PLS\x00" as *const u8 as *const libc::c_char,
                       text_col, selected_col, selected_col);
        }
        _ => { abort(); }
    }
    if (*crate_0).is_busy {
        split(left,
              from_right(25 as libc::c_int as libc::c_uint,
                         0 as libc::c_int as libc::c_uint), &mut left,
              &mut right);
        draw_token(surface, &mut right,
                   b"BUSY\x00" as *const u8 as *const libc::c_char, text_col,
                   dim(alert_col, 2 as libc::c_int), selected_col);
    }
    draw_text_in_locale(surface, &mut left, (*crate_0).name, font, col,
                        selected_col);
}
/*
 * Draw a crate index, with scrollbar and current selection
 */
unsafe extern "C" fn draw_crates(mut surface: *mut SDL_Surface, rect_0: rect,
                                 mut x: *const selector) {
    draw_listbox(&(*x).crates, surface, rect_0, x as *const libc::c_void,
                 Some(draw_crate_row as
                          unsafe extern "C" fn(_: *const libc::c_void,
                                               _: *mut SDL_Surface, _: rect,
                                               _: libc::c_uint, _: bool)
                              -> ()));
}
unsafe extern "C" fn draw_record_row(mut context: *const libc::c_void,
                                     mut surface: *mut SDL_Surface,
                                     rect_0: rect, mut entry: libc::c_uint,
                                     mut selected: bool) {
    let mut width_0: libc::c_int = 0;
    let mut record: *mut record = 0 as *mut record;
    let mut index: *const index = context as *const index;
    let mut left: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut right: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut col: SDL_Color = SDL_Color{r: 0, g: 0, b: 0, unused: 0,};
    if selected { col = selected_col } else { col = background_col }
    width_0 = rect_0.w as libc::c_int / 2 as libc::c_int;
    if width_0 > 200 as libc::c_int { width_0 = 200 as libc::c_int }
    record = *(*index).record.offset(entry as isize);
    split(rect_0,
          from_left(32 as libc::c_int as libc::c_uint,
                    0 as libc::c_int as libc::c_uint), &mut left, &mut right);
    draw_bpm_field(surface, &mut left, (*record).bpm, col);
    split(right,
          from_left(8 as libc::c_int as libc::c_uint,
                    0 as libc::c_int as libc::c_uint), &mut left, &mut right);
    draw_rect(surface, &mut left, col);
    split(right,
          from_left(width_0 as libc::c_uint,
                    0 as libc::c_int as libc::c_uint), &mut left, &mut right);
    draw_text_in_locale(surface, &mut left, (*record).artist, font, text_col,
                        col);
    split(right,
          from_left(8 as libc::c_int as libc::c_uint,
                    0 as libc::c_int as libc::c_uint), &mut left, &mut right);
    draw_rect(surface, &mut left, col);
    draw_text_in_locale(surface, &mut right, (*record).title, font, text_col,
                        col);
}
/*
 * Display a record library index, with scrollbar and current
 * selection
 */
unsafe extern "C" fn draw_index(mut surface: *mut SDL_Surface, rect_0: rect,
                                mut x: *const selector) {
    draw_listbox(&(*x).records, surface, rect_0,
                 (*x).view_index as *const libc::c_void,
                 Some(draw_record_row as
                          unsafe extern "C" fn(_: *const libc::c_void,
                                               _: *mut SDL_Surface, _: rect,
                                               _: libc::c_uint, _: bool)
                              -> ()));
}
/*
 * Display the music library, which consists of the query, and search
 * results
 */
unsafe extern "C" fn draw_library(mut surface: *mut SDL_Surface,
                                  mut rect_0: *const rect,
                                  mut sel: *mut selector) {
    let mut rsearch: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rlists: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rcrates: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rrecords: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rows: libc::c_uint = 0;
    split(*rect_0,
          from_top(15 as libc::c_int as libc::c_uint,
                   8 as libc::c_int as libc::c_uint), &mut rsearch,
          &mut rlists);
    rows = count_rows(rlists, 15 as libc::c_int as libc::c_uint);
    if rows == 0 as libc::c_int as libc::c_uint {
        /* Hide the selector: draw nothing, and make it a 'virtual'
         * one row selector. This is enough to use it from the search
         * field and status only */
        draw_search(surface, rect_0, sel);
        selector_set_lines(sel, 1 as libc::c_int as libc::c_uint);
        return
    }
    draw_search(surface, &mut rsearch, sel);
    selector_set_lines(sel, rows);
    split(rlists,
          columns(0 as libc::c_int as libc::c_uint,
                  4 as libc::c_int as libc::c_uint,
                  8 as libc::c_int as libc::c_uint), &mut rcrates,
          &mut rrecords);
    if rcrates.w as libc::c_int > 64 as libc::c_int {
        draw_index(surface, rrecords, sel);
        draw_crates(surface, rcrates, sel);
    } else { draw_index(surface, *rect_0, sel); };
}
/*
 * Handle a single key event
 *
 * Return: true if the selector needs to be redrawn, otherwise false
 */
unsafe extern "C" fn handle_key(mut key: SDLKey, mut mod_0: SDLMod) -> bool {
    let mut sel: *mut selector = &mut selector;
    if key as libc::c_uint >= SDLK_a as libc::c_int as libc::c_uint &&
           key as libc::c_uint <= SDLK_z as libc::c_int as libc::c_uint {
        selector_search_refine(sel,
                               (key as
                                    libc::c_uint).wrapping_sub(SDLK_a as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint).wrapping_add('a'
                                                                                                  as
                                                                                                  i32
                                                                                                  as
                                                                                                  libc::c_uint)
                                   as libc::c_char);
        return 1 as libc::c_int != 0
    } else {
        if key as libc::c_uint >= SDLK_0 as libc::c_int as libc::c_uint &&
               key as libc::c_uint <= SDLK_9 as libc::c_int as libc::c_uint {
            selector_search_refine(sel,
                                   (key as
                                        libc::c_uint).wrapping_sub(SDLK_0 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_add('0'
                                                                                                      as
                                                                                                      i32
                                                                                                      as
                                                                                                      libc::c_uint)
                                       as libc::c_char);
            return 1 as libc::c_int != 0
        } else {
            if key as libc::c_uint ==
                   SDLK_SPACE as libc::c_int as libc::c_uint {
                selector_search_refine(sel, ' ' as i32 as libc::c_char);
                return 1 as libc::c_int != 0
            } else {
                if key as libc::c_uint ==
                       SDLK_BACKSPACE as libc::c_int as libc::c_uint {
                    selector_search_expand(sel);
                    return 1 as libc::c_int != 0
                } else {
                    if key as libc::c_uint ==
                           SDLK_PERIOD as libc::c_int as libc::c_uint {
                        selector_search_refine(sel,
                                               '.' as i32 as libc::c_char);
                        return 1 as libc::c_int != 0
                    } else {
                        if key as libc::c_uint ==
                               SDLK_HOME as libc::c_int as libc::c_uint {
                            selector_top(sel);
                            return 1 as libc::c_int != 0
                        } else {
                            if key as libc::c_uint ==
                                   SDLK_END as libc::c_int as libc::c_uint {
                                selector_bottom(sel);
                                return 1 as libc::c_int != 0
                            } else {
                                if key as libc::c_uint ==
                                       SDLK_UP as libc::c_int as libc::c_uint
                                   {
                                    selector_up(sel);
                                    return 1 as libc::c_int != 0
                                } else {
                                    if key as libc::c_uint ==
                                           SDLK_DOWN as libc::c_int as
                                               libc::c_uint {
                                        selector_down(sel);
                                        return 1 as libc::c_int != 0
                                    } else {
                                        if key as libc::c_uint ==
                                               SDLK_PAGEUP as libc::c_int as
                                                   libc::c_uint {
                                            selector_page_up(sel);
                                            return 1 as libc::c_int != 0
                                        } else {
                                            if key as libc::c_uint ==
                                                   SDLK_PAGEDOWN as
                                                       libc::c_int as
                                                       libc::c_uint {
                                                selector_page_down(sel);
                                                return 1 as libc::c_int != 0
                                            } else {
                                                if key as libc::c_uint ==
                                                       SDLK_LEFT as
                                                           libc::c_int as
                                                           libc::c_uint {
                                                    selector_prev(sel);
                                                    return 1 as libc::c_int !=
                                                               0
                                                } else {
                                                    if key as libc::c_uint ==
                                                           SDLK_RIGHT as
                                                               libc::c_int as
                                                               libc::c_uint {
                                                        selector_next(sel);
                                                        return 1 as
                                                                   libc::c_int
                                                                   != 0
                                                    } else {
                                                        if key as libc::c_uint
                                                               ==
                                                               SDLK_TAB as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                           {
                                                            if mod_0 as
                                                                   libc::c_uint
                                                                   &
                                                                   (KMOD_LCTRL
                                                                        as
                                                                        libc::c_int
                                                                        |
                                                                        KMOD_RCTRL
                                                                            as
                                                                            libc::c_int)
                                                                       as
                                                                       libc::c_uint
                                                                   != 0 {
                                                                if mod_0 as
                                                                       libc::c_uint
                                                                       &
                                                                       (KMOD_LSHIFT
                                                                            as
                                                                            libc::c_int
                                                                            |
                                                                            KMOD_RSHIFT
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           libc::c_uint
                                                                       != 0 {
                                                                    selector_rescan(sel);
                                                                } else {
                                                                    selector_toggle_order(sel);
                                                                }
                                                            } else {
                                                                selector_toggle(sel);
                                                            }
                                                            return 1 as
                                                                       libc::c_int
                                                                       != 0
                                                        } else {
                                                            if key as
                                                                   libc::c_uint
                                                                   ==
                                                                   SDLK_EQUALS
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                                   ||
                                                                   key as
                                                                       libc::c_uint
                                                                       ==
                                                                       SDLK_PLUS
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint
                                                               {
                                                                meter_scale -=
                                                                    1;
                                                                if meter_scale
                                                                       <
                                                                       0 as
                                                                           libc::c_int
                                                                   {
                                                                    meter_scale
                                                                        =
                                                                        0 as
                                                                            libc::c_int
                                                                }
                                                                fprintf(stderr,
                                                                        b"Meter scale decreased to %d\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        meter_scale);
                                                            } else if key as
                                                                          libc::c_uint
                                                                          ==
                                                                          SDLK_MINUS
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint
                                                             {
                                                                meter_scale +=
                                                                    1;
                                                                if meter_scale
                                                                       >
                                                                       11 as
                                                                           libc::c_int
                                                                   {
                                                                    meter_scale
                                                                        =
                                                                        11 as
                                                                            libc::c_int
                                                                }
                                                                fprintf(stderr,
                                                                        b"Meter scale increased to %d\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        meter_scale);
                                                            } else if key as
                                                                          libc::c_uint
                                                                          >=
                                                                          SDLK_F1
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint
                                                                          &&
                                                                          key
                                                                              as
                                                                              libc::c_uint
                                                                              <=
                                                                              SDLK_F12
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint
                                                             {
                                                                let mut d:
                                                                        size_t =
                                                                    0;
                                                                /* Handle the function key press in groups of four --
	 * F1-F4 (deck 0), F5-F8 (deck 1) etc. */
                                                                d =
                                                                    (key as
                                                                         libc::c_uint).wrapping_sub(SDLK_F1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint).wrapping_div(4
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_uint)
                                                                        as
                                                                        size_t;
                                                                if d < ndeck {
                                                                    let mut func:
                                                                            libc::c_int =
                                                                        0;
                                                                    let mut de:
                                                                            *mut deck =
                                                                        0 as
                                                                            *mut deck;
                                                                    let mut pl:
                                                                            *mut player =
                                                                        0 as
                                                                            *mut player;
                                                                    let mut re:
                                                                            *mut record =
                                                                        0 as
                                                                            *mut record;
                                                                    let mut tc:
                                                                            *mut timecoder =
                                                                        0 as
                                                                            *mut timecoder;
                                                                    func =
                                                                        (key
                                                                             as
                                                                             libc::c_uint).wrapping_sub(SDLK_F1
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint).wrapping_rem(4
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           libc::c_uint)
                                                                            as
                                                                            libc::c_int;
                                                                    de =
                                                                        &mut *deck.as_mut_ptr().offset(d
                                                                                                           as
                                                                                                           isize)
                                                                            as
                                                                            *mut deck;
                                                                    pl =
                                                                        &mut (*de).player;
                                                                    tc =
                                                                        &mut (*de).timecoder;
                                                                    /* Some undocumented and 'special' functions exist
             * here for the developer */
                                                                    if mod_0
                                                                           as
                                                                           libc::c_uint
                                                                           &
                                                                           (KMOD_LSHIFT
                                                                                as
                                                                                libc::c_int
                                                                                |
                                                                                KMOD_RSHIFT
                                                                                    as
                                                                                    libc::c_int)
                                                                               as
                                                                               libc::c_uint
                                                                           !=
                                                                           0
                                                                           &&
                                                                           mod_0
                                                                               as
                                                                               libc::c_uint
                                                                               &
                                                                               (KMOD_LCTRL
                                                                                    as
                                                                                    libc::c_int
                                                                                    |
                                                                                    KMOD_RCTRL
                                                                                        as
                                                                                        libc::c_int)
                                                                                   as
                                                                                   libc::c_uint
                                                                               ==
                                                                               0
                                                                       {
                                                                        if (func
                                                                                as
                                                                                libc::c_ulong)
                                                                               <
                                                                               ndeck
                                                                           {
                                                                            deck_clone(de,
                                                                                       &mut *deck.as_mut_ptr().offset(func
                                                                                                                          as
                                                                                                                          isize));
                                                                        }
                                                                    } else {
                                                                        match func
                                                                            {
                                                                            0
                                                                            =>
                                                                            {
                                                                                re
                                                                                    =
                                                                                    selector_current(sel);
                                                                                if !re.is_null()
                                                                                   {
                                                                                    deck_load(de,
                                                                                              re);
                                                                                }
                                                                            }
                                                                            1
                                                                            =>
                                                                            {
                                                                                deck_recue(de);
                                                                            }
                                                                            2
                                                                            =>
                                                                            {
                                                                                if mod_0
                                                                                       as
                                                                                       libc::c_uint
                                                                                       &
                                                                                       (KMOD_LCTRL
                                                                                            as
                                                                                            libc::c_int
                                                                                            |
                                                                                            KMOD_RCTRL
                                                                                                as
                                                                                                libc::c_int)
                                                                                           as
                                                                                           libc::c_uint
                                                                                       !=
                                                                                       0
                                                                                   {
                                                                                    if mod_0
                                                                                           as
                                                                                           libc::c_uint
                                                                                           &
                                                                                           (KMOD_LSHIFT
                                                                                                as
                                                                                                libc::c_int
                                                                                                |
                                                                                                KMOD_RSHIFT
                                                                                                    as
                                                                                                    libc::c_int)
                                                                                               as
                                                                                               libc::c_uint
                                                                                           !=
                                                                                           0
                                                                                       {
                                                                                        player_set_internal_playback(pl);
                                                                                    } else {
                                                                                        timecoder_cycle_definition(tc);
                                                                                    }
                                                                                } else {
                                                                                    player_toggle_timecode_control(pl);
                                                                                }
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int != 0;
}
/*
 * Action on size change event on the main window
 */
unsafe extern "C" fn set_size(mut w: libc::c_int, mut h: libc::c_int,
                              mut r: *mut rect) -> *mut SDL_Surface {
    let mut surface: *mut SDL_Surface = 0 as *mut SDL_Surface;
    surface = SDL_SetVideoMode(w, h, 32 as libc::c_int, video_flags);
    if surface.is_null() {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                SDL_GetError());
        return 0 as *mut SDL_Surface
    }
    *r =
        shrink(rect(0 as libc::c_int as pix_t, 0 as libc::c_int as pix_t,
                    w as pix_t, h as pix_t, scale), 12 as libc::c_int);
    fprintf(stderr,
            b"New interface size is %dx%d.\n\x00" as *const u8 as
                *const libc::c_char, w, h);
    return surface;
}
unsafe extern "C" fn push_event(mut t: libc::c_int) {
    let mut e: SDL_Event = SDL_Event{type_0: 0,};
    if SDL_PeepEvents(&mut e, 1 as libc::c_int, SDL_PEEKEVENT,
                      ((1 as libc::c_int) << t) as Uint32) == 0 {
        e.type_0 = t as Uint8;
        if SDL_PushEvent(&mut e) == -(1 as libc::c_int) { abort(); }
    };
}
/*
 * Timer which posts a screen redraw event
 */
unsafe extern "C" fn ticker(mut interval: Uint32, mut p: *mut libc::c_void)
 -> Uint32 {
    push_event(SDL_USEREVENT as libc::c_int);
    return interval;
}
/*
 * Callback to tell the interface that status has changed
 */
unsafe extern "C" fn defer_status_redraw(mut o: *mut observer,
                                         mut x: *mut libc::c_void) {
    push_event(SDL_USEREVENT as libc::c_int + 2 as libc::c_int);
}
unsafe extern "C" fn defer_selector_redraw(mut o: *mut observer,
                                           mut x: *mut libc::c_void) {
    push_event(SDL_USEREVENT as libc::c_int + 3 as libc::c_int);
}
/*
 * The SDL interface thread
 */
unsafe extern "C" fn interface_main() -> libc::c_int {
    let mut library_update: bool = false;
    let mut decks_update: bool = false;
    let mut status_update: bool = false;
    let mut event: SDL_Event = SDL_Event{type_0: 0,};
    let mut timer: SDL_TimerID = 0 as *mut _SDL_TimerID;
    let mut surface: *mut SDL_Surface = 0 as *mut SDL_Surface;
    let mut rworkspace: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rplayers: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rlibrary: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rstatus: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    let mut rtmp: rect = rect{x: 0, y: 0, w: 0, h: 0, scale: 0.,};
    surface = set_size(width, height, &mut rworkspace);
    if surface.is_null() { return -(1 as libc::c_int) }
    decks_update = 1 as libc::c_int != 0;
    status_update = 1 as libc::c_int != 0;
    library_update = 1 as libc::c_int != 0;
    /* The final action is to add the timer which triggers refresh */
    timer =
        SDL_AddTimer(10 as libc::c_int as Uint32,
                     Some(ticker as
                              unsafe extern "C" fn(_: Uint32,
                                                   _: *mut libc::c_void)
                                  -> Uint32),
                     0 as *mut libc::c_void); /* main loop */
    rig_lock(); /* switch(event.type) */
    loop  {
        rig_unlock();
        if SDL_WaitEvent(&mut event) < 0 as libc::c_int { break ; }
        rig_lock();
        match event.type_0 as libc::c_int {
            12 => {
                /* user request to quit application; eg. window close */
                if rig_quit() == -(1 as libc::c_int) {
                    return -(1 as libc::c_int)
                }
            }
            16 => {
                surface =
                    set_size(event.resize.w, event.resize.h, &mut rworkspace);
                if surface.is_null() { return -(1 as libc::c_int) }
                library_update = 1 as libc::c_int != 0;
                decks_update = 1 as libc::c_int != 0;
                status_update = 1 as libc::c_int != 0
            }
            24 => { decks_update = 1 as libc::c_int != 0 }
            25 => { break ; }
            26 => { status_update = 1 as libc::c_int != 0 }
            27 => { library_update = 1 as libc::c_int != 0 }
            2 => {
                if handle_key(event.key.keysym.sym, event.key.keysym.mod_0) {
                    let mut r: *mut record = 0 as *mut record;
                    r = selector_current(&mut selector);
                    if !r.is_null() {
                        status_set(0 as libc::c_int, (*r).pathname);
                    } else {
                        status_set(0 as libc::c_int,
                                   b"No search results found\x00" as *const u8
                                       as *const libc::c_char);
                    }
                }
            }
            _ => { }
        }
        /* Split the display into the various areas. If an area is too
         * small, abandon any actions to happen in that area. */
        split(rworkspace,
              from_bottom(12 as libc::c_int as libc::c_uint,
                          8 as libc::c_int as libc::c_uint), &mut rtmp,
              &mut rstatus);
        if (rtmp.h as libc::c_int) < 128 as libc::c_int ||
               (rtmp.w as libc::c_int) < 0 as libc::c_int {
            rtmp = rworkspace;
            status_update = 0 as libc::c_int != 0
        }
        split(rtmp,
              from_top(213 as libc::c_int as libc::c_uint,
                       8 as libc::c_int as libc::c_uint), &mut rplayers,
              &mut rlibrary);
        if (rlibrary.h as libc::c_int) < 64 as libc::c_int ||
               (rlibrary.w as libc::c_int) < 64 as libc::c_int {
            rplayers = rtmp;
            library_update = 0 as libc::c_int != 0
        }
        if (rplayers.h as libc::c_int) < 0 as libc::c_int ||
               (rplayers.w as libc::c_int) < 0 as libc::c_int {
            decks_update = 0 as libc::c_int != 0
        }
        if !library_update && !decks_update && !status_update { continue ; }
        if (*surface).offset != 0 ||
               (*surface).flags &
                   (0x1 as libc::c_int | 0x4 as libc::c_int |
                        0x4000 as libc::c_int) as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
            SDL_LockSurface(surface);
        }
        if library_update {
            draw_library(surface, &mut rlibrary, &mut selector);
        }
        if status_update { draw_status(surface, &mut rstatus); }
        if decks_update {
            draw_decks(surface, &mut rplayers, deck.as_mut_ptr(), ndeck,
                       meter_scale);
        }
        if (*surface).offset != 0 ||
               (*surface).flags &
                   (0x1 as libc::c_int | 0x4 as libc::c_int |
                        0x4000 as libc::c_int) as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
            SDL_UnlockSurface(surface);
        }
        if library_update {
            SDL_UpdateRect(surface, rlibrary.x as Sint32,
                           rlibrary.y as Sint32, rlibrary.w as Uint32,
                           rlibrary.h as Uint32);
            library_update = 0 as libc::c_int != 0
        }
        if status_update {
            SDL_UpdateRect(surface, rstatus.x as Sint32, rstatus.y as Sint32,
                           rstatus.w as Uint32, rstatus.h as Uint32);
            status_update = 0 as libc::c_int != 0
        }
        if decks_update {
            SDL_UpdateRect(surface, rplayers.x as Sint32,
                           rplayers.y as Sint32, rplayers.w as Uint32,
                           rplayers.h as Uint32);
            decks_update = 0 as libc::c_int != 0
        }
    }
    /* internal request to finish this thread */
    rig_unlock();
    SDL_RemoveTimer(timer);
    return 0 as libc::c_int;
}
unsafe extern "C" fn launch(mut p: *mut libc::c_void) -> *mut libc::c_void {
    interface_main();
    return 0 as *mut libc::c_void;
}
/*
 * Parse and action the given geometry string
 *
 * Geometry string includes size, position and scale. The format is
 * "[<n>x<n>][+<n>+<n>][@<f>]". Some examples:
 *
 *   960x720
 *   +10+10
 *   960x720+10+10
 *   @1.6
 *   1920x1200@1.6
 *
 * Return: -1 if string could not be actioned, otherwise 0
 */
unsafe extern "C" fn parse_geometry(mut s: *const libc::c_char)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    /* The %n in format strings is not a token, see scanf(3) man page */
    n =
        sscanf(s, b"%[0-9]x%d%n\x00" as *const u8 as *const libc::c_char,
               buf.as_mut_ptr(), &mut height as *mut libc::c_int,
               &mut len as *mut libc::c_int);
    match n {
        -1 => { return 0 as libc::c_int }
        0 => { }
        2 => {
            /* we used a format to prevent parsing the '+' in the next block */
            width = atoi(buf.as_mut_ptr());
            s = s.offset(len as isize)
        }
        _ => { return -(1 as libc::c_int) }
    }
    n =
        sscanf(s, b"+%d+%d%n\x00" as *const u8 as *const libc::c_char,
               &mut x as *mut libc::c_int, &mut y as *mut libc::c_int,
               &mut len as *mut libc::c_int);
    match n {
        -1 => { return 0 as libc::c_int }
        0 => { }
        2 => {
            /* Not a desirable way to get geometry information to
         * SDL, but it seems to be the only way */
            sprintf(buf.as_mut_ptr(),
                    b"SDL_VIDEO_WINDOW_POS=%d,%d\x00" as *const u8 as
                        *const libc::c_char, x, y);
            if putenv(buf.as_mut_ptr()) != 0 as libc::c_int {
                return -(1 as libc::c_int)
            }
            s = s.offset(len as isize)
        }
        _ => { return -(1 as libc::c_int) }
    }
    n =
        sscanf(s, b"/%f%n\x00" as *const u8 as *const libc::c_char,
               &mut scale as *mut libc::c_float,
               &mut len as *mut libc::c_int);
    match n {
        -1 => { return 0 as libc::c_int }
        0 => { }
        1 => {
            if scale as libc::c_double <= 0.0f64 {
                return -(1 as libc::c_int)
            }
            s = s.offset(len as isize)
        }
        _ => { return -(1 as libc::c_int) }
    }
    if *s as libc::c_int != '\u{0}' as i32 { return -(1 as libc::c_int) }
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
 * Start the SDL interface
 *
 * FIXME: There are multiple points where resources are leaked on
 * error
 */
#[no_mangle]
pub unsafe extern "C" fn interface_start(mut lib: *mut library,
                                         mut geo: *const libc::c_char,
                                         mut decor: bool) -> libc::c_int {
    let mut n: size_t = 0;
    if parse_geometry(geo) == -(1 as libc::c_int) {
        fprintf(stderr,
                b"Window geometry (\'%s\') is not valid.\n\x00" as *const u8
                    as *const libc::c_char, geo);
        return -(1 as libc::c_int)
    }
    if !decor { video_flags |= 0x20 as libc::c_int as libc::c_uint }
    n = 0 as libc::c_int as size_t;
    while n < ndeck {
        if timecoder_monitor_init(&mut (*deck.as_mut_ptr().offset(n as
                                                                      isize)).timecoder,
                                  zoom(32 as libc::c_int * 2 as libc::c_int -
                                           6 as libc::c_int)) ==
               -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        }
        n = n.wrapping_add(1)
    }
    if init_spinner(zoom(32 as libc::c_int * 2 as libc::c_int -
                             6 as libc::c_int)) == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    selector_init(&mut selector, lib);
    watch(&mut on_status, &mut status_changed,
          Some(defer_status_redraw as
                   unsafe extern "C" fn(_: *mut observer,
                                        _: *mut libc::c_void) -> ()));
    watch(&mut on_selector, &mut selector.changed,
          Some(defer_selector_redraw as
                   unsafe extern "C" fn(_: *mut observer,
                                        _: *mut libc::c_void) -> ()));
    status_set(0 as libc::c_int, banner);
    fprintf(stderr,
            b"Initialising SDL...\n\x00" as *const u8 as *const libc::c_char);
    if SDL_Init((0x20 as libc::c_int | 0x1 as libc::c_int) as Uint32) ==
           -(1 as libc::c_int) {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                SDL_GetError());
        return -(1 as libc::c_int)
    }
    SDL_WM_SetCaption(banner, 0 as *const libc::c_char);
    SDL_EnableKeyRepeat(500 as libc::c_int, 30 as libc::c_int);
    /* Initialise the fonts */
    if TTF_Init() == -(1 as libc::c_int) {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                SDL_GetError());
        return -(1 as libc::c_int)
    }
    if load_fonts() == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    utf =
        iconv_open(b"UTF8\x00" as *const u8 as *const libc::c_char,
                   b"\x00" as *const u8 as *const libc::c_char);
    if utf == -(1 as libc::c_int) as iconv_t {
        perror(b"iconv_open\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    fprintf(stderr,
            b"Launching interface thread...\n\x00" as *const u8 as
                *const libc::c_char);
    if pthread_create(&mut ph, 0 as *const pthread_attr_t,
                      Some(launch as
                               unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> *mut libc::c_void),
                      0 as *mut libc::c_void) != 0 {
        perror(b"pthread_create\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * Synchronise with the SDL interface and exit
 */
#[no_mangle]
pub unsafe extern "C" fn interface_stop() {
    let mut n: size_t = 0;
    push_event(SDL_USEREVENT as libc::c_int + 1 as libc::c_int);
    if pthread_join(ph, 0 as *mut *mut libc::c_void) != 0 as libc::c_int {
        abort();
    }
    n = 0 as libc::c_int as size_t;
    while n < ndeck {
        timecoder_monitor_clear(&mut (*deck.as_mut_ptr().offset(n as
                                                                    isize)).timecoder);
        n = n.wrapping_add(1)
    }
    clear_spinner();
    ignore(&mut on_status);
    ignore(&mut on_selector);
    selector_clear(&mut selector);
    clear_fonts();
    if iconv_close(utf) == -(1 as libc::c_int) { abort(); }
    TTF_Quit();
    SDL_Quit();
}
