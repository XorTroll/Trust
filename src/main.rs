
#![allow(non_snake_case)]
#![feature(asm)]

mod svc;

fn main()
{
    // This SVC can be used on emulators as console output, as in actual consoles we wouldn't have a better way to print stuff
    svc::outputDebugString(String::from("Test output!"));
    loop
    {
        // I finish with this infinite loop as the custom std impl for Rust is outdated and keeps logging invalid handle shit on emulators
    }
}