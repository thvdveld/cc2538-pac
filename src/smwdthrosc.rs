#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdctl: Wdctl,
    _reserved1: [u8; 0x3c],
    st0: St0,
    st1: St1,
    st2: St2,
    st3: St3,
    stload: Stload,
    stcc: Stcc,
    stcs: Stcs,
    stcv0: Stcv0,
    stcv1: Stcv1,
    stcv2: Stcv2,
    stcv3: Stcv3,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control"]
    #[inline(always)]
    pub const fn wdctl(&self) -> &Wdctl {
        &self.wdctl
    }
    #[doc = "0x40 - Sleep Timer 0 count and compare"]
    #[inline(always)]
    pub const fn st0(&self) -> &St0 {
        &self.st0
    }
    #[doc = "0x44 - Sleep Timer 1 count and compare"]
    #[inline(always)]
    pub const fn st1(&self) -> &St1 {
        &self.st1
    }
    #[doc = "0x48 - Sleep Timer 2 count and compare"]
    #[inline(always)]
    pub const fn st2(&self) -> &St2 {
        &self.st2
    }
    #[doc = "0x4c - Sleep Timer 3 count and compare"]
    #[inline(always)]
    pub const fn st3(&self) -> &St3 {
        &self.st3
    }
    #[doc = "0x50 - Sleep Timer load status"]
    #[inline(always)]
    pub const fn stload(&self) -> &Stload {
        &self.stload
    }
    #[doc = "0x54 - Sleep Timer Capture control"]
    #[inline(always)]
    pub const fn stcc(&self) -> &Stcc {
        &self.stcc
    }
    #[doc = "0x58 - Sleep Timer Capture status"]
    #[inline(always)]
    pub const fn stcs(&self) -> &Stcs {
        &self.stcs
    }
    #[doc = "0x5c - Sleep Timer Capture value byte 0"]
    #[inline(always)]
    pub const fn stcv0(&self) -> &Stcv0 {
        &self.stcv0
    }
    #[doc = "0x60 - Sleep Timer Capture value byte 1"]
    #[inline(always)]
    pub const fn stcv1(&self) -> &Stcv1 {
        &self.stcv1
    }
    #[doc = "0x64 - Sleep Timer Capture value byte 2"]
    #[inline(always)]
    pub const fn stcv2(&self) -> &Stcv2 {
        &self.stcv2
    }
    #[doc = "0x68 - Sleep Timer Capture value byte 3"]
    #[inline(always)]
    pub const fn stcv3(&self) -> &Stcv3 {
        &self.stcv3
    }
}
#[doc = "WDCTL (rw) register accessor: Watchdog Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdctl`]
module"]
#[doc(alias = "WDCTL")]
pub type Wdctl = crate::Reg<wdctl::WdctlSpec>;
#[doc = "Watchdog Timer Control"]
pub mod wdctl;
#[doc = "ST0 (rw) register accessor: Sleep Timer 0 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st0`]
module"]
#[doc(alias = "ST0")]
pub type St0 = crate::Reg<st0::St0Spec>;
#[doc = "Sleep Timer 0 count and compare"]
pub mod st0;
#[doc = "ST1 (rw) register accessor: Sleep Timer 1 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st1`]
module"]
#[doc(alias = "ST1")]
pub type St1 = crate::Reg<st1::St1Spec>;
#[doc = "Sleep Timer 1 count and compare"]
pub mod st1;
#[doc = "ST2 (rw) register accessor: Sleep Timer 2 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2`]
module"]
#[doc(alias = "ST2")]
pub type St2 = crate::Reg<st2::St2Spec>;
#[doc = "Sleep Timer 2 count and compare"]
pub mod st2;
#[doc = "ST3 (rw) register accessor: Sleep Timer 3 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st3`]
module"]
#[doc(alias = "ST3")]
pub type St3 = crate::Reg<st3::St3Spec>;
#[doc = "Sleep Timer 3 count and compare"]
pub mod st3;
#[doc = "STLOAD (r) register accessor: Sleep Timer load status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stload::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stload`]
module"]
#[doc(alias = "STLOAD")]
pub type Stload = crate::Reg<stload::StloadSpec>;
#[doc = "Sleep Timer load status"]
pub mod stload;
#[doc = "STCC (rw) register accessor: Sleep Timer Capture control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcc`]
module"]
#[doc(alias = "STCC")]
pub type Stcc = crate::Reg<stcc::StccSpec>;
#[doc = "Sleep Timer Capture control"]
pub mod stcc;
#[doc = "STCS (rw) register accessor: Sleep Timer Capture status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcs`]
module"]
#[doc(alias = "STCS")]
pub type Stcs = crate::Reg<stcs::StcsSpec>;
#[doc = "Sleep Timer Capture status"]
pub mod stcs;
#[doc = "STCV0 (r) register accessor: Sleep Timer Capture value byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcv0`]
module"]
#[doc(alias = "STCV0")]
pub type Stcv0 = crate::Reg<stcv0::Stcv0Spec>;
#[doc = "Sleep Timer Capture value byte 0"]
pub mod stcv0;
#[doc = "STCV1 (r) register accessor: Sleep Timer Capture value byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcv1`]
module"]
#[doc(alias = "STCV1")]
pub type Stcv1 = crate::Reg<stcv1::Stcv1Spec>;
#[doc = "Sleep Timer Capture value byte 1"]
pub mod stcv1;
#[doc = "STCV2 (r) register accessor: Sleep Timer Capture value byte 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcv2`]
module"]
#[doc(alias = "STCV2")]
pub type Stcv2 = crate::Reg<stcv2::Stcv2Spec>;
#[doc = "Sleep Timer Capture value byte 2"]
pub mod stcv2;
#[doc = "STCV3 (r) register accessor: Sleep Timer Capture value byte 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcv3`]
module"]
#[doc(alias = "STCV3")]
pub type Stcv3 = crate::Reg<stcv3::Stcv3Spec>;
#[doc = "Sleep Timer Capture value byte 3"]
pub mod stcv3;
