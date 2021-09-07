#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control"]
    pub wdctl: crate::Reg<wdctl::WDCTL_SPEC>,
    _reserved1: [u8; 0x3c],
    #[doc = "0x40 - Sleep Timer 0 count and compare"]
    pub st0: crate::Reg<st0::ST0_SPEC>,
    #[doc = "0x44 - Sleep Timer 1 count and compare"]
    pub st1: crate::Reg<st1::ST1_SPEC>,
    #[doc = "0x48 - Sleep Timer 2 count and compare"]
    pub st2: crate::Reg<st2::ST2_SPEC>,
    #[doc = "0x4c - Sleep Timer 3 count and compare"]
    pub st3: crate::Reg<st3::ST3_SPEC>,
    #[doc = "0x50 - Sleep Timer load status"]
    pub stload: crate::Reg<stload::STLOAD_SPEC>,
    #[doc = "0x54 - Sleep Timer Capture control"]
    pub stcc: crate::Reg<stcc::STCC_SPEC>,
    #[doc = "0x58 - Sleep Timer Capture status"]
    pub stcs: crate::Reg<stcs::STCS_SPEC>,
    #[doc = "0x5c - Sleep Timer Capture value byte 0"]
    pub stcv0: crate::Reg<stcv0::STCV0_SPEC>,
    #[doc = "0x60 - Sleep Timer Capture value byte 1"]
    pub stcv1: crate::Reg<stcv1::STCV1_SPEC>,
    #[doc = "0x64 - Sleep Timer Capture value byte 2"]
    pub stcv2: crate::Reg<stcv2::STCV2_SPEC>,
    #[doc = "0x68 - Sleep Timer Capture value byte 3"]
    pub stcv3: crate::Reg<stcv3::STCV3_SPEC>,
}
#[doc = "WDCTL register accessor: an alias for `Reg<WDCTL_SPEC>`"]
pub type WDCTL = crate::Reg<wdctl::WDCTL_SPEC>;
#[doc = "Watchdog Timer Control"]
pub mod wdctl;
#[doc = "ST0 register accessor: an alias for `Reg<ST0_SPEC>`"]
pub type ST0 = crate::Reg<st0::ST0_SPEC>;
#[doc = "Sleep Timer 0 count and compare"]
pub mod st0;
#[doc = "ST1 register accessor: an alias for `Reg<ST1_SPEC>`"]
pub type ST1 = crate::Reg<st1::ST1_SPEC>;
#[doc = "Sleep Timer 1 count and compare"]
pub mod st1;
#[doc = "ST2 register accessor: an alias for `Reg<ST2_SPEC>`"]
pub type ST2 = crate::Reg<st2::ST2_SPEC>;
#[doc = "Sleep Timer 2 count and compare"]
pub mod st2;
#[doc = "ST3 register accessor: an alias for `Reg<ST3_SPEC>`"]
pub type ST3 = crate::Reg<st3::ST3_SPEC>;
#[doc = "Sleep Timer 3 count and compare"]
pub mod st3;
#[doc = "STLOAD register accessor: an alias for `Reg<STLOAD_SPEC>`"]
pub type STLOAD = crate::Reg<stload::STLOAD_SPEC>;
#[doc = "Sleep Timer load status"]
pub mod stload;
#[doc = "STCC register accessor: an alias for `Reg<STCC_SPEC>`"]
pub type STCC = crate::Reg<stcc::STCC_SPEC>;
#[doc = "Sleep Timer Capture control"]
pub mod stcc;
#[doc = "STCS register accessor: an alias for `Reg<STCS_SPEC>`"]
pub type STCS = crate::Reg<stcs::STCS_SPEC>;
#[doc = "Sleep Timer Capture status"]
pub mod stcs;
#[doc = "STCV0 register accessor: an alias for `Reg<STCV0_SPEC>`"]
pub type STCV0 = crate::Reg<stcv0::STCV0_SPEC>;
#[doc = "Sleep Timer Capture value byte 0"]
pub mod stcv0;
#[doc = "STCV1 register accessor: an alias for `Reg<STCV1_SPEC>`"]
pub type STCV1 = crate::Reg<stcv1::STCV1_SPEC>;
#[doc = "Sleep Timer Capture value byte 1"]
pub mod stcv1;
#[doc = "STCV2 register accessor: an alias for `Reg<STCV2_SPEC>`"]
pub type STCV2 = crate::Reg<stcv2::STCV2_SPEC>;
#[doc = "Sleep Timer Capture value byte 2"]
pub mod stcv2;
#[doc = "STCV3 register accessor: an alias for `Reg<STCV3_SPEC>`"]
pub type STCV3 = crate::Reg<stcv3::STCV3_SPEC>;
#[doc = "Sleep Timer Capture value byte 3"]
pub mod stcv3;
