
#![allow(non_snake_case)]
#![feature(asm)]

pub mod svc
{
    pub fn sleepThread(nano: i64)
    {
        unsafe
        {
            asm!("svc 0x0B" : : "{x0}"(nano) : "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18" : "volatile");
        }
    }
}

fn main()
{
    svc::sleepThread(1000000000);
}