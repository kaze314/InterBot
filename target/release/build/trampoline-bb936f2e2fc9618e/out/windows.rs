#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(
                :: std :: default :: Default,
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::Abi for BOOL {
                type Abi = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::HRESULT::from_thread().into())
                    }
                }
                #[inline]
                #[track_caller]
                pub fn unwrap(self) {
                    self.ok().unwrap();
                }
                #[inline]
                #[track_caller]
                pub fn expect(self, msg: &str) {
                    self.ok().expect(msg);
                }
            }
            impl ::std::convert::From<BOOL> for bool {
                fn from(value: BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<&BOOL> for bool {
                fn from(value: &BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<bool> for BOOL {
                fn from(value: bool) -> Self {
                    if value {
                        BOOL(1)
                    } else {
                        BOOL(0)
                    }
                }
            }
            impl ::std::convert::From<&bool> for BOOL {
                fn from(value: &bool) -> Self {
                    (*value).into()
                }
            }
            impl ::std::cmp::PartialEq<bool> for BOOL {
                fn eq(&self, other: &bool) -> bool {
                    self.as_bool() == *other
                }
            }
            impl ::std::cmp::PartialEq<BOOL> for bool {
                fn eq(&self, other: &BOOL) -> bool {
                    *self == other.as_bool()
                }
            }
            impl std::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::Param<'a, BOOL> {
                    ::windows::Param::Owned(self.into())
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Memory {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct PAGE_PROTECTION_FLAGS(pub u32);
                pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1u32);
                pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2u32);
                pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4u32);
                pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8u32);
                pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16u32);
                pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32u32);
                pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(64u32);
                pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(128u32);
                pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(256u32);
                pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(512u32);
                pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1024u32);
                pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(2048u32);
                pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(4096u32);
                pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(8192u32);
                pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(16384u32);
                pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(32768u32);
                pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(65536u32);
                pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(131072u32);
                pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(262144u32);
                pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(2147483648u32);
                pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(2147483648u32);
                pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(1073741824u32);
                pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(1073741824u32);
                pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(536870912u32);
                pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(268435456u32);
                pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(268435456u32);
                pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(268435457u32);
                pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(268435458u32);
                pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(262144u32);
                pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(524288u32);
                pub const SEC_FILE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8388608u32);
                pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16777216u32);
                pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(33554432u32);
                pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(67108864u32);
                pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(134217728u32);
                pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
                pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(1073741824u32);
                pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(2147483648u32);
                pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS =
                    PAGE_PROTECTION_FLAGS(285212672u32);
                impl ::std::convert::From<u32> for PAGE_PROTECTION_FLAGS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for PAGE_PROTECTION_FLAGS {
                    type Abi = Self;
                }
                impl ::std::ops::BitOr for PAGE_PROTECTION_FLAGS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for PAGE_PROTECTION_FLAGS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for PAGE_PROTECTION_FLAGS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for PAGE_PROTECTION_FLAGS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct VIRTUAL_ALLOCATION_TYPE(pub u32);
                pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(4096u32);
                pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(8192u32);
                pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(524288u32);
                pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE =
                    VIRTUAL_ALLOCATION_TYPE(16777216u32);
                pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE =
                    VIRTUAL_ALLOCATION_TYPE(16384u32);
                pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE =
                    VIRTUAL_ALLOCATION_TYPE(536870912u32);
                pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE =
                    VIRTUAL_ALLOCATION_TYPE(262144u32);
                pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(65536u32);
                impl ::std::convert::From<u32> for VIRTUAL_ALLOCATION_TYPE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for VIRTUAL_ALLOCATION_TYPE {
                    type Abi = Self;
                }
                impl ::std::ops::BitOr for VIRTUAL_ALLOCATION_TYPE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for VIRTUAL_ALLOCATION_TYPE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for VIRTUAL_ALLOCATION_TYPE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for VIRTUAL_ALLOCATION_TYPE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct VIRTUAL_FREE_TYPE(pub u32);
                pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(16384u32);
                pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(32768u32);
                impl ::std::convert::From<u32> for VIRTUAL_FREE_TYPE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for VIRTUAL_FREE_TYPE {
                    type Abi = Self;
                }
                impl ::std::ops::BitOr for VIRTUAL_FREE_TYPE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for VIRTUAL_FREE_TYPE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for VIRTUAL_FREE_TYPE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for VIRTUAL_FREE_TYPE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub unsafe fn VirtualAlloc(
                    lpaddress: *mut ::std::ffi::c_void,
                    dwsize: usize,
                    flallocationtype: VIRTUAL_ALLOCATION_TYPE,
                    flprotect: PAGE_PROTECTION_FLAGS,
                ) -> *mut ::std::ffi::c_void {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn VirtualAlloc(
                                lpaddress: *mut ::std::ffi::c_void,
                                dwsize: usize,
                                flallocationtype: VIRTUAL_ALLOCATION_TYPE,
                                flprotect: PAGE_PROTECTION_FLAGS,
                            ) -> *mut ::std::ffi::c_void;
                        }
                        VirtualAlloc(
                            ::std::mem::transmute(lpaddress),
                            ::std::mem::transmute(dwsize),
                            ::std::mem::transmute(flallocationtype),
                            ::std::mem::transmute(flprotect),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn VirtualFree(
                    lpaddress: *mut ::std::ffi::c_void,
                    dwsize: usize,
                    dwfreetype: VIRTUAL_FREE_TYPE,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn VirtualFree(
                                lpaddress: *mut ::std::ffi::c_void,
                                dwsize: usize,
                                dwfreetype: VIRTUAL_FREE_TYPE,
                            ) -> super::super::Foundation::BOOL;
                        }
                        VirtualFree(
                            ::std::mem::transmute(lpaddress),
                            ::std::mem::transmute(dwsize),
                            ::std::mem::transmute(dwfreetype),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn VirtualProtect(
                    lpaddress: *mut ::std::ffi::c_void,
                    dwsize: usize,
                    flnewprotect: PAGE_PROTECTION_FLAGS,
                    lpfloldprotect: *mut PAGE_PROTECTION_FLAGS,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn VirtualProtect(
                                lpaddress: *mut ::std::ffi::c_void,
                                dwsize: usize,
                                flnewprotect: PAGE_PROTECTION_FLAGS,
                                lpfloldprotect: *mut PAGE_PROTECTION_FLAGS,
                            ) -> super::super::Foundation::BOOL;
                        }
                        VirtualProtect(
                            ::std::mem::transmute(lpaddress),
                            ::std::mem::transmute(dwsize),
                            ::std::mem::transmute(flnewprotect),
                            ::std::mem::transmute(lpfloldprotect),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
    }
}
