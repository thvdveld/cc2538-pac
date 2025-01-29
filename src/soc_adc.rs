#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adccon1: Adccon1,
    adccon2: Adccon2,
    adccon3: Adccon3,
    adcl: Adcl,
    adch: Adch,
    rndl: Rndl,
    rndh: Rndh,
    _reserved7: [u8; 0x08],
    cmpctl: Cmpctl,
}
impl RegisterBlock {
    #[doc = "0x00 - This register controls the ADC."]
    #[inline(always)]
    pub const fn adccon1(&self) -> &Adccon1 {
        &self.adccon1
    }
    #[doc = "0x04 - This register controls the ADC."]
    #[inline(always)]
    pub const fn adccon2(&self) -> &Adccon2 {
        &self.adccon2
    }
    #[doc = "0x08 - This register controls the ADC."]
    #[inline(always)]
    pub const fn adccon3(&self) -> &Adccon3 {
        &self.adccon3
    }
    #[doc = "0x0c - This register contains the least-significant part of ADC conversion result."]
    #[inline(always)]
    pub const fn adcl(&self) -> &Adcl {
        &self.adcl
    }
    #[doc = "0x10 - This register contains the most-significant part of ADC conversion result."]
    #[inline(always)]
    pub const fn adch(&self) -> &Adch {
        &self.adch
    }
    #[doc = "0x14 - This registers contains random-number-generator data; low byte."]
    #[inline(always)]
    pub const fn rndl(&self) -> &Rndl {
        &self.rndl
    }
    #[doc = "0x18 - This register contains random-number-generator data; high byte."]
    #[inline(always)]
    pub const fn rndh(&self) -> &Rndh {
        &self.rndh
    }
    #[doc = "0x24 - Analog comparator control and status register."]
    #[inline(always)]
    pub const fn cmpctl(&self) -> &Cmpctl {
        &self.cmpctl
    }
}
#[doc = "ADCCON1 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::Reg::read) this register and get [`adccon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccon1`]
module"]
#[doc(alias = "ADCCON1")]
pub type Adccon1 = crate::Reg<adccon1::Adccon1Spec>;
#[doc = "This register controls the ADC."]
pub mod adccon1;
#[doc = "ADCCON2 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::Reg::read) this register and get [`adccon2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccon2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccon2`]
module"]
#[doc(alias = "ADCCON2")]
pub type Adccon2 = crate::Reg<adccon2::Adccon2Spec>;
#[doc = "This register controls the ADC."]
pub mod adccon2;
#[doc = "ADCCON3 (rw) register accessor: This register controls the ADC.\n\nYou can [`read`](crate::Reg::read) this register and get [`adccon3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccon3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccon3`]
module"]
#[doc(alias = "ADCCON3")]
pub type Adccon3 = crate::Reg<adccon3::Adccon3Spec>;
#[doc = "This register controls the ADC."]
pub mod adccon3;
#[doc = "ADCL (r) register accessor: This register contains the least-significant part of ADC conversion result.\n\nYou can [`read`](crate::Reg::read) this register and get [`adcl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcl`]
module"]
#[doc(alias = "ADCL")]
pub type Adcl = crate::Reg<adcl::AdclSpec>;
#[doc = "This register contains the least-significant part of ADC conversion result."]
pub mod adcl;
#[doc = "ADCH (r) register accessor: This register contains the most-significant part of ADC conversion result.\n\nYou can [`read`](crate::Reg::read) this register and get [`adch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adch`]
module"]
#[doc(alias = "ADCH")]
pub type Adch = crate::Reg<adch::AdchSpec>;
#[doc = "This register contains the most-significant part of ADC conversion result."]
pub mod adch;
#[doc = "RNDL (rw) register accessor: This registers contains random-number-generator data; low byte.\n\nYou can [`read`](crate::Reg::read) this register and get [`rndl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rndl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rndl`]
module"]
#[doc(alias = "RNDL")]
pub type Rndl = crate::Reg<rndl::RndlSpec>;
#[doc = "This registers contains random-number-generator data; low byte."]
pub mod rndl;
#[doc = "RNDH (rw) register accessor: This register contains random-number-generator data; high byte.\n\nYou can [`read`](crate::Reg::read) this register and get [`rndh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rndh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rndh`]
module"]
#[doc(alias = "RNDH")]
pub type Rndh = crate::Reg<rndh::RndhSpec>;
#[doc = "This register contains random-number-generator data; high byte."]
pub mod rndh;
#[doc = "CMPCTL (rw) register accessor: Analog comparator control and status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpctl`]
module"]
#[doc(alias = "CMPCTL")]
pub type Cmpctl = crate::Reg<cmpctl::CmpctlSpec>;
#[doc = "Analog comparator control and status register."]
pub mod cmpctl;
