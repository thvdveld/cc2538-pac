#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    srcresmask0: SRCRESMASK0,
    srcresmask1: SRCRESMASK1,
    srcresmask2: SRCRESMASK2,
    srcresindex: SRCRESINDEX,
    srcextpenden0: SRCEXTPENDEN0,
    srcextpenden1: SRCEXTPENDEN1,
    srcextpenden2: SRCEXTPENDEN2,
    srcshortpenden0: SRCSHORTPENDEN0,
    srcshortpenden1: SRCSHORTPENDEN1,
    srcshortpenden2: SRCSHORTPENDEN2,
    ext_addr0: EXT_ADDR0,
    ext_addr1: EXT_ADDR1,
    ext_addr2: EXT_ADDR2,
    ext_addr3: EXT_ADDR3,
    ext_addr4: EXT_ADDR4,
    ext_addr5: EXT_ADDR5,
    ext_addr6: EXT_ADDR6,
    ext_addr7: EXT_ADDR7,
    pan_id0: PAN_ID0,
    pan_id1: PAN_ID1,
    short_addr0: SHORT_ADDR0,
    short_addr1: SHORT_ADDR1,
}
impl RegisterBlock {
    #[doc = "0x80 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcresmask0(&self) -> &SRCRESMASK0 {
        &self.srcresmask0
    }
    #[doc = "0x84 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcresmask1(&self) -> &SRCRESMASK1 {
        &self.srcresmask1
    }
    #[doc = "0x88 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcresmask2(&self) -> &SRCRESMASK2 {
        &self.srcresmask2
    }
    #[doc = "0x8c - Source address matching result This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcresindex(&self) -> &SRCRESINDEX {
        &self.srcresindex
    }
    #[doc = "0x90 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcextpenden0(&self) -> &SRCEXTPENDEN0 {
        &self.srcextpenden0
    }
    #[doc = "0x94 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcextpenden1(&self) -> &SRCEXTPENDEN1 {
        &self.srcextpenden1
    }
    #[doc = "0x98 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcextpenden2(&self) -> &SRCEXTPENDEN2 {
        &self.srcextpenden2
    }
    #[doc = "0x9c - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcshortpenden0(&self) -> &SRCSHORTPENDEN0 {
        &self.srcshortpenden0
    }
    #[doc = "0xa0 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcshortpenden1(&self) -> &SRCSHORTPENDEN1 {
        &self.srcshortpenden1
    }
    #[doc = "0xa4 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn srcshortpenden2(&self) -> &SRCSHORTPENDEN2 {
        &self.srcshortpenden2
    }
    #[doc = "0xa8 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr0(&self) -> &EXT_ADDR0 {
        &self.ext_addr0
    }
    #[doc = "0xac - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr1(&self) -> &EXT_ADDR1 {
        &self.ext_addr1
    }
    #[doc = "0xb0 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr2(&self) -> &EXT_ADDR2 {
        &self.ext_addr2
    }
    #[doc = "0xb4 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr3(&self) -> &EXT_ADDR3 {
        &self.ext_addr3
    }
    #[doc = "0xb8 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr4(&self) -> &EXT_ADDR4 {
        &self.ext_addr4
    }
    #[doc = "0xbc - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr5(&self) -> &EXT_ADDR5 {
        &self.ext_addr5
    }
    #[doc = "0xc0 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr6(&self) -> &EXT_ADDR6 {
        &self.ext_addr6
    }
    #[doc = "0xc4 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn ext_addr7(&self) -> &EXT_ADDR7 {
        &self.ext_addr7
    }
    #[doc = "0xc8 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn pan_id0(&self) -> &PAN_ID0 {
        &self.pan_id0
    }
    #[doc = "0xcc - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn pan_id1(&self) -> &PAN_ID1 {
        &self.pan_id1
    }
    #[doc = "0xd0 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn short_addr0(&self) -> &SHORT_ADDR0 {
        &self.short_addr0
    }
    #[doc = "0xd4 - Local address information This register is stored in RAM; the reset value is undefined."]
    #[inline(always)]
    pub const fn short_addr1(&self) -> &SHORT_ADDR1 {
        &self.short_addr1
    }
}
#[doc = "SRCRESMASK0 (rw) register accessor: Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcresmask0`]
module"]
pub type SRCRESMASK0 = crate::Reg<srcresmask0::SRCRESMASK0_SPEC>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask0;
#[doc = "SRCRESMASK1 (rw) register accessor: Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcresmask1`]
module"]
pub type SRCRESMASK1 = crate::Reg<srcresmask1::SRCRESMASK1_SPEC>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask1;
#[doc = "SRCRESMASK2 (rw) register accessor: Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcresmask2`]
module"]
pub type SRCRESMASK2 = crate::Reg<srcresmask2::SRCRESMASK2_SPEC>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask2;
#[doc = "SRCRESINDEX (rw) register accessor: Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresindex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresindex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcresindex`]
module"]
pub type SRCRESINDEX = crate::Reg<srcresindex::SRCRESINDEX_SPEC>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresindex;
#[doc = "SRCEXTPENDEN0 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcextpenden0`]
module"]
pub type SRCEXTPENDEN0 = crate::Reg<srcextpenden0::SRCEXTPENDEN0_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden0;
#[doc = "SRCEXTPENDEN1 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcextpenden1`]
module"]
pub type SRCEXTPENDEN1 = crate::Reg<srcextpenden1::SRCEXTPENDEN1_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden1;
#[doc = "SRCEXTPENDEN2 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcextpenden2`]
module"]
pub type SRCEXTPENDEN2 = crate::Reg<srcextpenden2::SRCEXTPENDEN2_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden2;
#[doc = "SRCSHORTPENDEN0 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshortpenden0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshortpenden0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshortpenden0`]
module"]
pub type SRCSHORTPENDEN0 = crate::Reg<srcshortpenden0::SRCSHORTPENDEN0_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden0;
#[doc = "SRCSHORTPENDEN1 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshortpenden1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshortpenden1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshortpenden1`]
module"]
pub type SRCSHORTPENDEN1 = crate::Reg<srcshortpenden1::SRCSHORTPENDEN1_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden1;
#[doc = "SRCSHORTPENDEN2 (rw) register accessor: Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshortpenden2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshortpenden2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshortpenden2`]
module"]
pub type SRCSHORTPENDEN2 = crate::Reg<srcshortpenden2::SRCSHORTPENDEN2_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden2;
#[doc = "EXT_ADDR0 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr0`]
module"]
pub type EXT_ADDR0 = crate::Reg<ext_addr0::EXT_ADDR0_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr0;
#[doc = "EXT_ADDR1 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr1`]
module"]
pub type EXT_ADDR1 = crate::Reg<ext_addr1::EXT_ADDR1_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr1;
#[doc = "EXT_ADDR2 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr2`]
module"]
pub type EXT_ADDR2 = crate::Reg<ext_addr2::EXT_ADDR2_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr2;
#[doc = "EXT_ADDR3 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr3`]
module"]
pub type EXT_ADDR3 = crate::Reg<ext_addr3::EXT_ADDR3_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr3;
#[doc = "EXT_ADDR4 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr4`]
module"]
pub type EXT_ADDR4 = crate::Reg<ext_addr4::EXT_ADDR4_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr4;
#[doc = "EXT_ADDR5 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr5`]
module"]
pub type EXT_ADDR5 = crate::Reg<ext_addr5::EXT_ADDR5_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr5;
#[doc = "EXT_ADDR6 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr6`]
module"]
pub type EXT_ADDR6 = crate::Reg<ext_addr6::EXT_ADDR6_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr6;
#[doc = "EXT_ADDR7 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_addr7`]
module"]
pub type EXT_ADDR7 = crate::Reg<ext_addr7::EXT_ADDR7_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr7;
#[doc = "PAN_ID0 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pan_id0`]
module"]
pub type PAN_ID0 = crate::Reg<pan_id0::PAN_ID0_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod pan_id0;
#[doc = "PAN_ID1 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pan_id1`]
module"]
pub type PAN_ID1 = crate::Reg<pan_id1::PAN_ID1_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod pan_id1;
#[doc = "SHORT_ADDR0 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@short_addr0`]
module"]
pub type SHORT_ADDR0 = crate::Reg<short_addr0::SHORT_ADDR0_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod short_addr0;
#[doc = "SHORT_ADDR1 (rw) register accessor: Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@short_addr1`]
module"]
pub type SHORT_ADDR1 = crate::Reg<short_addr1::SHORT_ADDR1_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod short_addr1;
