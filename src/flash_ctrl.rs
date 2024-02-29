#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    fctl: Fctl,
    faddr: Faddr,
    fwdata: Fwdata,
    diecfg0: Diecfg0,
    diecfg1: Diecfg1,
    diecfg2: Diecfg2,
}
impl RegisterBlock {
    #[doc = "0x08 - Flash control This register provides control and monitoring functions for the flash module."]
    #[inline(always)]
    pub const fn fctl(&self) -> &Fctl {
        &self.fctl
    }
    #[doc = "0x0c - Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information."]
    #[inline(always)]
    pub const fn faddr(&self) -> &Faddr {
        &self.faddr
    }
    #[doc = "0x10 - Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR."]
    #[inline(always)]
    pub const fn fwdata(&self) -> &Fwdata {
        &self.fwdata
    }
    #[doc = "0x14 - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
    #[inline(always)]
    pub const fn diecfg0(&self) -> &Diecfg0 {
        &self.diecfg0
    }
    #[doc = "0x18 - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
    #[inline(always)]
    pub const fn diecfg1(&self) -> &Diecfg1 {
        &self.diecfg1
    }
    #[doc = "0x1c - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page."]
    #[inline(always)]
    pub const fn diecfg2(&self) -> &Diecfg2 {
        &self.diecfg2
    }
}
#[doc = "FCTL (rw) register accessor: Flash control This register provides control and monitoring functions for the flash module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl`]
module"]
#[doc(alias = "FCTL")]
pub type Fctl = crate::Reg<fctl::FctlSpec>;
#[doc = "Flash control This register provides control and monitoring functions for the flash module."]
pub mod fctl;
#[doc = "FADDR (rw) register accessor: Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faddr`]
module"]
#[doc(alias = "FADDR")]
pub type Faddr = crate::Reg<faddr::FaddrSpec>;
#[doc = "Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information."]
pub mod faddr;
#[doc = "FWDATA (rw) register accessor: Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwdata`]
module"]
#[doc(alias = "FWDATA")]
pub type Fwdata = crate::Reg<fwdata::FwdataSpec>;
#[doc = "Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR."]
pub mod fwdata;
#[doc = "DIECFG0 (r) register accessor: These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diecfg0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diecfg0`]
module"]
#[doc(alias = "DIECFG0")]
pub type Diecfg0 = crate::Reg<diecfg0::Diecfg0Spec>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
pub mod diecfg0;
#[doc = "DIECFG1 (r) register accessor: These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diecfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diecfg1`]
module"]
#[doc(alias = "DIECFG1")]
pub type Diecfg1 = crate::Reg<diecfg1::Diecfg1Spec>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
pub mod diecfg1;
#[doc = "DIECFG2 (r) register accessor: These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diecfg2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diecfg2`]
module"]
#[doc(alias = "DIECFG2")]
pub type Diecfg2 = crate::Reg<diecfg2::Diecfg2Spec>;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page."]
pub mod diecfg2;
