#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdctl: WDCTL,
    _reserved1: [u8; 0x3c],
    st0: ST0,
    st1: ST1,
    st2: ST2,
    st3: ST3,
    stload: STLOAD,
    stcc: STCC,
    stcs: STCS,
    stcv0: STCV0,
    stcv1: STCV1,
    stcv2: STCV2,
    stcv3: STCV3,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control"]
    #[inline(always)]
    pub const fn wdctl(&self) -> &WDCTL {
        &self.wdctl
    }
    #[doc = "0x40 - Sleep Timer 0 count and compare"]
    #[inline(always)]
    pub const fn st0(&self) -> &ST0 {
        &self.st0
    }
    #[doc = "0x44 - Sleep Timer 1 count and compare"]
    #[inline(always)]
    pub const fn st1(&self) -> &ST1 {
        &self.st1
    }
    #[doc = "0x48 - Sleep Timer 2 count and compare"]
    #[inline(always)]
    pub const fn st2(&self) -> &ST2 {
        &self.st2
    }
    #[doc = "0x4c - Sleep Timer 3 count and compare"]
    #[inline(always)]
    pub const fn st3(&self) -> &ST3 {
        &self.st3
    }
    #[doc = "0x50 - Sleep Timer load status"]
    #[inline(always)]
    pub const fn stload(&self) -> &STLOAD {
        &self.stload
    }
    #[doc = "0x54 - Sleep Timer Capture control"]
    #[inline(always)]
    pub const fn stcc(&self) -> &STCC {
        &self.stcc
    }
    #[doc = "0x58 - Sleep Timer Capture status"]
    #[inline(always)]
    pub const fn stcs(&self) -> &STCS {
        &self.stcs
    }
    #[doc = "0x5c - Sleep Timer Capture value byte 0"]
    #[inline(always)]
    pub const fn stcv0(&self) -> &STCV0 {
        &self.stcv0
    }
    #[doc = "0x60 - Sleep Timer Capture value byte 1"]
    #[inline(always)]
    pub const fn stcv1(&self) -> &STCV1 {
        &self.stcv1
    }
    #[doc = "0x64 - Sleep Timer Capture value byte 2"]
    #[inline(always)]
    pub const fn stcv2(&self) -> &STCV2 {
        &self.stcv2
    }
    #[doc = "0x68 - Sleep Timer Capture value byte 3"]
    #[inline(always)]
    pub const fn stcv3(&self) -> &STCV3 {
        &self.stcv3
    }
}
#[doc = "WDCTL (rw) register accessor: Watchdog Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdctl`]
module"]
pub type WDCTL = crate::Reg<wdctl::WDCTL_SPEC>;
#[doc = "Watchdog Timer Control"]
pub mod wdctl;
#[doc = "ST0 (rw) register accessor: Sleep Timer 0 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st0`]
module"]
pub type ST0 = crate::Reg<st0::ST0_SPEC>;
#[doc = "Sleep Timer 0 count and compare"]
pub mod st0;
#[doc = "ST1 (rw) register accessor: Sleep Timer 1 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st1`]
module"]
pub type ST1 = crate::Reg<st1::ST1_SPEC>;
#[doc = "Sleep Timer 1 count and compare"]
pub mod st1;
#[doc = "ST2 (rw) register accessor: Sleep Timer 2 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2`]
module"]
pub type ST2 = crate::Reg<st2::ST2_SPEC>;
#[doc = "Sleep Timer 2 count and compare"]
pub mod st2;
#[doc = "ST3 (rw) register accessor: Sleep Timer 3 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st3`]
module"]
pub type ST3 = crate::Reg<st3::ST3_SPEC>;
#[doc = "Sleep Timer 3 count and compare"]
pub mod st3;
#[doc = "STLOAD (r) register accessor: Sleep Timer load status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stload::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stload`]
module"]
pub type STLOAD = crate::Reg<stload::STLOAD_SPEC>;
#[doc = "Sleep Timer load status"]
pub mod stload;
#[doc = "STCC (rw) register accessor: Sleep Timer Capture control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcc`]
module"]
pub type STCC = crate::Reg<stcc::STCC_SPEC>;
#[doc = "Sleep Timer Capture control"]
pub mod stcc;
#[doc = "STCS (rw) register accessor: Sleep Timer Capture status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcs`]
module"]
pub type STCS = crate::Reg<stcs::STCS_SPEC>;
#[doc = "Sleep Timer Capture status"]
pub mod stcs;
#[doc = "STCV0 (r) register accessor: Sleep Timer Capture value byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcv0`]
module"]
pub type STCV0 = crate::Reg<stcv0::STCV0_SPEC>;
#[doc = "Sleep Timer Capture value byte 0"]
pub mod stcv0;
#[doc = "STCV1 (r) register accessor: Sleep Timer Capture value byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcv1`]
module"]
pub type STCV1 = crate::Reg<stcv1::STCV1_SPEC>;
#[doc = "Sleep Timer Capture value byte 1"]
pub mod stcv1;
#[doc = "STCV2 (r) register accessor: Sleep Timer Capture value byte 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcv2`]
module"]
pub type STCV2 = crate::Reg<stcv2::STCV2_SPEC>;
#[doc = "Sleep Timer Capture value byte 2"]
pub mod stcv2;
#[doc = "STCV3 (r) register accessor: Sleep Timer Capture value byte 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcv3`]
module"]
pub type STCV3 = crate::Reg<stcv3::STCV3_SPEC>;
#[doc = "Sleep Timer Capture value byte 3"]
pub mod stcv3;
