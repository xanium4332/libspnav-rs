//! All the functions in this crate have identical functionality to the
//! underlying `libspnav` library, with the key change being the use of a Rust
//! enum to represent the events returned by `wait_event()` safely.

use std::os::raw::{c_double, c_uint};
use std::os::unix::io::RawFd;
use libspnav_sys as sys;

mod error;
pub use error::{Error, Result};

#[cfg(feature = "serde-serialize")]
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct MotionEvent {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rx: i32,
    pub ry: i32,
    pub rz: i32,
    pub period: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct ButtonEvent {
    pub press: i32,
    pub bnum: i32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum Event {
    Motion(MotionEvent),
    Button(ButtonEvent),
}

// Does not implement From so as to avoid appearing in public API
impl Event {
    fn from_sys(item: sys::spnav_event) -> Self {
        unsafe {
            match item.type_  as u32 {
                sys::SPNAV_EVENT_MOTION => {
                    Event::Motion(MotionEvent {
                        x:  item.motion.x  as i32,
                        y:  item.motion.y  as i32,
                        z:  item.motion.z  as i32,
                        rx: item.motion.rx as i32,
                        ry: item.motion.ry as i32,
                        rz: item.motion.rz as i32,
                        period: item.motion.period  as u32,
                    })
                },
                sys::SPNAV_EVENT_BUTTON => {
                    Event::Button(ButtonEvent {
                        press: item.button.press as i32,
                        bnum:  item.button.bnum as i32,
                    })
                }
                _ => panic!("Unknown event type"),
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum EventType {
    Motion,
    Button,
    Any,
}

impl EventType {
    fn from_sys(event: c_uint) -> Self {
        match event {
            sys::SPNAV_EVENT_MOTION => EventType::Motion,
            sys::SPNAV_EVENT_BUTTON => EventType::Button,
            _ => panic!("Unknown event type"),
        } 
    }

    fn to_sys(&self) -> c_uint {
        match self {
            EventType::Motion => sys::SPNAV_EVENT_MOTION,
            EventType::Button => sys::SPNAV_EVENT_BUTTON,
            EventType::Any    => sys::SPNAV_EVENT_ANY,
        }
    }
}

pub fn open() -> Result<()> {
    let ret = unsafe { sys::spnav_open() };
    if ret != 0 {
        Err(Error::GenericError(ret))
    } else {
        Ok(())
    }
}

pub fn close() -> Result<()> {
    let ret = unsafe { sys::spnav_close() };
    if ret != 0 {
        Err(Error::GenericError(ret))
    } else {
        Ok(())
    }
}

pub fn fd() -> RawFd {
    unsafe { sys::spnav_fd() }
}

pub fn sensitivity(sens: f64) -> Result<()> {
    let ret = unsafe { sys::spnav_sensitivity(sens as c_double) };
    if ret != 0 {
        Err(Error::GenericError(ret))
    } else {
        Ok(())
    }
}

pub fn wait_event() -> Result<Event> {
    let mut event: sys::spnav_event = Default::default();

    let ret = unsafe { sys::spnav_wait_event(&mut event) };
    if ret == 0 {
        Err(Error::GenericError(ret))
    } else {
        Ok(Event::from_sys(event))
    }
}

pub fn poll_event() -> Option<EventType> {
    let mut event: sys::spnav_event = Default::default();

    let ret = unsafe { sys::spnav_poll_event(&mut event) };
    if ret == 0 {
        None
    } else {
        Some(EventType::from_sys(ret as c_uint))
    }
}

pub fn remove_events(type_: EventType) -> u32 {
    unsafe { sys::spnav_remove_events(type_.to_sys() as i32) as u32 }
}