pub fn getThreadLocalStorage() -> *mut u8
{
    let tlsptr : *mut u8;
    unsafe
    {
        asm!("mrs $0, tpidrro_el0" : "=r" (tlsptr));
    }
    tlsptr
}