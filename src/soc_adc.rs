#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    adccon1: ADCCON1,
    adccon2: ADCCON2,
    adccon3: ADCCON3,
    adcl: ADCL,
    adch: ADCH,
    rndl: RNDL,
    rndh: RNDH,
    _reserved7: [u8; 0x08],
    cmpctl: CMPCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - This register controls the ADC."]
    #[inline(always)]
    pub const fn adccon1(&self) -> &ADCCON1 {
        &self.adccon1
    }
    #[doc = "0x04 - This register controls the ADC."]
    #[inline(always)]
    pub const fn adccon2(&self) -> &ADCCON2 {
        &self.adccon2
    }
    #[doc = "0x08 - This register controls the ADC."]
    #[inline(always)]
    pub const fn adccon3(&self) -> &ADCCON3 {
        &self.adccon3
    }
    #[doc = "0x0c - This register contains the least-significant part of ADC conversion result."]
    #[inline(always)]
    pub const fn adcl(&self) -> &ADCL {
        &self.adcl
    }
    #[doc = "0x10 - This register contains the most-significant part of ADC conversion result."]
    #[inline(always)]
    pub const fn adch(&self) -> &ADCH {
        &self.adch
    }
    #[doc = "0x14 - This registers contains random-number-generator data; low byte."]
    #[inline(always)]
    pub const fn rndl(&self) -> &RNDL {
        &self.rndl
    }
    #[doc = "0x18 - This register contains random-number-generator data; high byte."]
    #[inline(always)]
    pub const fn rndh(&self) -> &RNDH {
        &self.rndh
    }
    #[doc = "0x24 - Analog comparator control and status register."]
    #[inline(always)]
    pub const fn cmpctl(&self) -> &CMPCTL {
        &self.cmpctl
    }
}
#[doc = "ADCCON1 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adccon1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adccon1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccon1`]
module"]
pub type ADCCON1 = crate::Reg<adccon1::ADCCON1_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon1;
#[doc = "ADCCON2 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adccon2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adccon2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccon2`]
module"]
pub type ADCCON2 = crate::Reg<adccon2::ADCCON2_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon2;
#[doc = "ADCCON3 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adccon3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adccon3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccon3`]
module"]
pub type ADCCON3 = crate::Reg<adccon3::ADCCON3_SPEC>;
#[doc = "This register controls the ADC."]
pub mod adccon3;
#[doc = "ADCL (r) register accessor: This register contains the least-significant part of ADC conversion result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcl`]
module"]
pub type ADCL = crate::Reg<adcl::ADCL_SPEC>;
#[doc = "This register contains the least-significant part of ADC conversion result."]
pub mod adcl;
#[doc = "ADCH (r) register accessor: This register contains the most-significant part of ADC conversion result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adch`]
module"]
pub type ADCH = crate::Reg<adch::ADCH_SPEC>;
#[doc = "This register contains the most-significant part of ADC conversion result."]
pub mod adch;
#[doc = "RNDL (rw) register accessor: This registers contains random-number-generator data; low byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rndl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rndl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rndl`]
module"]
pub type RNDL = crate::Reg<rndl::RNDL_SPEC>;
#[doc = "This registers contains random-number-generator data; low byte."]
pub mod rndl;
#[doc = "RNDH (rw) register accessor: This register contains random-number-generator data; high byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rndh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rndh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rndh`]
module"]
pub type RNDH = crate::Reg<rndh::RNDH_SPEC>;
#[doc = "This register contains random-number-generator data; high byte."]
pub mod rndh;
#[doc = "CMPCTL (rw) register accessor: Analog comparator control and status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpctl`]
module"]
pub type CMPCTL = crate::Reg<cmpctl::CMPCTL_SPEC>;
#[doc = "Analog comparator control and status register."]
pub mod cmpctl;
