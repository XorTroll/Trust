
#![allow(non_snake_case)]
#![feature(asm)]

mod svc;
mod arm;
mod hipc;

unsafe fn encodeService(name: String) -> u64
{
    let mut ename: u64 = 0;
    let nameptr = name.as_ptr();
    for i in 0..8
    {
        if *nameptr.offset(i) as char == '\0'
        {
            break;
        }
        ename |= (*nameptr.offset(i) as u64) << (8 * i);
    }
    ename
}

fn main()
{
    // This SVC can be used on emulators as console output, as in actual consoles we wouldn't have a better way to print stuff
    unsafe
    {
        let (mut rc, smh) = svc::connectToNamedPort(String::from("sm:"));
        let mut sm = hipc::Object::fromHandle(smh);
        {
            let mut ctx = hipc::SessionContext::forObject(&sm);
            ctx.inProcessId = true;
            ctx.inRaws.push( hipc::InRawValue { value: &(0 as u64) as *const u64 as *mut u8, rawType: hipc::RawType::U64 } );
            ctx.inRaws.push( hipc::InRawValue { value: &(0 as u64) as *const u64 as *mut u8, rawType: hipc::RawType::U64 } );
            ctx.inRaws.push( hipc::InRawValue { value: &(0 as u64) as *const u64 as *mut u8, rawType: hipc::RawType::U64 } );
            let src = ctx.processRequest(110);
            svc::outputDebugString(format!("Result: {}", src));
        }
        {
            let mut ctx = hipc::SessionContext::forObject(&sm);
            let srv = encodeService(String::from("psm"));
            ctx.inRaws.push( hipc::InRawValue { value: &srv as *const u64 as *mut u8, rawType: hipc::RawType::U64 } );
            ctx.inRaws.push( hipc::InRawValue { value: &(0 as u64) as *const u64 as *mut u8, rawType: hipc::RawType::U64 } );
            ctx.inRaws.push( hipc::InRawValue { value: &(0 as u64) as *const u64 as *mut u8, rawType: hipc::RawType::U64 } );
            let src = ctx.processRequest(1);
            svc::outputDebugString(format!("Result: {}, Handle: {}", src, ctx.outHandles.len()));
        }
    }
    loop
    {
        // I finish with this infinite loop as the custom std impl for Rust is outdated and keeps logging invalid handle shit on emulators
    }
}