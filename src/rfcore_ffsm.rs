#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    pub srcresmask0: SRCRESMASK0,
    #[doc = "0x84 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    pub srcresmask1: SRCRESMASK1,
    #[doc = "0x88 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    pub srcresmask2: SRCRESMASK2,
    #[doc = "0x8c - Source address matching result This register is stored in RAM; the reset value is undefined."]
    pub srcresindex: SRCRESINDEX,
    #[doc = "0x90 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcextpenden0: SRCEXTPENDEN0,
    #[doc = "0x94 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcextpenden1: SRCEXTPENDEN1,
    #[doc = "0x98 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcextpenden2: SRCEXTPENDEN2,
    #[doc = "0x9c - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcshortpenden0: SRCSHORTPENDEN0,
    #[doc = "0xa0 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcshortpenden1: SRCSHORTPENDEN1,
    #[doc = "0xa4 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcshortpenden2: SRCSHORTPENDEN2,
    #[doc = "0xa8 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr0: EXT_ADDR0,
    #[doc = "0xac - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr1: EXT_ADDR1,
    #[doc = "0xb0 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr2: EXT_ADDR2,
    #[doc = "0xb4 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr3: EXT_ADDR3,
    #[doc = "0xb8 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr4: EXT_ADDR4,
    #[doc = "0xbc - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr5: EXT_ADDR5,
    #[doc = "0xc0 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr6: EXT_ADDR6,
    #[doc = "0xc4 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr7: EXT_ADDR7,
    #[doc = "0xc8 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub pan_id0: PAN_ID0,
    #[doc = "0xcc - Local address information This register is stored in RAM; the reset value is undefined."]
    pub pan_id1: PAN_ID1,
    #[doc = "0xd0 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub short_addr0: SHORT_ADDR0,
    #[doc = "0xd4 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub short_addr1: SHORT_ADDR1,
}
#[doc = "SRCRESMASK0 (rw) register accessor: an alias for `Reg<SRCRESMASK0_SPEC>`"]
pub type SRCRESMASK0 = crate::Reg<srcresmask0::SRCRESMASK0_SPEC>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask0;
#[doc = "SRCRESMASK1 (rw) register accessor: an alias for `Reg<SRCRESMASK1_SPEC>`"]
pub type SRCRESMASK1 = crate::Reg<srcresmask1::SRCRESMASK1_SPEC>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask1;
#[doc = "SRCRESMASK2 (rw) register accessor: an alias for `Reg<SRCRESMASK2_SPEC>`"]
pub type SRCRESMASK2 = crate::Reg<srcresmask2::SRCRESMASK2_SPEC>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask2;
#[doc = "SRCRESINDEX (rw) register accessor: an alias for `Reg<SRCRESINDEX_SPEC>`"]
pub type SRCRESINDEX = crate::Reg<srcresindex::SRCRESINDEX_SPEC>;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresindex;
#[doc = "SRCEXTPENDEN0 (rw) register accessor: an alias for `Reg<SRCEXTPENDEN0_SPEC>`"]
pub type SRCEXTPENDEN0 = crate::Reg<srcextpenden0::SRCEXTPENDEN0_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden0;
#[doc = "SRCEXTPENDEN1 (rw) register accessor: an alias for `Reg<SRCEXTPENDEN1_SPEC>`"]
pub type SRCEXTPENDEN1 = crate::Reg<srcextpenden1::SRCEXTPENDEN1_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden1;
#[doc = "SRCEXTPENDEN2 (rw) register accessor: an alias for `Reg<SRCEXTPENDEN2_SPEC>`"]
pub type SRCEXTPENDEN2 = crate::Reg<srcextpenden2::SRCEXTPENDEN2_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden2;
#[doc = "SRCSHORTPENDEN0 (rw) register accessor: an alias for `Reg<SRCSHORTPENDEN0_SPEC>`"]
pub type SRCSHORTPENDEN0 = crate::Reg<srcshortpenden0::SRCSHORTPENDEN0_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden0;
#[doc = "SRCSHORTPENDEN1 (rw) register accessor: an alias for `Reg<SRCSHORTPENDEN1_SPEC>`"]
pub type SRCSHORTPENDEN1 = crate::Reg<srcshortpenden1::SRCSHORTPENDEN1_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden1;
#[doc = "SRCSHORTPENDEN2 (rw) register accessor: an alias for `Reg<SRCSHORTPENDEN2_SPEC>`"]
pub type SRCSHORTPENDEN2 = crate::Reg<srcshortpenden2::SRCSHORTPENDEN2_SPEC>;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden2;
#[doc = "EXT_ADDR0 (rw) register accessor: an alias for `Reg<EXT_ADDR0_SPEC>`"]
pub type EXT_ADDR0 = crate::Reg<ext_addr0::EXT_ADDR0_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr0;
#[doc = "EXT_ADDR1 (rw) register accessor: an alias for `Reg<EXT_ADDR1_SPEC>`"]
pub type EXT_ADDR1 = crate::Reg<ext_addr1::EXT_ADDR1_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr1;
#[doc = "EXT_ADDR2 (rw) register accessor: an alias for `Reg<EXT_ADDR2_SPEC>`"]
pub type EXT_ADDR2 = crate::Reg<ext_addr2::EXT_ADDR2_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr2;
#[doc = "EXT_ADDR3 (rw) register accessor: an alias for `Reg<EXT_ADDR3_SPEC>`"]
pub type EXT_ADDR3 = crate::Reg<ext_addr3::EXT_ADDR3_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr3;
#[doc = "EXT_ADDR4 (rw) register accessor: an alias for `Reg<EXT_ADDR4_SPEC>`"]
pub type EXT_ADDR4 = crate::Reg<ext_addr4::EXT_ADDR4_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr4;
#[doc = "EXT_ADDR5 (rw) register accessor: an alias for `Reg<EXT_ADDR5_SPEC>`"]
pub type EXT_ADDR5 = crate::Reg<ext_addr5::EXT_ADDR5_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr5;
#[doc = "EXT_ADDR6 (rw) register accessor: an alias for `Reg<EXT_ADDR6_SPEC>`"]
pub type EXT_ADDR6 = crate::Reg<ext_addr6::EXT_ADDR6_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr6;
#[doc = "EXT_ADDR7 (rw) register accessor: an alias for `Reg<EXT_ADDR7_SPEC>`"]
pub type EXT_ADDR7 = crate::Reg<ext_addr7::EXT_ADDR7_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr7;
#[doc = "PAN_ID0 (rw) register accessor: an alias for `Reg<PAN_ID0_SPEC>`"]
pub type PAN_ID0 = crate::Reg<pan_id0::PAN_ID0_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod pan_id0;
#[doc = "PAN_ID1 (rw) register accessor: an alias for `Reg<PAN_ID1_SPEC>`"]
pub type PAN_ID1 = crate::Reg<pan_id1::PAN_ID1_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod pan_id1;
#[doc = "SHORT_ADDR0 (rw) register accessor: an alias for `Reg<SHORT_ADDR0_SPEC>`"]
pub type SHORT_ADDR0 = crate::Reg<short_addr0::SHORT_ADDR0_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod short_addr0;
#[doc = "SHORT_ADDR1 (rw) register accessor: an alias for `Reg<SHORT_ADDR1_SPEC>`"]
pub type SHORT_ADDR1 = crate::Reg<short_addr1::SHORT_ADDR1_SPEC>;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod short_addr1;
