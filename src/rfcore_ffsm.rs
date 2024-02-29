#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    srcresmask0: Srcresmask0,
    srcresmask1: Srcresmask1,
    srcresmask2: Srcresmask2,
    srcresindex: Srcresindex,
    srcextpenden0: Srcextpenden0,
    srcextpenden1: Srcextpenden1,
    srcextpenden2: Srcextpenden2,
    srcshortpenden0: Srcshortpenden0,
    srcshortpenden1: Srcshortpenden1,
    srcshortpenden2: Srcshortpenden2,
    ext_addr0: ExtAddr0,
    ext_addr1: ExtAddr1,
    ext_addr2: ExtAddr2,
    ext_addr3: ExtAddr3,
    ext_addr4: ExtAddr4,
    ext_addr5: ExtAddr5,
    ext_addr6: ExtAddr6,
    ext_addr7: ExtAddr7,
    pan_id0: PanId0,
    pan_id1: PanId1,
    short_addr0: ShortAddr0,
    short_addr1: ShortAddr1,
}
impl RegisterBlock {
    #[doc = "0x80 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcresmask0(&self) -> &Srcresmask0 {
        &self.srcresmask0
    }
    #[doc = "0x84 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcresmask1(&self) -> &Srcresmask1 {
        &self.srcresmask1
    }
    #[doc = "0x88 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcresmask2(&self) -> &Srcresmask2 {
        &self.srcresmask2
    }
    #[doc = "0x8c - Source address matching result This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcresindex(&self) -> &Srcresindex {
        &self.srcresindex
    }
    #[doc = "0x90 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcextpenden0(&self) -> &Srcextpenden0 {
        &self.srcextpenden0
    }
    #[doc = "0x94 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcextpenden1(&self) -> &Srcextpenden1 {
        &self.srcextpenden1
    }
    #[doc = "0x98 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcextpenden2(&self) -> &Srcextpenden2 {
        &self.srcextpenden2
    }
    #[doc = "0x9c - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcshortpenden0(&self) -> &Srcshortpenden0 {
        &self.srcshortpenden0
    }
    #[doc = "0xa0 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcshortpenden1(&self) -> &Srcshortpenden1 {
        &self.srcshortpenden1
    }
    #[doc = "0xa4 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcshortpenden2(&self) -> &Srcshortpenden2 {
        &self.srcshortpenden2
    }
    #[doc = "0xa8 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr0(&self) -> &ExtAddr0 {
        &self.ext_addr0
    }
    #[doc = "0xac - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr1(&self) -> &ExtAddr1 {
        &self.ext_addr1
    }
    #[doc = "0xb0 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr2(&self) -> &ExtAddr2 {
        &self.ext_addr2
    }
    #[doc = "0xb4 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr3(&self) -> &ExtAddr3 {
        &self.ext_addr3
    }
    #[doc = "0xb8 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr4(&self) -> &ExtAddr4 {
        &self.ext_addr4
    }
    #[doc = "0xbc - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr5(&self) -> &ExtAddr5 {
        &self.ext_addr5
    }
    #[doc = "0xc0 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr6(&self) -> &ExtAddr6 {
        &self.ext_addr6
    }
    #[doc = "0xc4 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr7(&self) -> &ExtAddr7 {
        &self.ext_addr7
    }
    #[doc = "0xc8 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn pan_id0(&self) -> &PanId0 {
        &self.pan_id0
    }
    #[doc = "0xcc - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn pan_id1(&self) -> &PanId1 {
        &self.pan_id1
    }
    #[doc = "0xd0 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn short_addr0(&self) -> &ShortAddr0 {
        &self.short_addr0
    }
    #[doc = "0xd4 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn short_addr1(&self) -> &ShortAddr1 {
        &self.short_addr1
    }
}
#[doc = "SRCRESMASK0 (rw) register accessor: Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcresmask0`]
module"]
#[doc(alias = "SRCRESMASK0")]
pub type Srcresmask0 = crate::Reg<srcresmask0::Srcresmask0Spec>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask0;
#[doc = "SRCRESMASK1 (rw) register accessor: Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcresmask1`]
module"]
#[doc(alias = "SRCRESMASK1")]
pub type Srcresmask1 = crate::Reg<srcresmask1::Srcresmask1Spec>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask1;
#[doc = "SRCRESMASK2 (rw) register accessor: Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcresmask2`]
module"]
#[doc(alias = "SRCRESMASK2")]
pub type Srcresmask2 = crate::Reg<srcresmask2::Srcresmask2Spec>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask2;
#[doc = "SRCRESINDEX (rw) register accessor: Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresindex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresindex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcresindex`]
module"]
#[doc(alias = "SRCRESINDEX")]
pub type Srcresindex = crate::Reg<srcresindex::SrcresindexSpec>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresindex;
#[doc = "SRCEXTPENDEN0 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcextpenden0`]
module"]
#[doc(alias = "SRCEXTPENDEN0")]
pub type Srcextpenden0 = crate::Reg<srcextpenden0::Srcextpenden0Spec>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden0;
#[doc = "SRCEXTPENDEN1 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcextpenden1`]
module"]
#[doc(alias = "SRCEXTPENDEN1")]
pub type Srcextpenden1 = crate::Reg<srcextpenden1::Srcextpenden1Spec>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden1;
#[doc = "SRCEXTPENDEN2 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcextpenden2`]
module"]
#[doc(alias = "SRCEXTPENDEN2")]
pub type Srcextpenden2 = crate::Reg<srcextpenden2::Srcextpenden2Spec>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden2;
#[doc = "SRCSHORTPENDEN0 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshortpenden0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshortpenden0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshortpenden0`]
module"]
#[doc(alias = "SRCSHORTPENDEN0")]
pub type Srcshortpenden0 = crate::Reg<srcshortpenden0::Srcshortpenden0Spec>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden0;
#[doc = "SRCSHORTPENDEN1 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshortpenden1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshortpenden1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshortpenden1`]
module"]
#[doc(alias = "SRCSHORTPENDEN1")]
pub type Srcshortpenden1 = crate::Reg<srcshortpenden1::Srcshortpenden1Spec>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden1;
#[doc = "SRCSHORTPENDEN2 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshortpenden2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshortpenden2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshortpenden2`]
module"]
#[doc(alias = "SRCSHORTPENDEN2")]
pub type Srcshortpenden2 = crate::Reg<srcshortpenden2::Srcshortpenden2Spec>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden2;
#[doc = "EXT_ADDR0 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr0`]
module"]
#[doc(alias = "EXT_ADDR0")]
pub type ExtAddr0 = crate::Reg<ext_addr0::ExtAddr0Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr0;
#[doc = "EXT_ADDR1 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr1`]
module"]
#[doc(alias = "EXT_ADDR1")]
pub type ExtAddr1 = crate::Reg<ext_addr1::ExtAddr1Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr1;
#[doc = "EXT_ADDR2 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr2`]
module"]
#[doc(alias = "EXT_ADDR2")]
pub type ExtAddr2 = crate::Reg<ext_addr2::ExtAddr2Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr2;
#[doc = "EXT_ADDR3 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr3`]
module"]
#[doc(alias = "EXT_ADDR3")]
pub type ExtAddr3 = crate::Reg<ext_addr3::ExtAddr3Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr3;
#[doc = "EXT_ADDR4 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr4`]
module"]
#[doc(alias = "EXT_ADDR4")]
pub type ExtAddr4 = crate::Reg<ext_addr4::ExtAddr4Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr4;
#[doc = "EXT_ADDR5 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr5`]
module"]
#[doc(alias = "EXT_ADDR5")]
pub type ExtAddr5 = crate::Reg<ext_addr5::ExtAddr5Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr5;
#[doc = "EXT_ADDR6 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr6`]
module"]
#[doc(alias = "EXT_ADDR6")]
pub type ExtAddr6 = crate::Reg<ext_addr6::ExtAddr6Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr6;
#[doc = "EXT_ADDR7 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr7`]
module"]
#[doc(alias = "EXT_ADDR7")]
pub type ExtAddr7 = crate::Reg<ext_addr7::ExtAddr7Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr7;
#[doc = "PAN_ID0 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pan_id0`]
module"]
#[doc(alias = "PAN_ID0")]
pub type PanId0 = crate::Reg<pan_id0::PanId0Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod pan_id0;
#[doc = "PAN_ID1 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pan_id1`]
module"]
#[doc(alias = "PAN_ID1")]
pub type PanId1 = crate::Reg<pan_id1::PanId1Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod pan_id1;
#[doc = "SHORT_ADDR0 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@short_addr0`]
module"]
#[doc(alias = "SHORT_ADDR0")]
pub type ShortAddr0 = crate::Reg<short_addr0::ShortAddr0Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod short_addr0;
#[doc = "SHORT_ADDR1 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@short_addr1`]
module"]
#[doc(alias = "SHORT_ADDR1")]
pub type ShortAddr1 = crate::Reg<short_addr1::ShortAddr1Spec>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod short_addr1;
