#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register controls the ADC."]
    pub adccon1: ADCCON1,
    #[doc = "0x04 - This register controls the ADC."]
    pub adccon2: ADCCON2,
    #[doc = "0x08 - This register controls the ADC."]
    pub adccon3: ADCCON3,
    #[doc = "0x0c - This register contains the least-significant part of ADC conversion result."]
    pub adcl: ADCL,
    #[doc = "0x10 - This register contains the most-significant part of ADC conversion result."]
    pub adch: ADCH,
    #[doc = "0x14 - This registers contains random-number-generator data; low byte."]
    pub rndl: RNDL,
    #[doc = "0x18 - This register contains random-number-generator data; high byte."]
    pub rndh: RNDH,
    _reserved7: [u8; 0x08],
    #[doc = "0x24 - Analog comparator control and status register."]
    pub cmpctl: CMPCTL,
}
#[doc = "ADCCON1 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adccon1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adccon1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adccon1`]
module"]
pub type ADCCON1 = crate::Reg<adccon1::ADCCON1_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon1;
#[doc = "ADCCON2 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adccon2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adccon2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adccon2`]
module"]
pub type ADCCON2 = crate::Reg<adccon2::ADCCON2_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon2;
#[doc = "ADCCON3 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adccon3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adccon3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adccon3`]
module"]
pub type ADCCON3 = crate::Reg<adccon3::ADCCON3_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon3;
#[doc = "ADCL (r) register accessor: This register contains the least-significant part of ADC conversion result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adcl`]
module"]
pub type ADCL = crate::Reg<adcl::ADCL_SPEC>;
#[doc = "This register contains the least-significant part of ADC conversion result."]
pub mod adcl;
#[doc = "ADCH (r) register accessor: This register contains the most-significant part of ADC conversion result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adch`]
module"]
pub type ADCH = crate::Reg<adch::ADCH_SPEC>;
#[doc = "This register contains the most-significant part of ADC conversion result."]
pub mod adch;
#[doc = "RNDL (rw) register accessor: This registers contains random-number-generator data; low byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rndl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rndl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rndl`]
module"]
pub type RNDL = crate::Reg<rndl::RNDL_SPEC>;
#[doc = "This registers contains random-number-generator data; low byte."]
pub mod rndl;
#[doc = "RNDH (rw) register accessor: This register contains random-number-generator data; high byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rndh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rndh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rndh`]
module"]
pub type RNDH = crate::Reg<rndh::RNDH_SPEC>;
#[doc = "This register contains random-number-generator data; high byte."]
pub mod rndh;
#[doc = "CMPCTL (rw) register accessor: Analog comparator control and status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmpctl`]
module"]
pub type CMPCTL = crate::Reg<cmpctl::CMPCTL_SPEC>;
#[doc = "Analog comparator control and status register."]
pub mod cmpctl;
