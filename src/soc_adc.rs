#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register controls the ADC."]
    pub adccon1: crate::Reg<adccon1::ADCCON1_SPEC>,
    #[doc = "0x04 - This register controls the ADC."]
    pub adccon2: crate::Reg<adccon2::ADCCON2_SPEC>,
    #[doc = "0x08 - This register controls the ADC."]
    pub adccon3: crate::Reg<adccon3::ADCCON3_SPEC>,
    #[doc = "0x0c - This register contains the least-significant part of ADC conversion result."]
    pub adcl: crate::Reg<adcl::ADCL_SPEC>,
    #[doc = "0x10 - This register contains the most-significant part of ADC conversion result."]
    pub adch: crate::Reg<adch::ADCH_SPEC>,
    #[doc = "0x14 - This registers contains random-number-generator data; low byte."]
    pub rndl: crate::Reg<rndl::RNDL_SPEC>,
    #[doc = "0x18 - This register contains random-number-generator data; high byte."]
    pub rndh: crate::Reg<rndh::RNDH_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x24 - Analog comparator control and status register."]
    pub cmpctl: crate::Reg<cmpctl::CMPCTL_SPEC>,
}
#[doc = "ADCCON1 register accessor: an alias for `Reg<ADCCON1_SPEC>`"]
pub type ADCCON1 = crate::Reg<adccon1::ADCCON1_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon1;
#[doc = "ADCCON2 register accessor: an alias for `Reg<ADCCON2_SPEC>`"]
pub type ADCCON2 = crate::Reg<adccon2::ADCCON2_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon2;
#[doc = "ADCCON3 register accessor: an alias for `Reg<ADCCON3_SPEC>`"]
pub type ADCCON3 = crate::Reg<adccon3::ADCCON3_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon3;
#[doc = "ADCL register accessor: an alias for `Reg<ADCL_SPEC>`"]
pub type ADCL = crate::Reg<adcl::ADCL_SPEC>;
#[doc = "This register contains the least-significant part of ADC conversion result."]
pub mod adcl;
#[doc = "ADCH register accessor: an alias for `Reg<ADCH_SPEC>`"]
pub type ADCH = crate::Reg<adch::ADCH_SPEC>;
#[doc = "This register contains the most-significant part of ADC conversion result."]
pub mod adch;
#[doc = "RNDL register accessor: an alias for `Reg<RNDL_SPEC>`"]
pub type RNDL = crate::Reg<rndl::RNDL_SPEC>;
#[doc = "This registers contains random-number-generator data; low byte."]
pub mod rndl;
#[doc = "RNDH register accessor: an alias for `Reg<RNDH_SPEC>`"]
pub type RNDH = crate::Reg<rndh::RNDH_SPEC>;
#[doc = "This register contains random-number-generator data; high byte."]
pub mod rndh;
#[doc = "CMPCTL register accessor: an alias for `Reg<CMPCTL_SPEC>`"]
pub type CMPCTL = crate::Reg<cmpctl::CMPCTL_SPEC>;
#[doc = "Analog comparator control and status register."]
pub mod cmpctl;
