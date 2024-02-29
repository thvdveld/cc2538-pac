#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mtcspcfg: Mtcspcfg,
    mtctrl: Mtctrl,
    mtirqm: Mtirqm,
    mtirqf: Mtirqf,
    mtmsel: Mtmsel,
    mtm0: Mtm0,
    mtm1: Mtm1,
    mtmovf2: Mtmovf2,
    mtmovf1: Mtmovf1,
    mtmovf0: Mtmovf0,
    rfdata: Rfdata,
    rferrf: Rferrf,
    rfirqf1: Rfirqf1,
    rfirqf0: Rfirqf0,
    rfst: Rfst,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Timer event configuration"]
    #[inline(always)]
    pub const fn mtcspcfg(&self) -> &Mtcspcfg {
        &self.mtcspcfg
    }
    #[doc = "0x04 - MAC Timer control register"]
    #[inline(always)]
    pub const fn mtctrl(&self) -> &Mtctrl {
        &self.mtctrl
    }
    #[doc = "0x08 - MAC Timer interrupt mask"]
    #[inline(always)]
    pub const fn mtirqm(&self) -> &Mtirqm {
        &self.mtirqm
    }
    #[doc = "0x0c - MAC Timer interrupt flags"]
    #[inline(always)]
    pub const fn mtirqf(&self) -> &Mtirqf {
        &self.mtirqf
    }
    #[doc = "0x10 - MAC Timer multiplex select"]
    #[inline(always)]
    pub const fn mtmsel(&self) -> &Mtmsel {
        &self.mtmsel
    }
    #[doc = "0x14 - MAC Timer multiplexed register 0"]
    #[inline(always)]
    pub const fn mtm0(&self) -> &Mtm0 {
        &self.mtm0
    }
    #[doc = "0x18 - MAC Timer multiplexed register 1"]
    #[inline(always)]
    pub const fn mtm1(&self) -> &Mtm1 {
        &self.mtm1
    }
    #[doc = "0x1c - MAC Timer multiplexed overflow register 2"]
    #[inline(always)]
    pub const fn mtmovf2(&self) -> &Mtmovf2 {
        &self.mtmovf2
    }
    #[doc = "0x20 - MAC Timer multiplexed overflow register 1"]
    #[inline(always)]
    pub const fn mtmovf1(&self) -> &Mtmovf1 {
        &self.mtmovf1
    }
    #[doc = "0x24 - MAC Timer multiplexed overflow register 0"]
    #[inline(always)]
    pub const fn mtmovf0(&self) -> &Mtmovf0 {
        &self.mtmovf0
    }
    #[doc = "0x28 - The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX."]
    #[inline(always)]
    pub const fn rfdata(&self) -> &Rfdata {
        &self.rfdata
    }
    #[doc = "0x2c - RF error interrupt flags"]
    #[inline(always)]
    pub const fn rferrf(&self) -> &Rferrf {
        &self.rferrf
    }
    #[doc = "0x30 - RF interrupt flags"]
    #[inline(always)]
    pub const fn rfirqf1(&self) -> &Rfirqf1 {
        &self.rfirqf1
    }
    #[doc = "0x34 - RF interrupt flags"]
    #[inline(always)]
    pub const fn rfirqf0(&self) -> &Rfirqf0 {
        &self.rfirqf0
    }
    #[doc = "0x38 - RF CSMA-CA/strobe processor"]
    #[inline(always)]
    pub const fn rfst(&self) -> &Rfst {
        &self.rfst
    }
}
#[doc = "MTCSPCFG (rw) register accessor: MAC Timer event configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcspcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcspcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtcspcfg`]
module"]
#[doc(alias = "MTCSPCFG")]
pub type Mtcspcfg = crate::Reg<mtcspcfg::MtcspcfgSpec>;
#[doc = "MAC Timer event configuration"]
pub mod mtcspcfg;
#[doc = "MTCTRL (rw) register accessor: MAC Timer control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtctrl`]
module"]
#[doc(alias = "MTCTRL")]
pub type Mtctrl = crate::Reg<mtctrl::MtctrlSpec>;
#[doc = "MAC Timer control register"]
pub mod mtctrl;
#[doc = "MTIRQM (rw) register accessor: MAC Timer interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtirqm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtirqm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtirqm`]
module"]
#[doc(alias = "MTIRQM")]
pub type Mtirqm = crate::Reg<mtirqm::MtirqmSpec>;
#[doc = "MAC Timer interrupt mask"]
pub mod mtirqm;
#[doc = "MTIRQF (rw) register accessor: MAC Timer interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtirqf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtirqf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtirqf`]
module"]
#[doc(alias = "MTIRQF")]
pub type Mtirqf = crate::Reg<mtirqf::MtirqfSpec>;
#[doc = "MAC Timer interrupt flags"]
pub mod mtirqf;
#[doc = "MTMSEL (rw) register accessor: MAC Timer multiplex select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtmsel`]
module"]
#[doc(alias = "MTMSEL")]
pub type Mtmsel = crate::Reg<mtmsel::MtmselSpec>;
#[doc = "MAC Timer multiplex select"]
pub mod mtmsel;
#[doc = "MTM0 (rw) register accessor: MAC Timer multiplexed register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtm0`]
module"]
#[doc(alias = "MTM0")]
pub type Mtm0 = crate::Reg<mtm0::Mtm0Spec>;
#[doc = "MAC Timer multiplexed register 0"]
pub mod mtm0;
#[doc = "MTM1 (rw) register accessor: MAC Timer multiplexed register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtm1`]
module"]
#[doc(alias = "MTM1")]
pub type Mtm1 = crate::Reg<mtm1::Mtm1Spec>;
#[doc = "MAC Timer multiplexed register 1"]
pub mod mtm1;
#[doc = "MTMOVF2 (rw) register accessor: MAC Timer multiplexed overflow register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmovf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmovf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtmovf2`]
module"]
#[doc(alias = "MTMOVF2")]
pub type Mtmovf2 = crate::Reg<mtmovf2::Mtmovf2Spec>;
#[doc = "MAC Timer multiplexed overflow register 2"]
pub mod mtmovf2;
#[doc = "MTMOVF1 (rw) register accessor: MAC Timer multiplexed overflow register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmovf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmovf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtmovf1`]
module"]
#[doc(alias = "MTMOVF1")]
pub type Mtmovf1 = crate::Reg<mtmovf1::Mtmovf1Spec>;
#[doc = "MAC Timer multiplexed overflow register 1"]
pub mod mtmovf1;
#[doc = "MTMOVF0 (rw) register accessor: MAC Timer multiplexed overflow register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmovf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmovf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtmovf0`]
module"]
#[doc(alias = "MTMOVF0")]
pub type Mtmovf0 = crate::Reg<mtmovf0::Mtmovf0Spec>;
#[doc = "MAC Timer multiplexed overflow register 0"]
pub mod mtmovf0;
#[doc = "RFDATA (rw) register accessor: The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdata`]
module"]
#[doc(alias = "RFDATA")]
pub type Rfdata = crate::Reg<rfdata::RfdataSpec>;
#[doc = "The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX."]
pub mod rfdata;
#[doc = "RFERRF (rw) register accessor: RF error interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rferrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rferrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rferrf`]
module"]
#[doc(alias = "RFERRF")]
pub type Rferrf = crate::Reg<rferrf::RferrfSpec>;
#[doc = "RF error interrupt flags"]
pub mod rferrf;
#[doc = "RFIRQF1 (rw) register accessor: RF interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfirqf1`]
module"]
#[doc(alias = "RFIRQF1")]
pub type Rfirqf1 = crate::Reg<rfirqf1::Rfirqf1Spec>;
#[doc = "RF interrupt flags"]
pub mod rfirqf1;
#[doc = "RFIRQF0 (rw) register accessor: RF interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfirqf0`]
module"]
#[doc(alias = "RFIRQF0")]
pub type Rfirqf0 = crate::Reg<rfirqf0::Rfirqf0Spec>;
#[doc = "RF interrupt flags"]
pub mod rfirqf0;
#[doc = "RFST (rw) register accessor: RF CSMA-CA/strobe processor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfst`]
module"]
#[doc(alias = "RFST")]
pub type Rfst = crate::Reg<rfst::RfstSpec>;
#[doc = "RF CSMA-CA/strobe processor"]
pub mod rfst;
