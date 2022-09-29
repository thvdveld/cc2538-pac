#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC Timer event configuration"]
    pub mtcspcfg: MTCSPCFG,
    #[doc = "0x04 - MAC Timer control register"]
    pub mtctrl: MTCTRL,
    #[doc = "0x08 - MAC Timer interrupt mask"]
    pub mtirqm: MTIRQM,
    #[doc = "0x0c - MAC Timer interrupt flags"]
    pub mtirqf: MTIRQF,
    #[doc = "0x10 - MAC Timer multiplex select"]
    pub mtmsel: MTMSEL,
    #[doc = "0x14 - MAC Timer multiplexed register 0"]
    pub mtm0: MTM0,
    #[doc = "0x18 - MAC Timer multiplexed register 1"]
    pub mtm1: MTM1,
    #[doc = "0x1c - MAC Timer multiplexed overflow register 2"]
    pub mtmovf2: MTMOVF2,
    #[doc = "0x20 - MAC Timer multiplexed overflow register 1"]
    pub mtmovf1: MTMOVF1,
    #[doc = "0x24 - MAC Timer multiplexed overflow register 0"]
    pub mtmovf0: MTMOVF0,
    #[doc = "0x28 - The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX."]
    pub rfdata: RFDATA,
    #[doc = "0x2c - RF error interrupt flags"]
    pub rferrf: RFERRF,
    #[doc = "0x30 - RF interrupt flags"]
    pub rfirqf1: RFIRQF1,
    #[doc = "0x34 - RF interrupt flags"]
    pub rfirqf0: RFIRQF0,
    #[doc = "0x38 - RF CSMA-CA/strobe processor"]
    pub rfst: RFST,
}
#[doc = "MTCSPCFG (rw) register accessor: an alias for `Reg<MTCSPCFG_SPEC>`"]
pub type MTCSPCFG = crate::Reg<mtcspcfg::MTCSPCFG_SPEC>;
#[doc = "MAC Timer event configuration"]
pub mod mtcspcfg;
#[doc = "MTCTRL (rw) register accessor: an alias for `Reg<MTCTRL_SPEC>`"]
pub type MTCTRL = crate::Reg<mtctrl::MTCTRL_SPEC>;
#[doc = "MAC Timer control register"]
pub mod mtctrl;
#[doc = "MTIRQM (rw) register accessor: an alias for `Reg<MTIRQM_SPEC>`"]
pub type MTIRQM = crate::Reg<mtirqm::MTIRQM_SPEC>;
#[doc = "MAC Timer interrupt mask"]
pub mod mtirqm;
#[doc = "MTIRQF (rw) register accessor: an alias for `Reg<MTIRQF_SPEC>`"]
pub type MTIRQF = crate::Reg<mtirqf::MTIRQF_SPEC>;
#[doc = "MAC Timer interrupt flags"]
pub mod mtirqf;
#[doc = "MTMSEL (rw) register accessor: an alias for `Reg<MTMSEL_SPEC>`"]
pub type MTMSEL = crate::Reg<mtmsel::MTMSEL_SPEC>;
#[doc = "MAC Timer multiplex select"]
pub mod mtmsel;
#[doc = "MTM0 (rw) register accessor: an alias for `Reg<MTM0_SPEC>`"]
pub type MTM0 = crate::Reg<mtm0::MTM0_SPEC>;
#[doc = "MAC Timer multiplexed register 0"]
pub mod mtm0;
#[doc = "MTM1 (rw) register accessor: an alias for `Reg<MTM1_SPEC>`"]
pub type MTM1 = crate::Reg<mtm1::MTM1_SPEC>;
#[doc = "MAC Timer multiplexed register 1"]
pub mod mtm1;
#[doc = "MTMOVF2 (rw) register accessor: an alias for `Reg<MTMOVF2_SPEC>`"]
pub type MTMOVF2 = crate::Reg<mtmovf2::MTMOVF2_SPEC>;
#[doc = "MAC Timer multiplexed overflow register 2"]
pub mod mtmovf2;
#[doc = "MTMOVF1 (rw) register accessor: an alias for `Reg<MTMOVF1_SPEC>`"]
pub type MTMOVF1 = crate::Reg<mtmovf1::MTMOVF1_SPEC>;
#[doc = "MAC Timer multiplexed overflow register 1"]
pub mod mtmovf1;
#[doc = "MTMOVF0 (rw) register accessor: an alias for `Reg<MTMOVF0_SPEC>`"]
pub type MTMOVF0 = crate::Reg<mtmovf0::MTMOVF0_SPEC>;
#[doc = "MAC Timer multiplexed overflow register 0"]
pub mod mtmovf0;
#[doc = "RFDATA (rw) register accessor: an alias for `Reg<RFDATA_SPEC>`"]
pub type RFDATA = crate::Reg<rfdata::RFDATA_SPEC>;
#[doc = "The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX."]
pub mod rfdata;
#[doc = "RFERRF (rw) register accessor: an alias for `Reg<RFERRF_SPEC>`"]
pub type RFERRF = crate::Reg<rferrf::RFERRF_SPEC>;
#[doc = "RF error interrupt flags"]
pub mod rferrf;
#[doc = "RFIRQF1 (rw) register accessor: an alias for `Reg<RFIRQF1_SPEC>`"]
pub type RFIRQF1 = crate::Reg<rfirqf1::RFIRQF1_SPEC>;
#[doc = "RF interrupt flags"]
pub mod rfirqf1;
#[doc = "RFIRQF0 (rw) register accessor: an alias for `Reg<RFIRQF0_SPEC>`"]
pub type RFIRQF0 = crate::Reg<rfirqf0::RFIRQF0_SPEC>;
#[doc = "RF interrupt flags"]
pub mod rfirqf0;
#[doc = "RFST (rw) register accessor: an alias for `Reg<RFST_SPEC>`"]
pub type RFST = crate::Reg<rfst::RFST_SPEC>;
#[doc = "RF CSMA-CA/strobe processor"]
pub mod rfst;
