#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mtcspcfg: MTCSPCFG,
    mtctrl: MTCTRL,
    mtirqm: MTIRQM,
    mtirqf: MTIRQF,
    mtmsel: MTMSEL,
    mtm0: MTM0,
    mtm1: MTM1,
    mtmovf2: MTMOVF2,
    mtmovf1: MTMOVF1,
    mtmovf0: MTMOVF0,
    rfdata: RFDATA,
    rferrf: RFERRF,
    rfirqf1: RFIRQF1,
    rfirqf0: RFIRQF0,
    rfst: RFST,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Timer event configuration"]
    #[inline(always)]
    pub const fn mtcspcfg(&self) -> &MTCSPCFG {
        &self.mtcspcfg
    }
    #[doc = "0x04 - MAC Timer control register"]
    #[inline(always)]
    pub const fn mtctrl(&self) -> &MTCTRL {
        &self.mtctrl
    }
    #[doc = "0x08 - MAC Timer interrupt mask"]
    #[inline(always)]
    pub const fn mtirqm(&self) -> &MTIRQM {
        &self.mtirqm
    }
    #[doc = "0x0c - MAC Timer interrupt flags"]
    #[inline(always)]
    pub const fn mtirqf(&self) -> &MTIRQF {
        &self.mtirqf
    }
    #[doc = "0x10 - MAC Timer multiplex select"]
    #[inline(always)]
    pub const fn mtmsel(&self) -> &MTMSEL {
        &self.mtmsel
    }
    #[doc = "0x14 - MAC Timer multiplexed register 0"]
    #[inline(always)]
    pub const fn mtm0(&self) -> &MTM0 {
        &self.mtm0
    }
    #[doc = "0x18 - MAC Timer multiplexed register 1"]
    #[inline(always)]
    pub const fn mtm1(&self) -> &MTM1 {
        &self.mtm1
    }
    #[doc = "0x1c - MAC Timer multiplexed overflow register 2"]
    #[inline(always)]
    pub const fn mtmovf2(&self) -> &MTMOVF2 {
        &self.mtmovf2
    }
    #[doc = "0x20 - MAC Timer multiplexed overflow register 1"]
    #[inline(always)]
    pub const fn mtmovf1(&self) -> &MTMOVF1 {
        &self.mtmovf1
    }
    #[doc = "0x24 - MAC Timer multiplexed overflow register 0"]
    #[inline(always)]
    pub const fn mtmovf0(&self) -> &MTMOVF0 {
        &self.mtmovf0
    }
    #[doc = "0x28 - The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX."]
    #[inline(always)]
    pub const fn rfdata(&self) -> &RFDATA {
        &self.rfdata
    }
    #[doc = "0x2c - RF error interrupt flags"]
    #[inline(always)]
    pub const fn rferrf(&self) -> &RFERRF {
        &self.rferrf
    }
    #[doc = "0x30 - RF interrupt flags"]
    #[inline(always)]
    pub const fn rfirqf1(&self) -> &RFIRQF1 {
        &self.rfirqf1
    }
    #[doc = "0x34 - RF interrupt flags"]
    #[inline(always)]
    pub const fn rfirqf0(&self) -> &RFIRQF0 {
        &self.rfirqf0
    }
    #[doc = "0x38 - RF CSMA-CA/strobe processor"]
    #[inline(always)]
    pub const fn rfst(&self) -> &RFST {
        &self.rfst
    }
}
#[doc = "MTCSPCFG (rw) register accessor: MAC Timer event configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcspcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcspcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtcspcfg`]
module"]
pub type MTCSPCFG = crate::Reg<mtcspcfg::MTCSPCFG_SPEC>;
#[doc = "MAC Timer event configuration"]
pub mod mtcspcfg;
#[doc = "MTCTRL (rw) register accessor: MAC Timer control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtctrl`]
module"]
pub type MTCTRL = crate::Reg<mtctrl::MTCTRL_SPEC>;
#[doc = "MAC Timer control register"]
pub mod mtctrl;
#[doc = "MTIRQM (rw) register accessor: MAC Timer interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtirqm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtirqm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtirqm`]
module"]
pub type MTIRQM = crate::Reg<mtirqm::MTIRQM_SPEC>;
#[doc = "MAC Timer interrupt mask"]
pub mod mtirqm;
#[doc = "MTIRQF (rw) register accessor: MAC Timer interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtirqf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtirqf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtirqf`]
module"]
pub type MTIRQF = crate::Reg<mtirqf::MTIRQF_SPEC>;
#[doc = "MAC Timer interrupt flags"]
pub mod mtirqf;
#[doc = "MTMSEL (rw) register accessor: MAC Timer multiplex select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtmsel`]
module"]
pub type MTMSEL = crate::Reg<mtmsel::MTMSEL_SPEC>;
#[doc = "MAC Timer multiplex select"]
pub mod mtmsel;
#[doc = "MTM0 (rw) register accessor: MAC Timer multiplexed register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtm0`]
module"]
pub type MTM0 = crate::Reg<mtm0::MTM0_SPEC>;
#[doc = "MAC Timer multiplexed register 0"]
pub mod mtm0;
#[doc = "MTM1 (rw) register accessor: MAC Timer multiplexed register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtm1`]
module"]
pub type MTM1 = crate::Reg<mtm1::MTM1_SPEC>;
#[doc = "MAC Timer multiplexed register 1"]
pub mod mtm1;
#[doc = "MTMOVF2 (rw) register accessor: MAC Timer multiplexed overflow register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmovf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmovf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtmovf2`]
module"]
pub type MTMOVF2 = crate::Reg<mtmovf2::MTMOVF2_SPEC>;
#[doc = "MAC Timer multiplexed overflow register 2"]
pub mod mtmovf2;
#[doc = "MTMOVF1 (rw) register accessor: MAC Timer multiplexed overflow register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmovf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmovf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtmovf1`]
module"]
pub type MTMOVF1 = crate::Reg<mtmovf1::MTMOVF1_SPEC>;
#[doc = "MAC Timer multiplexed overflow register 1"]
pub mod mtmovf1;
#[doc = "MTMOVF0 (rw) register accessor: MAC Timer multiplexed overflow register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmovf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmovf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtmovf0`]
module"]
pub type MTMOVF0 = crate::Reg<mtmovf0::MTMOVF0_SPEC>;
#[doc = "MAC Timer multiplexed overflow register 0"]
pub mod mtmovf0;
#[doc = "RFDATA (rw) register accessor: The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdata`]
module"]
pub type RFDATA = crate::Reg<rfdata::RFDATA_SPEC>;
#[doc = "The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX."]
pub mod rfdata;
#[doc = "RFERRF (rw) register accessor: RF error interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rferrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rferrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rferrf`]
module"]
pub type RFERRF = crate::Reg<rferrf::RFERRF_SPEC>;
#[doc = "RF error interrupt flags"]
pub mod rferrf;
#[doc = "RFIRQF1 (rw) register accessor: RF interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfirqf1`]
module"]
pub type RFIRQF1 = crate::Reg<rfirqf1::RFIRQF1_SPEC>;
#[doc = "RF interrupt flags"]
pub mod rfirqf1;
#[doc = "RFIRQF0 (rw) register accessor: RF interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfirqf0`]
module"]
pub type RFIRQF0 = crate::Reg<rfirqf0::RFIRQF0_SPEC>;
#[doc = "RF interrupt flags"]
pub mod rfirqf0;
#[doc = "RFST (rw) register accessor: RF CSMA-CA/strobe processor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfst`]
module"]
pub type RFST = crate::Reg<rfst::RFST_SPEC>;
#[doc = "RF CSMA-CA/strobe processor"]
pub mod rfst;
