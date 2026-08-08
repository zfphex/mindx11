use crate::types::{BOOL, GUID, HRESULT, IUnknown, IUnknownVtbl};
use core::ffi::c_void;

pub const IID_ID3D11INFOQUEUE: GUID = GUID::from_u128(0x6543dbb6_1b48_42f5_ab82_e97ec74326f6);

pub const D3D11_MESSAGE_ID_UNKNOWN: u32 = 0;
pub const D3D11_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u64 = 1024;

pub type D3D11_MESSAGE_ID = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_MESSAGE_CATEGORY {
    APPLICATION_DEFINED = 0,
    MISCELLANEOUS = 1,
    INITIALIZATION = 2,
    CLEANUP = 3,
    COMPILATION = 4,
    STATE_CREATION = 5,
    STATE_SETTING = 6,
    STATE_GETTING = 7,
    RESOURCE_MANIPULATION = 8,
    EXECUTION = 9,
    SHADER = 10,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_MESSAGE_SEVERITY {
    CORRUPTION = 0,
    ERROR = 1,
    WARNING = 2,
    INFO = 3,
    MESSAGE = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D3D11_MESSAGE {
    pub Category: D3D11_MESSAGE_CATEGORY,
    pub Severity: D3D11_MESSAGE_SEVERITY,
    pub ID: D3D11_MESSAGE_ID,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D3D11_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D11_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D11_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D11_MESSAGE_ID,
}

impl Default for D3D11_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        Self {
            NumCategories: 0,
            pCategoryList: core::ptr::null_mut(),
            NumSeverities: 0,
            pSeverityList: core::ptr::null_mut(),
            NumIDs: 0,
            pIDList: core::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D3D11_INFO_QUEUE_FILTER {
    pub AllowList: D3D11_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D11_INFO_QUEUE_FILTER_DESC,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub Category: D3D11_MESSAGE_CATEGORY,
    pub Severity: D3D11_MESSAGE_SEVERITY,
    pub ID: D3D11_MESSAGE_ID,
    pub Description: String,
}

impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:?}] [{:?}] [{}] {}",
            self.Severity, self.Category, self.ID, self.Description
        )
    }
}

#[repr(C)]
pub struct ID3D11InfoQueueVtbl {
    pub base: IUnknownVtbl,
    pub SetMessageCountLimit:
        unsafe extern "system" fn(this: *mut c_void, MessageCountLimit: u64) -> HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(this: *mut c_void),
    pub GetMessage: unsafe extern "system" fn(
        this: *mut c_void,
        MessageIndex: u64,
        pMessage: *mut D3D11_MESSAGE,
        pMessageByteLength: *mut usize,
    ) -> HRESULT,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(this: *mut c_void) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(this: *mut c_void) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(this: *mut c_void) -> u64,
    pub GetNumStoredMessagesAllowedByRetrievalFilter:
        unsafe extern "system" fn(this: *mut c_void) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit:
        unsafe extern "system" fn(this: *mut c_void) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(this: *mut c_void) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(
        this: *mut c_void,
        pFilter: *const D3D11_INFO_QUEUE_FILTER,
    ) -> HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(
        this: *mut c_void,
        pFilter: *mut D3D11_INFO_QUEUE_FILTER,
        pFilterByteLength: *mut usize,
    ) -> HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(this: *mut c_void),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(
        this: *mut c_void,
        pFilter: *const D3D11_INFO_QUEUE_FILTER,
    ) -> HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(this: *mut c_void),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(this: *mut c_void) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(
        this: *mut c_void,
        pFilter: *const D3D11_INFO_QUEUE_FILTER,
    ) -> HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(
        this: *mut c_void,
        pFilter: *mut D3D11_INFO_QUEUE_FILTER,
        pFilterByteLength: *mut usize,
    ) -> HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(this: *mut c_void),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(
        this: *mut c_void,
        pFilter: *const D3D11_INFO_QUEUE_FILTER,
    ) -> HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(this: *mut c_void),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(this: *mut c_void) -> u32,
    pub AddMessage: unsafe extern "system" fn(
        this: *mut c_void,
        Category: D3D11_MESSAGE_CATEGORY,
        Severity: D3D11_MESSAGE_SEVERITY,
        ID: D3D11_MESSAGE_ID,
        pDescription: *const u8,
    ) -> HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(
        this: *mut c_void,
        Severity: D3D11_MESSAGE_SEVERITY,
        pDescription: *const u8,
    ) -> HRESULT,
    pub SetBreakOnCategory: unsafe extern "system" fn(
        this: *mut c_void,
        Category: D3D11_MESSAGE_CATEGORY,
        bEnable: BOOL,
    ) -> HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(
        this: *mut c_void,
        Severity: D3D11_MESSAGE_SEVERITY,
        bEnable: BOOL,
    ) -> HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(
        this: *mut c_void,
        ID: D3D11_MESSAGE_ID,
        bEnable: BOOL,
    ) -> HRESULT,
    pub GetBreakOnCategory:
        unsafe extern "system" fn(this: *mut c_void, Category: D3D11_MESSAGE_CATEGORY) -> BOOL,
    pub GetBreakOnSeverity:
        unsafe extern "system" fn(this: *mut c_void, Severity: D3D11_MESSAGE_SEVERITY) -> BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(this: *mut c_void, ID: D3D11_MESSAGE_ID) -> BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(this: *mut c_void, bMute: BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(this: *mut c_void) -> BOOL,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11InfoQueue(pub *mut *const ID3D11InfoQueueVtbl);

impl ID3D11InfoQueue {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn SetMessageCountLimit(&self, MessageCountLimit: u64) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).SetMessageCountLimit)(self.0 as _, MessageCountLimit) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn ClearStoredMessages(&self) {
        unsafe { ((*(*self.0)).ClearStoredMessages)(self.0 as _) }
    }
    pub unsafe fn GetMessage(&self, MessageIndex: u64) -> Result<Message, HRESULT> {
        let mut len = 0usize;
        let hr = unsafe {
            ((*(*self.0)).GetMessage)(self.0 as _, MessageIndex, core::ptr::null_mut(), &mut len)
        };
        if hr < 0 {
            return Err(hr);
        }

        let mut buf = vec![0u64; len.div_ceil(8)];
        let msg = buf.as_mut_ptr() as *mut D3D11_MESSAGE;
        let hr = unsafe { ((*(*self.0)).GetMessage)(self.0 as _, MessageIndex, msg, &mut len) };
        if hr < 0 {
            return Err(hr);
        }

        let msg = unsafe { &*msg };
        let desc = unsafe { core::slice::from_raw_parts(msg.pDescription, msg.DescriptionByteLength) };
        let desc = desc.split(|b| *b == 0).next().unwrap_or(desc);
        Ok(Message {
            Category: msg.Category,
            Severity: msg.Severity,
            ID: msg.ID,
            Description: String::from_utf8_lossy(desc).into_owned(),
        })
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self) -> u64 {
        unsafe { ((*(*self.0)).GetNumMessagesAllowedByStorageFilter)(self.0 as _) }
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self) -> u64 {
        unsafe { ((*(*self.0)).GetNumMessagesDeniedByStorageFilter)(self.0 as _) }
    }
    pub unsafe fn GetNumStoredMessages(&self) -> u64 {
        unsafe { ((*(*self.0)).GetNumStoredMessages)(self.0 as _) }
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64 {
        unsafe { ((*(*self.0)).GetNumStoredMessagesAllowedByRetrievalFilter)(self.0 as _) }
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64 {
        unsafe { ((*(*self.0)).GetNumMessagesDiscardedByMessageCountLimit)(self.0 as _) }
    }
    pub unsafe fn GetMessageCountLimit(&self) -> u64 {
        unsafe { ((*(*self.0)).GetMessageCountLimit)(self.0 as _) }
    }
    pub unsafe fn AddStorageFilterEntries(
        &self,
        pFilter: &D3D11_INFO_QUEUE_FILTER,
    ) -> Result<(), HRESULT> {
        let hr =
            unsafe { ((*(*self.0)).AddStorageFilterEntries)(self.0 as _, pFilter as *const _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn ClearStorageFilter(&self) {
        unsafe { ((*(*self.0)).ClearStorageFilter)(self.0 as _) }
    }
    pub unsafe fn PushEmptyStorageFilter(&self) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).PushEmptyStorageFilter)(self.0 as _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn PushCopyOfStorageFilter(&self) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).PushCopyOfStorageFilter)(self.0 as _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn PushStorageFilter(
        &self,
        pFilter: &D3D11_INFO_QUEUE_FILTER,
    ) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).PushStorageFilter)(self.0 as _, pFilter as *const _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn PopStorageFilter(&self) {
        unsafe { ((*(*self.0)).PopStorageFilter)(self.0 as _) }
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        unsafe { ((*(*self.0)).GetStorageFilterStackSize)(self.0 as _) }
    }
    pub unsafe fn AddRetrievalFilterEntries(
        &self,
        pFilter: &D3D11_INFO_QUEUE_FILTER,
    ) -> Result<(), HRESULT> {
        let hr =
            unsafe { ((*(*self.0)).AddRetrievalFilterEntries)(self.0 as _, pFilter as *const _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn ClearRetrievalFilter(&self) {
        unsafe { ((*(*self.0)).ClearRetrievalFilter)(self.0 as _) }
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).PushEmptyRetrievalFilter)(self.0 as _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).PushCopyOfRetrievalFilter)(self.0 as _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn PushRetrievalFilter(
        &self,
        pFilter: &D3D11_INFO_QUEUE_FILTER,
    ) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).PushRetrievalFilter)(self.0 as _, pFilter as *const _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        unsafe { ((*(*self.0)).PopRetrievalFilter)(self.0 as _) }
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        unsafe { ((*(*self.0)).GetRetrievalFilterStackSize)(self.0 as _) }
    }
    pub unsafe fn AddMessage(
        &self,
        Category: D3D11_MESSAGE_CATEGORY,
        Severity: D3D11_MESSAGE_SEVERITY,
        ID: D3D11_MESSAGE_ID,
        pDescription: &core::ffi::CStr,
    ) -> Result<(), HRESULT> {
        let hr = unsafe {
            ((*(*self.0)).AddMessage)(
                self.0 as _,
                Category,
                Severity,
                ID,
                pDescription.as_ptr() as *const u8,
            )
        };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn AddApplicationMessage(
        &self,
        Severity: D3D11_MESSAGE_SEVERITY,
        pDescription: &core::ffi::CStr,
    ) -> Result<(), HRESULT> {
        let hr = unsafe {
            ((*(*self.0)).AddApplicationMessage)(
                self.0 as _,
                Severity,
                pDescription.as_ptr() as *const u8,
            )
        };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn SetBreakOnCategory(
        &self,
        Category: D3D11_MESSAGE_CATEGORY,
        bEnable: BOOL,
    ) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).SetBreakOnCategory)(self.0 as _, Category, bEnable) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn SetBreakOnSeverity(
        &self,
        Severity: D3D11_MESSAGE_SEVERITY,
        bEnable: BOOL,
    ) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).SetBreakOnSeverity)(self.0 as _, Severity, bEnable) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn SetBreakOnID(&self, ID: D3D11_MESSAGE_ID, bEnable: BOOL) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).SetBreakOnID)(self.0 as _, ID, bEnable) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn GetBreakOnCategory(&self, Category: D3D11_MESSAGE_CATEGORY) -> BOOL {
        unsafe { ((*(*self.0)).GetBreakOnCategory)(self.0 as _, Category) }
    }
    pub unsafe fn GetBreakOnSeverity(&self, Severity: D3D11_MESSAGE_SEVERITY) -> BOOL {
        unsafe { ((*(*self.0)).GetBreakOnSeverity)(self.0 as _, Severity) }
    }
    pub unsafe fn GetBreakOnID(&self, ID: D3D11_MESSAGE_ID) -> BOOL {
        unsafe { ((*(*self.0)).GetBreakOnID)(self.0 as _, ID) }
    }
    pub unsafe fn SetMuteDebugOutput(&self, bMute: BOOL) {
        unsafe { ((*(*self.0)).SetMuteDebugOutput)(self.0 as _, bMute) }
    }
    pub unsafe fn GetMuteDebugOutput(&self) -> BOOL {
        unsafe { ((*(*self.0)).GetMuteDebugOutput)(self.0 as _) }
    }
}
