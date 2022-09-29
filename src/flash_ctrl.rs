#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Flash control This register provides control and monitoring functions for the flash module."]
    pub fctl: FCTL,
    #[doc = "0x0c - Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information."]
    pub faddr: FADDR,
    #[doc = "0x10 - Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR."]
    pub fwdata: FWDATA,
    #[doc = "0x14 - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
    pub diecfg0: DIECFG0,
    #[doc = "0x18 - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
    pub diecfg1: DIECFG1,
    #[doc = "0x1c - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page."]
    pub diecfg2: DIECFG2,
}
#[doc = "FCTL (rw) register accessor: an alias for `Reg<FCTL_SPEC>`"]
pub type FCTL = crate::Reg<fctl::FCTL_SPEC>;
#[doc = "Flash control This register provides control and monitoring functions for the flash module."]
pub mod fctl;
#[doc = "FADDR (rw) register accessor: an alias for `Reg<FADDR_SPEC>`"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information."]
pub mod faddr;
#[doc = "FWDATA (rw) register accessor: an alias for `Reg<FWDATA_SPEC>`"]
pub type FWDATA = crate::Reg<fwdata::FWDATA_SPEC>;
#[doc = "Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR."]
pub mod fwdata;
#[doc = "DIECFG0 (r) register accessor: an alias for `Reg<DIECFG0_SPEC>`"]
pub type DIECFG0 = crate::Reg<diecfg0::DIECFG0_SPEC>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
pub mod diecfg0;
#[doc = "DIECFG1 (r) register accessor: an alias for `Reg<DIECFG1_SPEC>`"]
pub type DIECFG1 = crate::Reg<diecfg1::DIECFG1_SPEC>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
pub mod diecfg1;
#[doc = "DIECFG2 (r) register accessor: an alias for `Reg<DIECFG2_SPEC>`"]
pub type DIECFG2 = crate::Reg<diecfg2::DIECFG2_SPEC>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page."]
pub mod diecfg2;
