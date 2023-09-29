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
#[doc = "FCTL (rw) register accessor: Flash control This register provides control and monitoring functions for the flash module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fctl`]
module"]
pub type FCTL = crate::Reg<fctl::FCTL_SPEC>;
#[doc = "Flash control This register provides control and monitoring functions for the flash module."]
pub mod fctl;
#[doc = "FADDR (rw) register accessor: Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`faddr`]
module"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information."]
pub mod faddr;
#[doc = "FWDATA (rw) register accessor: Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fwdata`]
module"]
pub type FWDATA = crate::Reg<fwdata::FWDATA_SPEC>;
#[doc = "Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR."]
pub mod fwdata;
#[doc = "DIECFG0 (r) register accessor: These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diecfg0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diecfg0`]
module"]
pub type DIECFG0 = crate::Reg<diecfg0::DIECFG0_SPEC>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
pub mod diecfg0;
#[doc = "DIECFG1 (r) register accessor: These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diecfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diecfg1`]
module"]
pub type DIECFG1 = crate::Reg<diecfg1::DIECFG1_SPEC>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
pub mod diecfg1;
#[doc = "DIECFG2 (r) register accessor: These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diecfg2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diecfg2`]
module"]
pub type DIECFG2 = crate::Reg<diecfg2::DIECFG2_SPEC>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page."]
pub mod diecfg2;
