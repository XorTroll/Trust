
// Somehow I managed to get these to work barely-barely knowing inline asm

pub fn sleepThread(nano: i64) -> u32
{
    let rc : u32;
    unsafe
    {
        asm!("svc 0xb" : "={x0}"(rc) : "{x0}"(nano) : "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18" : "volatile");
    }
    rc
}

pub fn outputDebugString(dstr: String) -> u32
{
    let rc : u32;
    unsafe
    {
        asm!("svc 0x27" : "={x0}"(rc) : "{x0}"(dstr.as_ptr() as *const u8), "{x1}"(dstr.len() + 1) : "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18" : "volatile");
    }
    rc
}

pub fn connectToNamedPort(mut name: String) -> (u32, u32)
{
    name.push('\0');
    let rc : u32;
    let handle : u32;
    unsafe
    {
        asm!("svc 0x1f" : "={x0}"(rc), "={x1}"(handle) : "{x1}"(name.as_ptr() as *const char) : "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18" : "volatile");
    }
    (rc, handle)
}

pub unsafe fn sendSyncRequest(handle: u32) -> u32
{
    let rc : u32;
    asm!("svc 0x21" : "={x0}"(rc) : "{x0}"(handle) : "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18" : "volatile");
    rc
}