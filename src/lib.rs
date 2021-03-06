#![no_std]
#![feature(alloc)]
#![feature(asm)]

#[macro_use]
extern crate alloc;

use alloc::String;

pub use self::file::EcFile;
pub use self::flash::EcFlash;

mod file;
mod flash;
mod io;

pub trait Ec {
    fn size(&mut self) -> usize;
    fn project(&mut self) -> String;
    fn version(&mut self) -> String;
}
