#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
    pub frmfilt0: FRMFILT0,
    #[doc = "0x04 - The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
    pub frmfilt1: FRMFILT1,
    #[doc = "0x08 - Source address matching and pending bits"]
    pub srcmatch: SRCMATCH,
    #[doc = "0x0c - Short address matching"]
    pub srcshorten0: SRCSHORTEN0,
    #[doc = "0x10 - Short address matching"]
    pub srcshorten1: SRCSHORTEN1,
    #[doc = "0x14 - Short address matching"]
    pub srcshorten2: SRCSHORTEN2,
    #[doc = "0x18 - Extended address matching"]
    pub srcexten0: SRCEXTEN0,
    #[doc = "0x1c - Extended address matching"]
    pub srcexten1: SRCEXTEN1,
    #[doc = "0x20 - Extended address matching"]
    pub srcexten2: SRCEXTEN2,
    #[doc = "0x24 - Frame handling"]
    pub frmctrl0: FRMCTRL0,
    #[doc = "0x28 - Frame handling"]
    pub frmctrl1: FRMCTRL1,
    #[doc = "0x2c - RX enabling"]
    pub rxenable: RXENABLE,
    #[doc = "0x30 - RX enabling"]
    pub rxmaskset: RXMASKSET,
    #[doc = "0x34 - RX disabling"]
    pub rxmaskclr: RXMASKCLR,
    #[doc = "0x38 - Crystal oscillator frequency tuning"]
    pub freqtune: FREQTUNE,
    #[doc = "0x3c - Controls the RF frequency"]
    pub freqctrl: FREQCTRL,
    #[doc = "0x40 - Controls the output power"]
    pub txpower: TXPOWER,
    #[doc = "0x44 - Controls the TX settings"]
    pub txctrl: TXCTRL,
    #[doc = "0x48 - Radio status register"]
    pub fsmstat0: FSMSTAT0,
    #[doc = "0x4c - Radio status register"]
    pub fsmstat1: FSMSTAT1,
    #[doc = "0x50 - FIFOP threshold"]
    pub fifopctrl: FIFOPCTRL,
    #[doc = "0x54 - FSM options"]
    pub fsmctrl: FSMCTRL,
    #[doc = "0x58 - CCA threshold"]
    pub ccactrl0: CCACTRL0,
    #[doc = "0x5c - Other CCA Options"]
    pub ccactrl1: CCACTRL1,
    #[doc = "0x60 - RSSI status register"]
    pub rssi: RSSI,
    #[doc = "0x64 - RSSI valid status register"]
    pub rssistat: RSSISTAT,
    #[doc = "0x68 - First byte in RX FIFO"]
    pub rxfirst: RXFIRST,
    #[doc = "0x6c - Number of bytes in RX FIFO"]
    pub rxfifocnt: RXFIFOCNT,
    #[doc = "0x70 - Number of bytes in TX FIFO"]
    pub txfifocnt: TXFIFOCNT,
    #[doc = "0x74 - RX FIFO pointer"]
    pub rxfirst_ptr: RXFIRST_PTR,
    #[doc = "0x78 - RX FIFO pointer"]
    pub rxlast_ptr: RXLAST_PTR,
    #[doc = "0x7c - RX FIFO pointer"]
    pub rxp1_ptr: RXP1_PTR,
    _reserved32: [u8; 0x04],
    #[doc = "0x84 - TX FIFO pointer"]
    pub txfirst_ptr: TXFIRST_PTR,
    #[doc = "0x88 - TX FIFO pointer"]
    pub txlast_ptr: TXLAST_PTR,
    #[doc = "0x8c - RF interrupt masks"]
    pub rfirqm0: RFIRQM0,
    #[doc = "0x90 - RF interrupt masks"]
    pub rfirqm1: RFIRQM1,
    #[doc = "0x94 - RF error interrupt mask"]
    pub rferrm: RFERRM,
    _reserved37: [u8; 0x04],
    #[doc = "0x9c - Random data"]
    pub rfrnd: RFRND,
    #[doc = "0xa0 - Controls modem"]
    pub mdmctrl0: MDMCTRL0,
    #[doc = "0xa4 - Controls modem"]
    pub mdmctrl1: MDMCTRL1,
    #[doc = "0xa8 - Estimated RF frequency offset"]
    pub freqest: FREQEST,
    #[doc = "0xac - Tune receive section"]
    pub rxctrl: RXCTRL,
    #[doc = "0xb0 - Tune frequency synthesizer"]
    pub fsctrl: FSCTRL,
    #[doc = "0xb4 - Tune frequency calibration"]
    pub fscal0: FSCAL0,
    #[doc = "0xb8 - Tune frequency calibration"]
    pub fscal1: FSCAL1,
    #[doc = "0xbc - Tune frequency calibration"]
    pub fscal2: FSCAL2,
    #[doc = "0xc0 - Tune frequency calibration"]
    pub fscal3: FSCAL3,
    #[doc = "0xc4 - AGC dynamic range control"]
    pub agcctrl0: AGCCTRL0,
    #[doc = "0xc8 - AGC reference level"]
    pub agcctrl1: AGCCTRL1,
    #[doc = "0xcc - AGC gain override"]
    pub agcctrl2: AGCCTRL2,
    #[doc = "0xd0 - AGC control"]
    pub agcctrl3: AGCCTRL3,
    #[doc = "0xd4 - ADC tuning"]
    pub adctest0: ADCTEST0,
    #[doc = "0xd8 - ADC tuning"]
    pub adctest1: ADCTEST1,
    #[doc = "0xdc - ADC tuning"]
    pub adctest2: ADCTEST2,
    #[doc = "0xe0 - Test register for modem"]
    pub mdmtest0: MDMTEST0,
    #[doc = "0xe4 - Test Register for Modem"]
    pub mdmtest1: MDMTEST1,
    #[doc = "0xe8 - DAC override value"]
    pub dactest0: DACTEST0,
    #[doc = "0xec - DAC override value"]
    pub dactest1: DACTEST1,
    #[doc = "0xf0 - DAC test setting"]
    pub dactest2: DACTEST2,
    #[doc = "0xf4 - Analog test control"]
    pub atest: ATEST,
    #[doc = "0xf8 - Override power-down register"]
    pub ptest0: PTEST0,
    #[doc = "0xfc - Override power-down register"]
    pub ptest1: PTEST1,
    #[doc = "0x100 - CSP program"]
    pub cspprog_0: CSPPROG_0,
    #[doc = "0x104 - CSP program"]
    pub cspprog_1: CSPPROG_1,
    #[doc = "0x108 - CSP program"]
    pub cspprog_2: CSPPROG_2,
    #[doc = "0x10c - CSP program"]
    pub cspprog_3: CSPPROG_3,
    #[doc = "0x110 - CSP program"]
    pub cspprog_4: CSPPROG_4,
    #[doc = "0x114 - CSP program"]
    pub cspprog_5: CSPPROG_5,
    #[doc = "0x118 - CSP program"]
    pub cspprog_6: CSPPROG_6,
    #[doc = "0x11c - CSP program"]
    pub cspprog_7: CSPPROG_7,
    #[doc = "0x120 - CSP program"]
    pub cspprog_8: CSPPROG_8,
    #[doc = "0x124 - CSP program"]
    pub cspprog_9: CSPPROG_9,
    #[doc = "0x128 - CSP program"]
    pub cspprog_10: CSPPROG_10,
    #[doc = "0x12c - CSP program"]
    pub cspprog_11: CSPPROG_11,
    #[doc = "0x130 - CSP program"]
    pub cspprog_12: CSPPROG_12,
    #[doc = "0x134 - CSP program"]
    pub cspprog_13: CSPPROG_13,
    #[doc = "0x138 - CSP program"]
    pub cspprog_14: CSPPROG_14,
    #[doc = "0x13c - CSP program"]
    pub cspprog_15: CSPPROG_15,
    #[doc = "0x140 - CSP program"]
    pub cspprog_16: CSPPROG_16,
    #[doc = "0x144 - CSP program"]
    pub cspprog_17: CSPPROG_17,
    #[doc = "0x148 - CSP program"]
    pub cspprog_18: CSPPROG_18,
    #[doc = "0x14c - CSP program"]
    pub cspprog_19: CSPPROG_19,
    #[doc = "0x150 - CSP program"]
    pub cspprog_20: CSPPROG_20,
    #[doc = "0x154 - CSP program"]
    pub cspprog_21: CSPPROG_21,
    #[doc = "0x158 - CSP program"]
    pub cspprog_22: CSPPROG_22,
    #[doc = "0x15c - CSP program"]
    pub cspprog_23: CSPPROG_23,
    _reserved86: [u8; 0x20],
    #[doc = "0x180 - CSP control bit"]
    pub cspctrl: CSPCTRL,
    #[doc = "0x184 - CSP status register"]
    pub cspstat: CSPSTAT,
    #[doc = "0x188 - CSP X data register"]
    pub cspx: CSPX,
    #[doc = "0x18c - CSP Y data register"]
    pub cspy: CSPY,
    #[doc = "0x190 - CSP Z data register"]
    pub cspz: CSPZ,
    #[doc = "0x194 - CSP T data register"]
    pub cspt: CSPT,
    _reserved92: [u8; 0x14],
    #[doc = "0x1ac - RF observation mux control"]
    pub rfc_obs_ctrl0: RFC_OBS_CTRL0,
    #[doc = "0x1b0 - RF observation mux control"]
    pub rfc_obs_ctrl1: RFC_OBS_CTRL1,
    #[doc = "0x1b4 - RF observation mux control"]
    pub rfc_obs_ctrl2: RFC_OBS_CTRL2,
    _reserved95: [u8; 0x30],
    #[doc = "0x1e8 - TX filter configuration"]
    pub txfiltcfg: TXFILTCFG,
}
#[doc = "FRMFILT0 (rw) register accessor: an alias for `Reg<FRMFILT0_SPEC>`"]
pub type FRMFILT0 = crate::Reg<frmfilt0::FRMFILT0_SPEC>;
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
pub mod frmfilt0;
#[doc = "FRMFILT1 (rw) register accessor: an alias for `Reg<FRMFILT1_SPEC>`"]
pub type FRMFILT1 = crate::Reg<frmfilt1::FRMFILT1_SPEC>;
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
pub mod frmfilt1;
#[doc = "SRCMATCH (rw) register accessor: an alias for `Reg<SRCMATCH_SPEC>`"]
pub type SRCMATCH = crate::Reg<srcmatch::SRCMATCH_SPEC>;
#[doc = "Source address matching and pending bits"]
pub mod srcmatch;
#[doc = "SRCSHORTEN0 (rw) register accessor: an alias for `Reg<SRCSHORTEN0_SPEC>`"]
pub type SRCSHORTEN0 = crate::Reg<srcshorten0::SRCSHORTEN0_SPEC>;
#[doc = "Short address matching"]
pub mod srcshorten0;
#[doc = "SRCSHORTEN1 (rw) register accessor: an alias for `Reg<SRCSHORTEN1_SPEC>`"]
pub type SRCSHORTEN1 = crate::Reg<srcshorten1::SRCSHORTEN1_SPEC>;
#[doc = "Short address matching"]
pub mod srcshorten1;
#[doc = "SRCSHORTEN2 (rw) register accessor: an alias for `Reg<SRCSHORTEN2_SPEC>`"]
pub type SRCSHORTEN2 = crate::Reg<srcshorten2::SRCSHORTEN2_SPEC>;
#[doc = "Short address matching"]
pub mod srcshorten2;
#[doc = "SRCEXTEN0 (rw) register accessor: an alias for `Reg<SRCEXTEN0_SPEC>`"]
pub type SRCEXTEN0 = crate::Reg<srcexten0::SRCEXTEN0_SPEC>;
#[doc = "Extended address matching"]
pub mod srcexten0;
#[doc = "SRCEXTEN1 (rw) register accessor: an alias for `Reg<SRCEXTEN1_SPEC>`"]
pub type SRCEXTEN1 = crate::Reg<srcexten1::SRCEXTEN1_SPEC>;
#[doc = "Extended address matching"]
pub mod srcexten1;
#[doc = "SRCEXTEN2 (rw) register accessor: an alias for `Reg<SRCEXTEN2_SPEC>`"]
pub type SRCEXTEN2 = crate::Reg<srcexten2::SRCEXTEN2_SPEC>;
#[doc = "Extended address matching"]
pub mod srcexten2;
#[doc = "FRMCTRL0 (rw) register accessor: an alias for `Reg<FRMCTRL0_SPEC>`"]
pub type FRMCTRL0 = crate::Reg<frmctrl0::FRMCTRL0_SPEC>;
#[doc = "Frame handling"]
pub mod frmctrl0;
#[doc = "FRMCTRL1 (rw) register accessor: an alias for `Reg<FRMCTRL1_SPEC>`"]
pub type FRMCTRL1 = crate::Reg<frmctrl1::FRMCTRL1_SPEC>;
#[doc = "Frame handling"]
pub mod frmctrl1;
#[doc = "RXENABLE (r) register accessor: an alias for `Reg<RXENABLE_SPEC>`"]
pub type RXENABLE = crate::Reg<rxenable::RXENABLE_SPEC>;
#[doc = "RX enabling"]
pub mod rxenable;
#[doc = "RXMASKSET (rw) register accessor: an alias for `Reg<RXMASKSET_SPEC>`"]
pub type RXMASKSET = crate::Reg<rxmaskset::RXMASKSET_SPEC>;
#[doc = "RX enabling"]
pub mod rxmaskset;
#[doc = "RXMASKCLR (rw) register accessor: an alias for `Reg<RXMASKCLR_SPEC>`"]
pub type RXMASKCLR = crate::Reg<rxmaskclr::RXMASKCLR_SPEC>;
#[doc = "RX disabling"]
pub mod rxmaskclr;
#[doc = "FREQTUNE (rw) register accessor: an alias for `Reg<FREQTUNE_SPEC>`"]
pub type FREQTUNE = crate::Reg<freqtune::FREQTUNE_SPEC>;
#[doc = "Crystal oscillator frequency tuning"]
pub mod freqtune;
#[doc = "FREQCTRL (rw) register accessor: an alias for `Reg<FREQCTRL_SPEC>`"]
pub type FREQCTRL = crate::Reg<freqctrl::FREQCTRL_SPEC>;
#[doc = "Controls the RF frequency"]
pub mod freqctrl;
#[doc = "TXPOWER (rw) register accessor: an alias for `Reg<TXPOWER_SPEC>`"]
pub type TXPOWER = crate::Reg<txpower::TXPOWER_SPEC>;
#[doc = "Controls the output power"]
pub mod txpower;
#[doc = "TXCTRL (rw) register accessor: an alias for `Reg<TXCTRL_SPEC>`"]
pub type TXCTRL = crate::Reg<txctrl::TXCTRL_SPEC>;
#[doc = "Controls the TX settings"]
pub mod txctrl;
#[doc = "FSMSTAT0 (r) register accessor: an alias for `Reg<FSMSTAT0_SPEC>`"]
pub type FSMSTAT0 = crate::Reg<fsmstat0::FSMSTAT0_SPEC>;
#[doc = "Radio status register"]
pub mod fsmstat0;
#[doc = "FSMSTAT1 (r) register accessor: an alias for `Reg<FSMSTAT1_SPEC>`"]
pub type FSMSTAT1 = crate::Reg<fsmstat1::FSMSTAT1_SPEC>;
#[doc = "Radio status register"]
pub mod fsmstat1;
#[doc = "FIFOPCTRL (rw) register accessor: an alias for `Reg<FIFOPCTRL_SPEC>`"]
pub type FIFOPCTRL = crate::Reg<fifopctrl::FIFOPCTRL_SPEC>;
#[doc = "FIFOP threshold"]
pub mod fifopctrl;
#[doc = "FSMCTRL (rw) register accessor: an alias for `Reg<FSMCTRL_SPEC>`"]
pub type FSMCTRL = crate::Reg<fsmctrl::FSMCTRL_SPEC>;
#[doc = "FSM options"]
pub mod fsmctrl;
#[doc = "CCACTRL0 (rw) register accessor: an alias for `Reg<CCACTRL0_SPEC>`"]
pub type CCACTRL0 = crate::Reg<ccactrl0::CCACTRL0_SPEC>;
#[doc = "CCA threshold"]
pub mod ccactrl0;
#[doc = "CCACTRL1 (rw) register accessor: an alias for `Reg<CCACTRL1_SPEC>`"]
pub type CCACTRL1 = crate::Reg<ccactrl1::CCACTRL1_SPEC>;
#[doc = "Other CCA Options"]
pub mod ccactrl1;
#[doc = "RSSI (r) register accessor: an alias for `Reg<RSSI_SPEC>`"]
pub type RSSI = crate::Reg<rssi::RSSI_SPEC>;
#[doc = "RSSI status register"]
pub mod rssi;
#[doc = "RSSISTAT (r) register accessor: an alias for `Reg<RSSISTAT_SPEC>`"]
pub type RSSISTAT = crate::Reg<rssistat::RSSISTAT_SPEC>;
#[doc = "RSSI valid status register"]
pub mod rssistat;
#[doc = "RXFIRST (r) register accessor: an alias for `Reg<RXFIRST_SPEC>`"]
pub type RXFIRST = crate::Reg<rxfirst::RXFIRST_SPEC>;
#[doc = "First byte in RX FIFO"]
pub mod rxfirst;
#[doc = "RXFIFOCNT (r) register accessor: an alias for `Reg<RXFIFOCNT_SPEC>`"]
pub type RXFIFOCNT = crate::Reg<rxfifocnt::RXFIFOCNT_SPEC>;
#[doc = "Number of bytes in RX FIFO"]
pub mod rxfifocnt;
#[doc = "TXFIFOCNT (r) register accessor: an alias for `Reg<TXFIFOCNT_SPEC>`"]
pub type TXFIFOCNT = crate::Reg<txfifocnt::TXFIFOCNT_SPEC>;
#[doc = "Number of bytes in TX FIFO"]
pub mod txfifocnt;
#[doc = "RXFIRST_PTR (r) register accessor: an alias for `Reg<RXFIRST_PTR_SPEC>`"]
pub type RXFIRST_PTR = crate::Reg<rxfirst_ptr::RXFIRST_PTR_SPEC>;
#[doc = "RX FIFO pointer"]
pub mod rxfirst_ptr;
#[doc = "RXLAST_PTR (r) register accessor: an alias for `Reg<RXLAST_PTR_SPEC>`"]
pub type RXLAST_PTR = crate::Reg<rxlast_ptr::RXLAST_PTR_SPEC>;
#[doc = "RX FIFO pointer"]
pub mod rxlast_ptr;
#[doc = "RXP1_PTR (r) register accessor: an alias for `Reg<RXP1_PTR_SPEC>`"]
pub type RXP1_PTR = crate::Reg<rxp1_ptr::RXP1_PTR_SPEC>;
#[doc = "RX FIFO pointer"]
pub mod rxp1_ptr;
#[doc = "TXFIRST_PTR (r) register accessor: an alias for `Reg<TXFIRST_PTR_SPEC>`"]
pub type TXFIRST_PTR = crate::Reg<txfirst_ptr::TXFIRST_PTR_SPEC>;
#[doc = "TX FIFO pointer"]
pub mod txfirst_ptr;
#[doc = "TXLAST_PTR (r) register accessor: an alias for `Reg<TXLAST_PTR_SPEC>`"]
pub type TXLAST_PTR = crate::Reg<txlast_ptr::TXLAST_PTR_SPEC>;
#[doc = "TX FIFO pointer"]
pub mod txlast_ptr;
#[doc = "RFIRQM0 (rw) register accessor: an alias for `Reg<RFIRQM0_SPEC>`"]
pub type RFIRQM0 = crate::Reg<rfirqm0::RFIRQM0_SPEC>;
#[doc = "RF interrupt masks"]
pub mod rfirqm0;
#[doc = "RFIRQM1 (rw) register accessor: an alias for `Reg<RFIRQM1_SPEC>`"]
pub type RFIRQM1 = crate::Reg<rfirqm1::RFIRQM1_SPEC>;
#[doc = "RF interrupt masks"]
pub mod rfirqm1;
#[doc = "RFERRM (rw) register accessor: an alias for `Reg<RFERRM_SPEC>`"]
pub type RFERRM = crate::Reg<rferrm::RFERRM_SPEC>;
#[doc = "RF error interrupt mask"]
pub mod rferrm;
#[doc = "RFRND (r) register accessor: an alias for `Reg<RFRND_SPEC>`"]
pub type RFRND = crate::Reg<rfrnd::RFRND_SPEC>;
#[doc = "Random data"]
pub mod rfrnd;
#[doc = "MDMCTRL0 (rw) register accessor: an alias for `Reg<MDMCTRL0_SPEC>`"]
pub type MDMCTRL0 = crate::Reg<mdmctrl0::MDMCTRL0_SPEC>;
#[doc = "Controls modem"]
pub mod mdmctrl0;
#[doc = "MDMCTRL1 (rw) register accessor: an alias for `Reg<MDMCTRL1_SPEC>`"]
pub type MDMCTRL1 = crate::Reg<mdmctrl1::MDMCTRL1_SPEC>;
#[doc = "Controls modem"]
pub mod mdmctrl1;
#[doc = "FREQEST (r) register accessor: an alias for `Reg<FREQEST_SPEC>`"]
pub type FREQEST = crate::Reg<freqest::FREQEST_SPEC>;
#[doc = "Estimated RF frequency offset"]
pub mod freqest;
#[doc = "RXCTRL (rw) register accessor: an alias for `Reg<RXCTRL_SPEC>`"]
pub type RXCTRL = crate::Reg<rxctrl::RXCTRL_SPEC>;
#[doc = "Tune receive section"]
pub mod rxctrl;
#[doc = "FSCTRL (rw) register accessor: an alias for `Reg<FSCTRL_SPEC>`"]
pub type FSCTRL = crate::Reg<fsctrl::FSCTRL_SPEC>;
#[doc = "Tune frequency synthesizer"]
pub mod fsctrl;
#[doc = "FSCAL0 (rw) register accessor: an alias for `Reg<FSCAL0_SPEC>`"]
pub type FSCAL0 = crate::Reg<fscal0::FSCAL0_SPEC>;
#[doc = "Tune frequency calibration"]
pub mod fscal0;
#[doc = "FSCAL1 (rw) register accessor: an alias for `Reg<FSCAL1_SPEC>`"]
pub type FSCAL1 = crate::Reg<fscal1::FSCAL1_SPEC>;
#[doc = "Tune frequency calibration"]
pub mod fscal1;
#[doc = "FSCAL2 (rw) register accessor: an alias for `Reg<FSCAL2_SPEC>`"]
pub type FSCAL2 = crate::Reg<fscal2::FSCAL2_SPEC>;
#[doc = "Tune frequency calibration"]
pub mod fscal2;
#[doc = "FSCAL3 (rw) register accessor: an alias for `Reg<FSCAL3_SPEC>`"]
pub type FSCAL3 = crate::Reg<fscal3::FSCAL3_SPEC>;
#[doc = "Tune frequency calibration"]
pub mod fscal3;
#[doc = "AGCCTRL0 (rw) register accessor: an alias for `Reg<AGCCTRL0_SPEC>`"]
pub type AGCCTRL0 = crate::Reg<agcctrl0::AGCCTRL0_SPEC>;
#[doc = "AGC dynamic range control"]
pub mod agcctrl0;
#[doc = "AGCCTRL1 (rw) register accessor: an alias for `Reg<AGCCTRL1_SPEC>`"]
pub type AGCCTRL1 = crate::Reg<agcctrl1::AGCCTRL1_SPEC>;
#[doc = "AGC reference level"]
pub mod agcctrl1;
#[doc = "AGCCTRL2 (rw) register accessor: an alias for `Reg<AGCCTRL2_SPEC>`"]
pub type AGCCTRL2 = crate::Reg<agcctrl2::AGCCTRL2_SPEC>;
#[doc = "AGC gain override"]
pub mod agcctrl2;
#[doc = "AGCCTRL3 (rw) register accessor: an alias for `Reg<AGCCTRL3_SPEC>`"]
pub type AGCCTRL3 = crate::Reg<agcctrl3::AGCCTRL3_SPEC>;
#[doc = "AGC control"]
pub mod agcctrl3;
#[doc = "ADCTEST0 (rw) register accessor: an alias for `Reg<ADCTEST0_SPEC>`"]
pub type ADCTEST0 = crate::Reg<adctest0::ADCTEST0_SPEC>;
#[doc = "ADC tuning"]
pub mod adctest0;
#[doc = "ADCTEST1 (rw) register accessor: an alias for `Reg<ADCTEST1_SPEC>`"]
pub type ADCTEST1 = crate::Reg<adctest1::ADCTEST1_SPEC>;
#[doc = "ADC tuning"]
pub mod adctest1;
#[doc = "ADCTEST2 (rw) register accessor: an alias for `Reg<ADCTEST2_SPEC>`"]
pub type ADCTEST2 = crate::Reg<adctest2::ADCTEST2_SPEC>;
#[doc = "ADC tuning"]
pub mod adctest2;
#[doc = "MDMTEST0 (rw) register accessor: an alias for `Reg<MDMTEST0_SPEC>`"]
pub type MDMTEST0 = crate::Reg<mdmtest0::MDMTEST0_SPEC>;
#[doc = "Test register for modem"]
pub mod mdmtest0;
#[doc = "MDMTEST1 (rw) register accessor: an alias for `Reg<MDMTEST1_SPEC>`"]
pub type MDMTEST1 = crate::Reg<mdmtest1::MDMTEST1_SPEC>;
#[doc = "Test Register for Modem"]
pub mod mdmtest1;
#[doc = "DACTEST0 (rw) register accessor: an alias for `Reg<DACTEST0_SPEC>`"]
pub type DACTEST0 = crate::Reg<dactest0::DACTEST0_SPEC>;
#[doc = "DAC override value"]
pub mod dactest0;
#[doc = "DACTEST1 (rw) register accessor: an alias for `Reg<DACTEST1_SPEC>`"]
pub type DACTEST1 = crate::Reg<dactest1::DACTEST1_SPEC>;
#[doc = "DAC override value"]
pub mod dactest1;
#[doc = "DACTEST2 (rw) register accessor: an alias for `Reg<DACTEST2_SPEC>`"]
pub type DACTEST2 = crate::Reg<dactest2::DACTEST2_SPEC>;
#[doc = "DAC test setting"]
pub mod dactest2;
#[doc = "ATEST (rw) register accessor: an alias for `Reg<ATEST_SPEC>`"]
pub type ATEST = crate::Reg<atest::ATEST_SPEC>;
#[doc = "Analog test control"]
pub mod atest;
#[doc = "PTEST0 (rw) register accessor: an alias for `Reg<PTEST0_SPEC>`"]
pub type PTEST0 = crate::Reg<ptest0::PTEST0_SPEC>;
#[doc = "Override power-down register"]
pub mod ptest0;
#[doc = "PTEST1 (rw) register accessor: an alias for `Reg<PTEST1_SPEC>`"]
pub type PTEST1 = crate::Reg<ptest1::PTEST1_SPEC>;
#[doc = "Override power-down register"]
pub mod ptest1;
#[doc = "CSPPROG_0 (r) register accessor: an alias for `Reg<CSPPROG_0_SPEC>`"]
pub type CSPPROG_0 = crate::Reg<cspprog_0::CSPPROG_0_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_0;
#[doc = "CSPPROG_1 (r) register accessor: an alias for `Reg<CSPPROG_1_SPEC>`"]
pub type CSPPROG_1 = crate::Reg<cspprog_1::CSPPROG_1_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_1;
#[doc = "CSPPROG_2 (r) register accessor: an alias for `Reg<CSPPROG_2_SPEC>`"]
pub type CSPPROG_2 = crate::Reg<cspprog_2::CSPPROG_2_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_2;
#[doc = "CSPPROG_3 (r) register accessor: an alias for `Reg<CSPPROG_3_SPEC>`"]
pub type CSPPROG_3 = crate::Reg<cspprog_3::CSPPROG_3_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_3;
#[doc = "CSPPROG_4 (r) register accessor: an alias for `Reg<CSPPROG_4_SPEC>`"]
pub type CSPPROG_4 = crate::Reg<cspprog_4::CSPPROG_4_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_4;
#[doc = "CSPPROG_5 (r) register accessor: an alias for `Reg<CSPPROG_5_SPEC>`"]
pub type CSPPROG_5 = crate::Reg<cspprog_5::CSPPROG_5_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_5;
#[doc = "CSPPROG_6 (r) register accessor: an alias for `Reg<CSPPROG_6_SPEC>`"]
pub type CSPPROG_6 = crate::Reg<cspprog_6::CSPPROG_6_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_6;
#[doc = "CSPPROG_7 (r) register accessor: an alias for `Reg<CSPPROG_7_SPEC>`"]
pub type CSPPROG_7 = crate::Reg<cspprog_7::CSPPROG_7_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_7;
#[doc = "CSPPROG_8 (r) register accessor: an alias for `Reg<CSPPROG_8_SPEC>`"]
pub type CSPPROG_8 = crate::Reg<cspprog_8::CSPPROG_8_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_8;
#[doc = "CSPPROG_9 (r) register accessor: an alias for `Reg<CSPPROG_9_SPEC>`"]
pub type CSPPROG_9 = crate::Reg<cspprog_9::CSPPROG_9_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_9;
#[doc = "CSPPROG_10 (r) register accessor: an alias for `Reg<CSPPROG_10_SPEC>`"]
pub type CSPPROG_10 = crate::Reg<cspprog_10::CSPPROG_10_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_10;
#[doc = "CSPPROG_11 (r) register accessor: an alias for `Reg<CSPPROG_11_SPEC>`"]
pub type CSPPROG_11 = crate::Reg<cspprog_11::CSPPROG_11_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_11;
#[doc = "CSPPROG_12 (r) register accessor: an alias for `Reg<CSPPROG_12_SPEC>`"]
pub type CSPPROG_12 = crate::Reg<cspprog_12::CSPPROG_12_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_12;
#[doc = "CSPPROG_13 (r) register accessor: an alias for `Reg<CSPPROG_13_SPEC>`"]
pub type CSPPROG_13 = crate::Reg<cspprog_13::CSPPROG_13_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_13;
#[doc = "CSPPROG_14 (r) register accessor: an alias for `Reg<CSPPROG_14_SPEC>`"]
pub type CSPPROG_14 = crate::Reg<cspprog_14::CSPPROG_14_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_14;
#[doc = "CSPPROG_15 (r) register accessor: an alias for `Reg<CSPPROG_15_SPEC>`"]
pub type CSPPROG_15 = crate::Reg<cspprog_15::CSPPROG_15_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_15;
#[doc = "CSPPROG_16 (r) register accessor: an alias for `Reg<CSPPROG_16_SPEC>`"]
pub type CSPPROG_16 = crate::Reg<cspprog_16::CSPPROG_16_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_16;
#[doc = "CSPPROG_17 (r) register accessor: an alias for `Reg<CSPPROG_17_SPEC>`"]
pub type CSPPROG_17 = crate::Reg<cspprog_17::CSPPROG_17_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_17;
#[doc = "CSPPROG_18 (r) register accessor: an alias for `Reg<CSPPROG_18_SPEC>`"]
pub type CSPPROG_18 = crate::Reg<cspprog_18::CSPPROG_18_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_18;
#[doc = "CSPPROG_19 (r) register accessor: an alias for `Reg<CSPPROG_19_SPEC>`"]
pub type CSPPROG_19 = crate::Reg<cspprog_19::CSPPROG_19_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_19;
#[doc = "CSPPROG_20 (r) register accessor: an alias for `Reg<CSPPROG_20_SPEC>`"]
pub type CSPPROG_20 = crate::Reg<cspprog_20::CSPPROG_20_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_20;
#[doc = "CSPPROG_21 (r) register accessor: an alias for `Reg<CSPPROG_21_SPEC>`"]
pub type CSPPROG_21 = crate::Reg<cspprog_21::CSPPROG_21_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_21;
#[doc = "CSPPROG_22 (r) register accessor: an alias for `Reg<CSPPROG_22_SPEC>`"]
pub type CSPPROG_22 = crate::Reg<cspprog_22::CSPPROG_22_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_22;
#[doc = "CSPPROG_23 (r) register accessor: an alias for `Reg<CSPPROG_23_SPEC>`"]
pub type CSPPROG_23 = crate::Reg<cspprog_23::CSPPROG_23_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_23;
#[doc = "CSPCTRL (rw) register accessor: an alias for `Reg<CSPCTRL_SPEC>`"]
pub type CSPCTRL = crate::Reg<cspctrl::CSPCTRL_SPEC>;
#[doc = "CSP control bit"]
pub mod cspctrl;
#[doc = "CSPSTAT (r) register accessor: an alias for `Reg<CSPSTAT_SPEC>`"]
pub type CSPSTAT = crate::Reg<cspstat::CSPSTAT_SPEC>;
#[doc = "CSP status register"]
pub mod cspstat;
#[doc = "CSPX (r) register accessor: an alias for `Reg<CSPX_SPEC>`"]
pub type CSPX = crate::Reg<cspx::CSPX_SPEC>;
#[doc = "CSP X data register"]
pub mod cspx;
#[doc = "CSPY (r) register accessor: an alias for `Reg<CSPY_SPEC>`"]
pub type CSPY = crate::Reg<cspy::CSPY_SPEC>;
#[doc = "CSP Y data register"]
pub mod cspy;
#[doc = "CSPZ (r) register accessor: an alias for `Reg<CSPZ_SPEC>`"]
pub type CSPZ = crate::Reg<cspz::CSPZ_SPEC>;
#[doc = "CSP Z data register"]
pub mod cspz;
#[doc = "CSPT (r) register accessor: an alias for `Reg<CSPT_SPEC>`"]
pub type CSPT = crate::Reg<cspt::CSPT_SPEC>;
#[doc = "CSP T data register"]
pub mod cspt;
#[doc = "RFC_OBS_CTRL0 (rw) register accessor: an alias for `Reg<RFC_OBS_CTRL0_SPEC>`"]
pub type RFC_OBS_CTRL0 = crate::Reg<rfc_obs_ctrl0::RFC_OBS_CTRL0_SPEC>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl0;
#[doc = "RFC_OBS_CTRL1 (rw) register accessor: an alias for `Reg<RFC_OBS_CTRL1_SPEC>`"]
pub type RFC_OBS_CTRL1 = crate::Reg<rfc_obs_ctrl1::RFC_OBS_CTRL1_SPEC>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl1;
#[doc = "RFC_OBS_CTRL2 (rw) register accessor: an alias for `Reg<RFC_OBS_CTRL2_SPEC>`"]
pub type RFC_OBS_CTRL2 = crate::Reg<rfc_obs_ctrl2::RFC_OBS_CTRL2_SPEC>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl2;
#[doc = "TXFILTCFG (rw) register accessor: an alias for `Reg<TXFILTCFG_SPEC>`"]
pub type TXFILTCFG = crate::Reg<txfiltcfg::TXFILTCFG_SPEC>;
#[doc = "TX filter configuration"]
pub mod txfiltcfg;
