
use svc;
use arm;

pub enum ObjectType
{
    Invalid,
    Normal,
    Domain,
    DomainSubservice,
    HBABIOverride,
}

pub enum RawType
{
    U8,
    U16,
    U32,
    U64,
    U128,
    RawData,
}

pub struct Object
{
    pub handle: u32,
    pub objectId: u32,
    pub objectType: ObjectType,
}

impl Object
{
    pub fn fromHandle(handle: u32) -> Object
    {
        Object
        {
            handle: handle,
            objectId: std::u32::MAX,
            objectType: ObjectType::Normal
        }
    }
}

pub struct Buffer
{
    pub ptr: *const u8,
    pub size: u64,
    pub bufType: u32,
}

pub struct StaticBuffer
{
    pub ptr: *const u8,
    pub size: u64,
    pub index: u32,
}

pub struct InRawValue
{
    pub value: *mut u8,
    pub rawType: RawType,
    pub rawOffset: usize,
}

pub struct OutRawValue
{
    pub value: *mut u8,
    pub rawType: RawType,
    pub rawOffset: usize,
}

pub struct SessionContext
{
    pub inProcessId: bool,
    pub inRaw: *mut u8,
    pub inRaws: Vec<InRawValue>,
    pub inRawSize: u64,
    pub inCopyHandles: Vec<u32>,
    pub inMoveHandles: Vec<u32>,
    pub inObjectIds: Vec<u32>,
    pub inBuffers: Vec<Buffer>,
    pub inStaticBuffers: Vec<StaticBuffer>,
    pub sent: bool,
    pub outProcessId: u64,
    pub outHandles: Vec<u32>,
    pub outObjectIds: Vec<u32>,
    pub outRaw: *const u8,
    pub outRaws: Vec<OutRawValue>,
    pub outRawSize: u64,
    pub outBuffers: Vec<Buffer>,
    pub outStaticBuffers: Vec<StaticBuffer>,
    pub sessionHandle: u32,
    pub sessionObjectId: u32,
}

impl SessionContext
{
    pub fn forObject(obj: &Object) -> SessionContext
    {
        SessionContext
        {
            inProcessId: false,
            inRaw: std::ptr::null_mut(),
            inRaws: Vec::new(),
            inRawSize: 0,
            inCopyHandles: Vec::new(),
            inMoveHandles: Vec::new(),
            inObjectIds: Vec::new(),
            inBuffers: Vec::new(),
            inStaticBuffers: Vec::new(),
            sent: false,
            outProcessId: 0,
            outHandles: Vec::new(),
            outObjectIds: Vec::new(),
            outRaw: std::ptr::null(),
            outRaws: Vec::new(),
            outRawSize: 0,
            outBuffers: Vec::new(),
            outStaticBuffers: Vec::new(),
            sessionHandle: obj.handle,
            sessionObjectId: obj.objectId,
        }
    }

    pub unsafe fn processRequest(&mut self, command: u32) -> u32
    {
        let tls = arm::getThreadLocalStorage() as *mut u32;

        self.inRawSize += (std::mem::align_of::<u64>() as u64) - 1;
        self.inRawSize -= self.inRawSize % std::mem::align_of::<u64>() as u64;
        let offmagic = self.inRawSize;
        self.inRawSize += std::mem::size_of::<u64>() as u64;

        self.inRawSize += (std::mem::align_of::<u64>() as u64) - 1;
        self.inRawSize -= self.inRawSize % std::mem::align_of::<u64>() as u64;
        let offcmdid = self.inRawSize;
        self.inRawSize += std::mem::size_of::<u64>() as u64;

        for mut inraw in &mut self.inRaws
        {
            let szof = match inraw.rawType
            {
                RawType::U8 => std::mem::size_of::<u8>(),
                RawType::U16 => std::mem::size_of::<u16>(),
                RawType::U32 => std::mem::size_of::<u32>(),
                RawType::U64 => std::mem::size_of::<u64>(),
                RawType::U128 => std::mem::size_of::<u128>(),
                RawType::RawData => std::mem::size_of_val(&inraw.value),
            };
            let agof = match inraw.rawType
            {
                RawType::U8 => std::mem::align_of::<u8>(),
                RawType::U16 => std::mem::align_of::<u16>(),
                RawType::U32 => std::mem::align_of::<u32>(),
                RawType::U64 => std::mem::align_of::<u64>(),
                RawType::U128 => std::mem::align_of::<u128>(),
                RawType::RawData => std::mem::align_of_val(&inraw.value),
            };
            self.inRawSize += (agof as u64) - 1;
            self.inRawSize -= self.inRawSize % agof as u64;
            inraw.rawOffset = self.inRawSize as usize;
            self.inRawSize += szof as u64;
        }

        let mut tlsi: isize = 0;
        *tls.offset(tlsi) = 4 | (self.inStaticBuffers.len() << 16) as u32 | (self.inBuffers.len() << 20) as u32 | (self.outBuffers.len() << 24 | 0 << 28) as u32;
        tlsi += 1;
        let tlsifsz: isize = tlsi;
        if self.outStaticBuffers.len() > 0
        {
            *tls.offset(tlsi) = ((self.outStaticBuffers.len() + 2) << 10) as u32;
        }
        else
        {
            *tls.offset(tlsi) = 0;
        }
        if self.inProcessId || !self.inCopyHandles.is_empty() || !self.inMoveHandles.is_empty()
        {
            *tls.offset(tlsi) |= 0x80000000;
            tlsi += 1;
            *tls.offset(tlsi) = self.inProcessId as u8 as u32 | (self.inCopyHandles.len() << 1) as u32 | (self.inMoveHandles.len() << 5) as u32;
            tlsi += 1;
            if self.inProcessId 
            {
                tlsi += 2;
            }
            for handle in &self.inCopyHandles
            {
                *tls.offset(tlsi) = *handle;
                tlsi += 1;
            }
            for handle in &self.inMoveHandles
            {
                *tls.offset(tlsi) = *handle;
                tlsi += 1;
            }
        }
        else
        {
            tlsi += 1;
        }
        for stbuf in &self.inStaticBuffers
        {
            let uptr = stbuf.ptr as usize;
            *tls.offset(tlsi) = (stbuf.index | (stbuf.size << 16) as u32 | (((uptr >> 32) & 15) << 12) as u32 | (((uptr >> 36) & 15) << 6) as u32) as u32;
            tlsi += 1;
            *tls.offset(tlsi) = uptr as u32;
            tlsi += 1;
        }
        for buf in &self.inBuffers
        {
            *tls.offset(tlsi) = buf.size as u32;
            tlsi += 1;
            let uptr = buf.ptr as usize;
            *tls.offset(tlsi) = uptr as u32;
            tlsi += 1;
            *tls.offset(tlsi) = (buf.bufType | (((uptr >> 32) & 15) << 28) as u32 | ((uptr >> 36) << 2) as u32) as u32;
            tlsi += 1;
        }
        for buf in &self.outBuffers
        {
            *tls.offset(tlsi) = buf.size as u32;
            tlsi += 1;
            let uptr = buf.ptr as usize;
            *tls.offset(tlsi) = uptr as u32;
            tlsi += 1;
            *tls.offset(tlsi) = (buf.bufType | (((uptr >> 32) & 15) << 28) as u32 | (((uptr >> 36) << 2)) as u32) as u32;
            tlsi += 1;
        }
        let pad = ((16 - ((tls as u32) & 15)) & 15) / 4;
        let raw = tls.offset(tlsi + pad as isize) as *mut u8;
        let mut rawsz = (self.inRawSize / 4) + 4;
        tlsi += rawsz as isize;
        {
            let tls16 = (&mut *tls.offset(tlsi) as *mut u32) as *mut u16;
            let mut tls16i: isize = 0;
            for stbuf in &self.outStaticBuffers
            {
                let mut val: u16;
                if stbuf.size > 0xffff
                {
                    val = 0;
                }
                else
                {
                    val = stbuf.size as u16;
                }
                *tls16.offset(tls16i) = val;
                tls16i += 1;
            }
        }
        let u16s = ((2 * self.outStaticBuffers.len()) + 3) / 4;
        tlsi += u16s as isize;
        rawsz += u16s as u64;
        *tls.offset(tlsifsz) |= rawsz as u32;
        for stbuf in &self.outStaticBuffers
        {
            let uptr = stbuf.ptr as usize;
            *tls.offset(tlsi) = uptr as u32;
            tlsi += 1;
            *tls.offset(tlsi) = (uptr >> 32) as u32 | (stbuf.size << 16) as u32;
            tlsi += 1;
        }
        self.inRaw = raw as *mut u8;
        *(raw as *mut u64).offset(offmagic as isize) = 0x49434653;
        *(raw as *mut u64).offset(offcmdid as isize) = command as u64;

        for inraw in &self.inRaws
        {
            match inraw.rawType
            {
                RawType::U8 =>
                {
                    *(raw as *mut u8).offset(inraw.rawOffset as isize) = *(inraw.value as *mut u8);
                }
                RawType::U16 =>
                {
                    *(raw as *mut u16).offset(inraw.rawOffset as isize) = *(inraw.value as *mut u16);
                }
                RawType::U32 =>
                {
                    *(raw as *mut u32).offset(inraw.rawOffset as isize) = *(inraw.value as *mut u32);
                }
                RawType::U64 =>
                {
                    svc::outputDebugString(format!("InRaw u64: {}", *(inraw.value as *mut u64)));
                    *(raw as *mut u64).offset(inraw.rawOffset as isize) = *(inraw.value as *mut u64);
                }
                RawType::U128 =>
                {
                    *(raw as *mut u128).offset(inraw.rawOffset as isize) = *(inraw.value as *mut u128);
                }
                RawType::RawData =>
                {
                    // (raw as *mut u8).offset(inraw.rawOffset as isize) = inraw.value as *mut u8;
                }
            }
        }

        let mut rc = svc::sendSyncRequest(self.sessionHandle);
        if rc == 0
        {
            tlsi = 0;
            self.outRawSize += (std::mem::align_of::<u64>() as u64) - 1;
            self.outRawSize -= self.outRawSize % std::mem::align_of::<u64>() as u64;
            self.outRawSize += std::mem::size_of::<u64>() as u64;

            self.outRawSize += (std::mem::align_of::<u64>() as u64) - 1;
            self.outRawSize -= self.outRawSize % std::mem::align_of::<u64>() as u64;
            let offrc = self.outRawSize;
            self.outRawSize += std::mem::size_of::<u64>() as u64;

            let ctrl0 = *tls.offset(tlsi);
            tlsi += 1;
            let ctrl1 = *tls.offset(tlsi);
            svc::outputDebugString(format!("ctrl1: {}", ctrl1));
            tlsi += 1;
            if (ctrl1 & 0x80000000) > 0
            {
                let ctrl2 = *tls.offset(tlsi);
                tlsi += 1;
                if (ctrl2 & 1) != 0
                {
                    let mut rawpid = *tls.offset(tlsi) as u64;
                    tlsi += 1;
                    rawpid |= (*tls.offset(tlsi) as u64) << 32;
                    tlsi += 1;
                    self.outProcessId = rawpid;
                }
                let ohcopy = (ctrl2 >> 1) & 15;
                let ohmove = (ctrl2 >> 5) & 15;
                let mut oh = ohcopy + ohmove;
                svc::outputDebugString(format!("oh: {}", oh));
                let otlsi = tlsi + oh as isize;
                if oh > 8
                {
                    oh = 8;
                }
                if oh > 0
                {
                    self.outHandles.reserve(oh as usize);
                    for i in 0..oh
                    {
                        let ohandle = *tls.offset(tlsi + i as isize);
                        self.outHandles.push(ohandle);
                    }
                }
                tlsi = otlsi;
            }
            let stnum = (ctrl0 >> 16) & 15;
            tlsi += (stnum * 2) as isize;
            let bsnum = (ctrl0 >> 20) & 15;
            let brnum = (ctrl0 >> 24) & 15;
            let benum = (ctrl0 >> 28) & 15;
            let bufnum = bsnum + brnum + benum;
            let oraw = (((tls.offset(tlsi + (bufnum * 3) as isize) as u32) + 15) &! 15) as *const u8;
            self.outRaw = oraw;
            rc = *(oraw as *const u64).offset(offrc as isize) as u64 as u32;
        }
        rc
    }
}