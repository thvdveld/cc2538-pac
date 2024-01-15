#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    frmfilt0: FRMFILT0,
    frmfilt1: FRMFILT1,
    srcmatch: SRCMATCH,
    srcshorten0: SRCSHORTEN0,
    srcshorten1: SRCSHORTEN1,
    srcshorten2: SRCSHORTEN2,
    srcexten0: SRCEXTEN0,
    srcexten1: SRCEXTEN1,
    srcexten2: SRCEXTEN2,
    frmctrl0: FRMCTRL0,
    frmctrl1: FRMCTRL1,
    rxenable: RXENABLE,
    rxmaskset: RXMASKSET,
    rxmaskclr: RXMASKCLR,
    freqtune: FREQTUNE,
    freqctrl: FREQCTRL,
    txpower: TXPOWER,
    txctrl: TXCTRL,
    fsmstat0: FSMSTAT0,
    fsmstat1: FSMSTAT1,
    fifopctrl: FIFOPCTRL,
    fsmctrl: FSMCTRL,
    ccactrl0: CCACTRL0,
    ccactrl1: CCACTRL1,
    rssi: RSSI,
    rssistat: RSSISTAT,
    rxfirst: RXFIRST,
    rxfifocnt: RXFIFOCNT,
    txfifocnt: TXFIFOCNT,
    rxfirst_ptr: RXFIRST_PTR,
    rxlast_ptr: RXLAST_PTR,
    rxp1_ptr: RXP1_PTR,
    _reserved32: [u8; 0x04],
    txfirst_ptr: TXFIRST_PTR,
    txlast_ptr: TXLAST_PTR,
    rfirqm0: RFIRQM0,
    rfirqm1: RFIRQM1,
    rferrm: RFERRM,
    _reserved37: [u8; 0x04],
    rfrnd: RFRND,
    mdmctrl0: MDMCTRL0,
    mdmctrl1: MDMCTRL1,
    freqest: FREQEST,
    rxctrl: RXCTRL,
    fsctrl: FSCTRL,
    fscal0: FSCAL0,
    fscal1: FSCAL1,
    fscal2: FSCAL2,
    fscal3: FSCAL3,
    agcctrl0: AGCCTRL0,
    agcctrl1: AGCCTRL1,
    agcctrl2: AGCCTRL2,
    agcctrl3: AGCCTRL3,
    adctest0: ADCTEST0,
    adctest1: ADCTEST1,
    adctest2: ADCTEST2,
    mdmtest0: MDMTEST0,
    mdmtest1: MDMTEST1,
    dactest0: DACTEST0,
    dactest1: DACTEST1,
    dactest2: DACTEST2,
    atest: ATEST,
    ptest0: PTEST0,
    ptest1: PTEST1,
    cspprog_0: CSPPROG_0,
    cspprog_1: CSPPROG_1,
    cspprog_2: CSPPROG_2,
    cspprog_3: CSPPROG_3,
    cspprog_4: CSPPROG_4,
    cspprog_5: CSPPROG_5,
    cspprog_6: CSPPROG_6,
    cspprog_7: CSPPROG_7,
    cspprog_8: CSPPROG_8,
    cspprog_9: CSPPROG_9,
    cspprog_10: CSPPROG_10,
    cspprog_11: CSPPROG_11,
    cspprog_12: CSPPROG_12,
    cspprog_13: CSPPROG_13,
    cspprog_14: CSPPROG_14,
    cspprog_15: CSPPROG_15,
    cspprog_16: CSPPROG_16,
    cspprog_17: CSPPROG_17,
    cspprog_18: CSPPROG_18,
    cspprog_19: CSPPROG_19,
    cspprog_20: CSPPROG_20,
    cspprog_21: CSPPROG_21,
    cspprog_22: CSPPROG_22,
    cspprog_23: CSPPROG_23,
    _reserved86: [u8; 0x20],
    cspctrl: CSPCTRL,
    cspstat: CSPSTAT,
    cspx: CSPX,
    cspy: CSPY,
    cspz: CSPZ,
    cspt: CSPT,
    _reserved92: [u8; 0x14],
    rfc_obs_ctrl0: RFC_OBS_CTRL0,
    rfc_obs_ctrl1: RFC_OBS_CTRL1,
    rfc_obs_ctrl2: RFC_OBS_CTRL2,
    _reserved95: [u8; 0x30],
    txfiltcfg: TXFILTCFG,
}
impl RegisterBlock {
    #[doc = "0x00 - The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
    #[inline(always)]
    pub const fn frmfilt0(&self) -> &FRMFILT0 {
        &self.frmfilt0
    }
    #[doc = "0x04 - The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
    #[inline(always)]
    pub const fn frmfilt1(&self) -> &FRMFILT1 {
        &self.frmfilt1
    }
    #[doc = "0x08 - Source address matching and pending bits"]
    #[inline(always)]
    pub const fn srcmatch(&self) -> &SRCMATCH {
        &self.srcmatch
    }
    #[doc = "0x0c - Short address matching"]
    #[inline(always)]
    pub const fn srcshorten0(&self) -> &SRCSHORTEN0 {
        &self.srcshorten0
    }
    #[doc = "0x10 - Short address matching"]
    #[inline(always)]
    pub const fn srcshorten1(&self) -> &SRCSHORTEN1 {
        &self.srcshorten1
    }
    #[doc = "0x14 - Short address matching"]
    #[inline(always)]
    pub const fn srcshorten2(&self) -> &SRCSHORTEN2 {
        &self.srcshorten2
    }
    #[doc = "0x18 - Extended address matching"]
    #[inline(always)]
    pub const fn srcexten0(&self) -> &SRCEXTEN0 {
        &self.srcexten0
    }
    #[doc = "0x1c - Extended address matching"]
    #[inline(always)]
    pub const fn srcexten1(&self) -> &SRCEXTEN1 {
        &self.srcexten1
    }
    #[doc = "0x20 - Extended address matching"]
    #[inline(always)]
    pub const fn srcexten2(&self) -> &SRCEXTEN2 {
        &self.srcexten2
    }
    #[doc = "0x24 - Frame handling"]
    #[inline(always)]
    pub const fn frmctrl0(&self) -> &FRMCTRL0 {
        &self.frmctrl0
    }
    #[doc = "0x28 - Frame handling"]
    #[inline(always)]
    pub const fn frmctrl1(&self) -> &FRMCTRL1 {
        &self.frmctrl1
    }
    #[doc = "0x2c - RX enabling"]
    #[inline(always)]
    pub const fn rxenable(&self) -> &RXENABLE {
        &self.rxenable
    }
    #[doc = "0x30 - RX enabling"]
    #[inline(always)]
    pub const fn rxmaskset(&self) -> &RXMASKSET {
        &self.rxmaskset
    }
    #[doc = "0x34 - RX disabling"]
    #[inline(always)]
    pub const fn rxmaskclr(&self) -> &RXMASKCLR {
        &self.rxmaskclr
    }
    #[doc = "0x38 - Crystal oscillator frequency tuning"]
    #[inline(always)]
    pub const fn freqtune(&self) -> &FREQTUNE {
        &self.freqtune
    }
    #[doc = "0x3c - Controls the RF frequency"]
    #[inline(always)]
    pub const fn freqctrl(&self) -> &FREQCTRL {
        &self.freqctrl
    }
    #[doc = "0x40 - Controls the output power"]
    #[inline(always)]
    pub const fn txpower(&self) -> &TXPOWER {
        &self.txpower
    }
    #[doc = "0x44 - Controls the TX settings"]
    #[inline(always)]
    pub const fn txctrl(&self) -> &TXCTRL {
        &self.txctrl
    }
    #[doc = "0x48 - Radio status register"]
    #[inline(always)]
    pub const fn fsmstat0(&self) -> &FSMSTAT0 {
        &self.fsmstat0
    }
    #[doc = "0x4c - Radio status register"]
    #[inline(always)]
    pub const fn fsmstat1(&self) -> &FSMSTAT1 {
        &self.fsmstat1
    }
    #[doc = "0x50 - FIFOP threshold"]
    #[inline(always)]
    pub const fn fifopctrl(&self) -> &FIFOPCTRL {
        &self.fifopctrl
    }
    #[doc = "0x54 - FSM options"]
    #[inline(always)]
    pub const fn fsmctrl(&self) -> &FSMCTRL {
        &self.fsmctrl
    }
    #[doc = "0x58 - CCA threshold"]
    #[inline(always)]
    pub const fn ccactrl0(&self) -> &CCACTRL0 {
        &self.ccactrl0
    }
    #[doc = "0x5c - Other CCA Options"]
    #[inline(always)]
    pub const fn ccactrl1(&self) -> &CCACTRL1 {
        &self.ccactrl1
    }
    #[doc = "0x60 - RSSI status register"]
    #[inline(always)]
    pub const fn rssi(&self) -> &RSSI {
        &self.rssi
    }
    #[doc = "0x64 - RSSI valid status register"]
    #[inline(always)]
    pub const fn rssistat(&self) -> &RSSISTAT {
        &self.rssistat
    }
    #[doc = "0x68 - First byte in RX FIFO"]
    #[inline(always)]
    pub const fn rxfirst(&self) -> &RXFIRST {
        &self.rxfirst
    }
    #[doc = "0x6c - Number of bytes in RX FIFO"]
    #[inline(always)]
    pub const fn rxfifocnt(&self) -> &RXFIFOCNT {
        &self.rxfifocnt
    }
    #[doc = "0x70 - Number of bytes in TX FIFO"]
    #[inline(always)]
    pub const fn txfifocnt(&self) -> &TXFIFOCNT {
        &self.txfifocnt
    }
    #[doc = "0x74 - RX FIFO pointer"]
    #[inline(always)]
    pub const fn rxfirst_ptr(&self) -> &RXFIRST_PTR {
        &self.rxfirst_ptr
    }
    #[doc = "0x78 - RX FIFO pointer"]
    #[inline(always)]
    pub const fn rxlast_ptr(&self) -> &RXLAST_PTR {
        &self.rxlast_ptr
    }
    #[doc = "0x7c - RX FIFO pointer"]
    #[inline(always)]
    pub const fn rxp1_ptr(&self) -> &RXP1_PTR {
        &self.rxp1_ptr
    }
    #[doc = "0x84 - TX FIFO pointer"]
    #[inline(always)]
    pub const fn txfirst_ptr(&self) -> &TXFIRST_PTR {
        &self.txfirst_ptr
    }
    #[doc = "0x88 - TX FIFO pointer"]
    #[inline(always)]
    pub const fn txlast_ptr(&self) -> &TXLAST_PTR {
        &self.txlast_ptr
    }
    #[doc = "0x8c - RF interrupt masks"]
    #[inline(always)]
    pub const fn rfirqm0(&self) -> &RFIRQM0 {
        &self.rfirqm0
    }
    #[doc = "0x90 - RF interrupt masks"]
    #[inline(always)]
    pub const fn rfirqm1(&self) -> &RFIRQM1 {
        &self.rfirqm1
    }
    #[doc = "0x94 - RF error interrupt mask"]
    #[inline(always)]
    pub const fn rferrm(&self) -> &RFERRM {
        &self.rferrm
    }
    #[doc = "0x9c - Random data"]
    #[inline(always)]
    pub const fn rfrnd(&self) -> &RFRND {
        &self.rfrnd
    }
    #[doc = "0xa0 - Controls modem"]
    #[inline(always)]
    pub const fn mdmctrl0(&self) -> &MDMCTRL0 {
        &self.mdmctrl0
    }
    #[doc = "0xa4 - Controls modem"]
    #[inline(always)]
    pub const fn mdmctrl1(&self) -> &MDMCTRL1 {
        &self.mdmctrl1
    }
    #[doc = "0xa8 - Estimated RF frequency offset"]
    #[inline(always)]
    pub const fn freqest(&self) -> &FREQEST {
        &self.freqest
    }
    #[doc = "0xac - Tune receive section"]
    #[inline(always)]
    pub const fn rxctrl(&self) -> &RXCTRL {
        &self.rxctrl
    }
    #[doc = "0xb0 - Tune frequency synthesizer"]
    #[inline(always)]
    pub const fn fsctrl(&self) -> &FSCTRL {
        &self.fsctrl
    }
    #[doc = "0xb4 - Tune frequency calibration"]
    #[inline(always)]
    pub const fn fscal0(&self) -> &FSCAL0 {
        &self.fscal0
    }
    #[doc = "0xb8 - Tune frequency calibration"]
    #[inline(always)]
    pub const fn fscal1(&self) -> &FSCAL1 {
        &self.fscal1
    }
    #[doc = "0xbc - Tune frequency calibration"]
    #[inline(always)]
    pub const fn fscal2(&self) -> &FSCAL2 {
        &self.fscal2
    }
    #[doc = "0xc0 - Tune frequency calibration"]
    #[inline(always)]
    pub const fn fscal3(&self) -> &FSCAL3 {
        &self.fscal3
    }
    #[doc = "0xc4 - AGC dynamic range control"]
    #[inline(always)]
    pub const fn agcctrl0(&self) -> &AGCCTRL0 {
        &self.agcctrl0
    }
    #[doc = "0xc8 - AGC reference level"]
    #[inline(always)]
    pub const fn agcctrl1(&self) -> &AGCCTRL1 {
        &self.agcctrl1
    }
    #[doc = "0xcc - AGC gain override"]
    #[inline(always)]
    pub const fn agcctrl2(&self) -> &AGCCTRL2 {
        &self.agcctrl2
    }
    #[doc = "0xd0 - AGC control"]
    #[inline(always)]
    pub const fn agcctrl3(&self) -> &AGCCTRL3 {
        &self.agcctrl3
    }
    #[doc = "0xd4 - ADC tuning"]
    #[inline(always)]
    pub const fn adctest0(&self) -> &ADCTEST0 {
        &self.adctest0
    }
    #[doc = "0xd8 - ADC tuning"]
    #[inline(always)]
    pub const fn adctest1(&self) -> &ADCTEST1 {
        &self.adctest1
    }
    #[doc = "0xdc - ADC tuning"]
    #[inline(always)]
    pub const fn adctest2(&self) -> &ADCTEST2 {
        &self.adctest2
    }
    #[doc = "0xe0 - Test register for modem"]
    #[inline(always)]
    pub const fn mdmtest0(&self) -> &MDMTEST0 {
        &self.mdmtest0
    }
    #[doc = "0xe4 - Test Register for Modem"]
    #[inline(always)]
    pub const fn mdmtest1(&self) -> &MDMTEST1 {
        &self.mdmtest1
    }
    #[doc = "0xe8 - DAC override value"]
    #[inline(always)]
    pub const fn dactest0(&self) -> &DACTEST0 {
        &self.dactest0
    }
    #[doc = "0xec - DAC override value"]
    #[inline(always)]
    pub const fn dactest1(&self) -> &DACTEST1 {
        &self.dactest1
    }
    #[doc = "0xf0 - DAC test setting"]
    #[inline(always)]
    pub const fn dactest2(&self) -> &DACTEST2 {
        &self.dactest2
    }
    #[doc = "0xf4 - Analog test control"]
    #[inline(always)]
    pub const fn atest(&self) -> &ATEST {
        &self.atest
    }
    #[doc = "0xf8 - Override power-down register"]
    #[inline(always)]
    pub const fn ptest0(&self) -> &PTEST0 {
        &self.ptest0
    }
    #[doc = "0xfc - Override power-down register"]
    #[inline(always)]
    pub const fn ptest1(&self) -> &PTEST1 {
        &self.ptest1
    }
    #[doc = "0x100 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_0(&self) -> &CSPPROG_0 {
        &self.cspprog_0
    }
    #[doc = "0x104 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_1(&self) -> &CSPPROG_1 {
        &self.cspprog_1
    }
    #[doc = "0x108 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_2(&self) -> &CSPPROG_2 {
        &self.cspprog_2
    }
    #[doc = "0x10c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_3(&self) -> &CSPPROG_3 {
        &self.cspprog_3
    }
    #[doc = "0x110 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_4(&self) -> &CSPPROG_4 {
        &self.cspprog_4
    }
    #[doc = "0x114 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_5(&self) -> &CSPPROG_5 {
        &self.cspprog_5
    }
    #[doc = "0x118 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_6(&self) -> &CSPPROG_6 {
        &self.cspprog_6
    }
    #[doc = "0x11c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_7(&self) -> &CSPPROG_7 {
        &self.cspprog_7
    }
    #[doc = "0x120 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_8(&self) -> &CSPPROG_8 {
        &self.cspprog_8
    }
    #[doc = "0x124 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_9(&self) -> &CSPPROG_9 {
        &self.cspprog_9
    }
    #[doc = "0x128 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_10(&self) -> &CSPPROG_10 {
        &self.cspprog_10
    }
    #[doc = "0x12c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_11(&self) -> &CSPPROG_11 {
        &self.cspprog_11
    }
    #[doc = "0x130 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_12(&self) -> &CSPPROG_12 {
        &self.cspprog_12
    }
    #[doc = "0x134 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_13(&self) -> &CSPPROG_13 {
        &self.cspprog_13
    }
    #[doc = "0x138 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_14(&self) -> &CSPPROG_14 {
        &self.cspprog_14
    }
    #[doc = "0x13c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_15(&self) -> &CSPPROG_15 {
        &self.cspprog_15
    }
    #[doc = "0x140 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_16(&self) -> &CSPPROG_16 {
        &self.cspprog_16
    }
    #[doc = "0x144 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_17(&self) -> &CSPPROG_17 {
        &self.cspprog_17
    }
    #[doc = "0x148 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_18(&self) -> &CSPPROG_18 {
        &self.cspprog_18
    }
    #[doc = "0x14c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_19(&self) -> &CSPPROG_19 {
        &self.cspprog_19
    }
    #[doc = "0x150 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_20(&self) -> &CSPPROG_20 {
        &self.cspprog_20
    }
    #[doc = "0x154 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_21(&self) -> &CSPPROG_21 {
        &self.cspprog_21
    }
    #[doc = "0x158 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_22(&self) -> &CSPPROG_22 {
        &self.cspprog_22
    }
    #[doc = "0x15c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_23(&self) -> &CSPPROG_23 {
        &self.cspprog_23
    }
    #[doc = "0x180 - CSP control bit"]
    #[inline(always)]
    pub const fn cspctrl(&self) -> &CSPCTRL {
        &self.cspctrl
    }
    #[doc = "0x184 - CSP status register"]
    #[inline(always)]
    pub const fn cspstat(&self) -> &CSPSTAT {
        &self.cspstat
    }
    #[doc = "0x188 - CSP X data register"]
    #[inline(always)]
    pub const fn cspx(&self) -> &CSPX {
        &self.cspx
    }
    #[doc = "0x18c - CSP Y data register"]
    #[inline(always)]
    pub const fn cspy(&self) -> &CSPY {
        &self.cspy
    }
    #[doc = "0x190 - CSP Z data register"]
    #[inline(always)]
    pub const fn cspz(&self) -> &CSPZ {
        &self.cspz
    }
    #[doc = "0x194 - CSP T data register"]
    #[inline(always)]
    pub const fn cspt(&self) -> &CSPT {
        &self.cspt
    }
    #[doc = "0x1ac - RF observation mux control"]
    #[inline(always)]
    pub const fn rfc_obs_ctrl0(&self) -> &RFC_OBS_CTRL0 {
        &self.rfc_obs_ctrl0
    }
    #[doc = "0x1b0 - RF observation mux control"]
    #[inline(always)]
    pub const fn rfc_obs_ctrl1(&self) -> &RFC_OBS_CTRL1 {
        &self.rfc_obs_ctrl1
    }
    #[doc = "0x1b4 - RF observation mux control"]
    #[inline(always)]
    pub const fn rfc_obs_ctrl2(&self) -> &RFC_OBS_CTRL2 {
        &self.rfc_obs_ctrl2
    }
    #[doc = "0x1e8 - TX filter configuration"]
    #[inline(always)]
    pub const fn txfiltcfg(&self) -> &TXFILTCFG {
        &self.txfiltcfg
    }
}
#[doc = "FRMFILT0 (rw) register accessor: The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmfilt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmfilt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmfilt0`]
module"]
pub type FRMFILT0 = crate::Reg<frmfilt0::FRMFILT0_SPEC>;
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
pub mod frmfilt0;
#[doc = "FRMFILT1 (rw) register accessor: The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmfilt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmfilt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmfilt1`]
module"]
pub type FRMFILT1 = crate::Reg<frmfilt1::FRMFILT1_SPEC>;
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
pub mod frmfilt1;
#[doc = "SRCMATCH (rw) register accessor: Source address matching and pending bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcmatch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcmatch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcmatch`]
module"]
pub type SRCMATCH = crate::Reg<srcmatch::SRCMATCH_SPEC>;
#[doc = "Source address matching and pending bits"]
pub mod srcmatch;
#[doc = "SRCSHORTEN0 (rw) register accessor: Short address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshorten0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshorten0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshorten0`]
module"]
pub type SRCSHORTEN0 = crate::Reg<srcshorten0::SRCSHORTEN0_SPEC>;
#[doc = "Short address matching"]
pub mod srcshorten0;
#[doc = "SRCSHORTEN1 (rw) register accessor: Short address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshorten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshorten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshorten1`]
module"]
pub type SRCSHORTEN1 = crate::Reg<srcshorten1::SRCSHORTEN1_SPEC>;
#[doc = "Short address matching"]
pub mod srcshorten1;
#[doc = "SRCSHORTEN2 (rw) register accessor: Short address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshorten2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshorten2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshorten2`]
module"]
pub type SRCSHORTEN2 = crate::Reg<srcshorten2::SRCSHORTEN2_SPEC>;
#[doc = "Short address matching"]
pub mod srcshorten2;
#[doc = "SRCEXTEN0 (rw) register accessor: Extended address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcexten0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcexten0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcexten0`]
module"]
pub type SRCEXTEN0 = crate::Reg<srcexten0::SRCEXTEN0_SPEC>;
#[doc = "Extended address matching"]
pub mod srcexten0;
#[doc = "SRCEXTEN1 (rw) register accessor: Extended address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcexten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcexten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcexten1`]
module"]
pub type SRCEXTEN1 = crate::Reg<srcexten1::SRCEXTEN1_SPEC>;
#[doc = "Extended address matching"]
pub mod srcexten1;
#[doc = "SRCEXTEN2 (rw) register accessor: Extended address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcexten2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcexten2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcexten2`]
module"]
pub type SRCEXTEN2 = crate::Reg<srcexten2::SRCEXTEN2_SPEC>;
#[doc = "Extended address matching"]
pub mod srcexten2;
#[doc = "FRMCTRL0 (rw) register accessor: Frame handling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmctrl0`]
module"]
pub type FRMCTRL0 = crate::Reg<frmctrl0::FRMCTRL0_SPEC>;
#[doc = "Frame handling"]
pub mod frmctrl0;
#[doc = "FRMCTRL1 (rw) register accessor: Frame handling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmctrl1`]
module"]
pub type FRMCTRL1 = crate::Reg<frmctrl1::FRMCTRL1_SPEC>;
#[doc = "Frame handling"]
pub mod frmctrl1;
#[doc = "RXENABLE (r) register accessor: RX enabling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxenable::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxenable`]
module"]
pub type RXENABLE = crate::Reg<rxenable::RXENABLE_SPEC>;
#[doc = "RX enabling"]
pub mod rxenable;
#[doc = "RXMASKSET (rw) register accessor: RX enabling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaskset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaskset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaskset`]
module"]
pub type RXMASKSET = crate::Reg<rxmaskset::RXMASKSET_SPEC>;
#[doc = "RX enabling"]
pub mod rxmaskset;
#[doc = "RXMASKCLR (rw) register accessor: RX disabling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaskclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaskclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaskclr`]
module"]
pub type RXMASKCLR = crate::Reg<rxmaskclr::RXMASKCLR_SPEC>;
#[doc = "RX disabling"]
pub mod rxmaskclr;
#[doc = "FREQTUNE (rw) register accessor: Crystal oscillator frequency tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqtune::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqtune::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqtune`]
module"]
pub type FREQTUNE = crate::Reg<freqtune::FREQTUNE_SPEC>;
#[doc = "Crystal oscillator frequency tuning"]
pub mod freqtune;
#[doc = "FREQCTRL (rw) register accessor: Controls the RF frequency\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqctrl`]
module"]
pub type FREQCTRL = crate::Reg<freqctrl::FREQCTRL_SPEC>;
#[doc = "Controls the RF frequency"]
pub mod freqctrl;
#[doc = "TXPOWER (rw) register accessor: Controls the output power\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpower`]
module"]
pub type TXPOWER = crate::Reg<txpower::TXPOWER_SPEC>;
#[doc = "Controls the output power"]
pub mod txpower;
#[doc = "TXCTRL (rw) register accessor: Controls the TX settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctrl`]
module"]
pub type TXCTRL = crate::Reg<txctrl::TXCTRL_SPEC>;
#[doc = "Controls the TX settings"]
pub mod txctrl;
#[doc = "FSMSTAT0 (r) register accessor: Radio status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsmstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmstat0`]
module"]
pub type FSMSTAT0 = crate::Reg<fsmstat0::FSMSTAT0_SPEC>;
#[doc = "Radio status register"]
pub mod fsmstat0;
#[doc = "FSMSTAT1 (r) register accessor: Radio status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsmstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmstat1`]
module"]
pub type FSMSTAT1 = crate::Reg<fsmstat1::FSMSTAT1_SPEC>;
#[doc = "Radio status register"]
pub mod fsmstat1;
#[doc = "FIFOPCTRL (rw) register accessor: FIFOP threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifopctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifopctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifopctrl`]
module"]
pub type FIFOPCTRL = crate::Reg<fifopctrl::FIFOPCTRL_SPEC>;
#[doc = "FIFOP threshold"]
pub mod fifopctrl;
#[doc = "FSMCTRL (rw) register accessor: FSM options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsmctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsmctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmctrl`]
module"]
pub type FSMCTRL = crate::Reg<fsmctrl::FSMCTRL_SPEC>;
#[doc = "FSM options"]
pub mod fsmctrl;
#[doc = "CCACTRL0 (rw) register accessor: CCA threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccactrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccactrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccactrl0`]
module"]
pub type CCACTRL0 = crate::Reg<ccactrl0::CCACTRL0_SPEC>;
#[doc = "CCA threshold"]
pub mod ccactrl0;
#[doc = "CCACTRL1 (rw) register accessor: Other CCA Options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccactrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccactrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccactrl1`]
module"]
pub type CCACTRL1 = crate::Reg<ccactrl1::CCACTRL1_SPEC>;
#[doc = "Other CCA Options"]
pub mod ccactrl1;
#[doc = "RSSI (r) register accessor: RSSI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rssi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rssi`]
module"]
pub type RSSI = crate::Reg<rssi::RSSI_SPEC>;
#[doc = "RSSI status register"]
pub mod rssi;
#[doc = "RSSISTAT (r) register accessor: RSSI valid status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rssistat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rssistat`]
module"]
pub type RSSISTAT = crate::Reg<rssistat::RSSISTAT_SPEC>;
#[doc = "RSSI valid status register"]
pub mod rssistat;
#[doc = "RXFIRST (r) register accessor: First byte in RX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfirst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfirst`]
module"]
pub type RXFIRST = crate::Reg<rxfirst::RXFIRST_SPEC>;
#[doc = "First byte in RX FIFO"]
pub mod rxfirst;
#[doc = "RXFIFOCNT (r) register accessor: Number of bytes in RX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifocnt`]
module"]
pub type RXFIFOCNT = crate::Reg<rxfifocnt::RXFIFOCNT_SPEC>;
#[doc = "Number of bytes in RX FIFO"]
pub mod rxfifocnt;
#[doc = "TXFIFOCNT (r) register accessor: Number of bytes in TX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifocnt`]
module"]
pub type TXFIFOCNT = crate::Reg<txfifocnt::TXFIFOCNT_SPEC>;
#[doc = "Number of bytes in TX FIFO"]
pub mod txfifocnt;
#[doc = "RXFIRST_PTR (r) register accessor: RX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfirst_ptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfirst_ptr`]
module"]
pub type RXFIRST_PTR = crate::Reg<rxfirst_ptr::RXFIRST_PTR_SPEC>;
#[doc = "RX FIFO pointer"]
pub mod rxfirst_ptr;
#[doc = "RXLAST_PTR (r) register accessor: RX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlast_ptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlast_ptr`]
module"]
pub type RXLAST_PTR = crate::Reg<rxlast_ptr::RXLAST_PTR_SPEC>;
#[doc = "RX FIFO pointer"]
pub mod rxlast_ptr;
#[doc = "RXP1_PTR (r) register accessor: RX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxp1_ptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxp1_ptr`]
module"]
pub type RXP1_PTR = crate::Reg<rxp1_ptr::RXP1_PTR_SPEC>;
#[doc = "RX FIFO pointer"]
pub mod rxp1_ptr;
#[doc = "TXFIRST_PTR (r) register accessor: TX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfirst_ptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfirst_ptr`]
module"]
pub type TXFIRST_PTR = crate::Reg<txfirst_ptr::TXFIRST_PTR_SPEC>;
#[doc = "TX FIFO pointer"]
pub mod txfirst_ptr;
#[doc = "TXLAST_PTR (r) register accessor: TX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlast_ptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlast_ptr`]
module"]
pub type TXLAST_PTR = crate::Reg<txlast_ptr::TXLAST_PTR_SPEC>;
#[doc = "TX FIFO pointer"]
pub mod txlast_ptr;
#[doc = "RFIRQM0 (rw) register accessor: RF interrupt masks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfirqm0`]
module"]
pub type RFIRQM0 = crate::Reg<rfirqm0::RFIRQM0_SPEC>;
#[doc = "RF interrupt masks"]
pub mod rfirqm0;
#[doc = "RFIRQM1 (rw) register accessor: RF interrupt masks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfirqm1`]
module"]
pub type RFIRQM1 = crate::Reg<rfirqm1::RFIRQM1_SPEC>;
#[doc = "RF interrupt masks"]
pub mod rfirqm1;
#[doc = "RFERRM (rw) register accessor: RF error interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rferrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rferrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rferrm`]
module"]
pub type RFERRM = crate::Reg<rferrm::RFERRM_SPEC>;
#[doc = "RF error interrupt mask"]
pub mod rferrm;
#[doc = "RFRND (r) register accessor: Random data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfrnd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfrnd`]
module"]
pub type RFRND = crate::Reg<rfrnd::RFRND_SPEC>;
#[doc = "Random data"]
pub mod rfrnd;
#[doc = "MDMCTRL0 (rw) register accessor: Controls modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdmctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdmctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmctrl0`]
module"]
pub type MDMCTRL0 = crate::Reg<mdmctrl0::MDMCTRL0_SPEC>;
#[doc = "Controls modem"]
pub mod mdmctrl0;
#[doc = "MDMCTRL1 (rw) register accessor: Controls modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdmctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdmctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmctrl1`]
module"]
pub type MDMCTRL1 = crate::Reg<mdmctrl1::MDMCTRL1_SPEC>;
#[doc = "Controls modem"]
pub mod mdmctrl1;
#[doc = "FREQEST (r) register accessor: Estimated RF frequency offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqest::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqest`]
module"]
pub type FREQEST = crate::Reg<freqest::FREQEST_SPEC>;
#[doc = "Estimated RF frequency offset"]
pub mod freqest;
#[doc = "RXCTRL (rw) register accessor: Tune receive section\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctrl`]
module"]
pub type RXCTRL = crate::Reg<rxctrl::RXCTRL_SPEC>;
#[doc = "Tune receive section"]
pub mod rxctrl;
#[doc = "FSCTRL (rw) register accessor: Tune frequency synthesizer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsctrl`]
module"]
pub type FSCTRL = crate::Reg<fsctrl::FSCTRL_SPEC>;
#[doc = "Tune frequency synthesizer"]
pub mod fsctrl;
#[doc = "FSCAL0 (rw) register accessor: Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscal0`]
module"]
pub type FSCAL0 = crate::Reg<fscal0::FSCAL0_SPEC>;
#[doc = "Tune frequency calibration"]
pub mod fscal0;
#[doc = "FSCAL1 (rw) register accessor: Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscal1`]
module"]
pub type FSCAL1 = crate::Reg<fscal1::FSCAL1_SPEC>;
#[doc = "Tune frequency calibration"]
pub mod fscal1;
#[doc = "FSCAL2 (rw) register accessor: Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscal2`]
module"]
pub type FSCAL2 = crate::Reg<fscal2::FSCAL2_SPEC>;
#[doc = "Tune frequency calibration"]
pub mod fscal2;
#[doc = "FSCAL3 (rw) register accessor: Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscal3`]
module"]
pub type FSCAL3 = crate::Reg<fscal3::FSCAL3_SPEC>;
#[doc = "Tune frequency calibration"]
pub mod fscal3;
#[doc = "AGCCTRL0 (rw) register accessor: AGC dynamic range control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agcctrl0`]
module"]
pub type AGCCTRL0 = crate::Reg<agcctrl0::AGCCTRL0_SPEC>;
#[doc = "AGC dynamic range control"]
pub mod agcctrl0;
#[doc = "AGCCTRL1 (rw) register accessor: AGC reference level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agcctrl1`]
module"]
pub type AGCCTRL1 = crate::Reg<agcctrl1::AGCCTRL1_SPEC>;
#[doc = "AGC reference level"]
pub mod agcctrl1;
#[doc = "AGCCTRL2 (rw) register accessor: AGC gain override\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agcctrl2`]
module"]
pub type AGCCTRL2 = crate::Reg<agcctrl2::AGCCTRL2_SPEC>;
#[doc = "AGC gain override"]
pub mod agcctrl2;
#[doc = "AGCCTRL3 (rw) register accessor: AGC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agcctrl3`]
module"]
pub type AGCCTRL3 = crate::Reg<agcctrl3::AGCCTRL3_SPEC>;
#[doc = "AGC control"]
pub mod agcctrl3;
#[doc = "ADCTEST0 (rw) register accessor: ADC tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctest0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctest0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctest0`]
module"]
pub type ADCTEST0 = crate::Reg<adctest0::ADCTEST0_SPEC>;
#[doc = "ADC tuning"]
pub mod adctest0;
#[doc = "ADCTEST1 (rw) register accessor: ADC tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctest1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctest1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctest1`]
module"]
pub type ADCTEST1 = crate::Reg<adctest1::ADCTEST1_SPEC>;
#[doc = "ADC tuning"]
pub mod adctest1;
#[doc = "ADCTEST2 (rw) register accessor: ADC tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctest2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctest2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctest2`]
module"]
pub type ADCTEST2 = crate::Reg<adctest2::ADCTEST2_SPEC>;
#[doc = "ADC tuning"]
pub mod adctest2;
#[doc = "MDMTEST0 (rw) register accessor: Test register for modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdmtest0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdmtest0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmtest0`]
module"]
pub type MDMTEST0 = crate::Reg<mdmtest0::MDMTEST0_SPEC>;
#[doc = "Test register for modem"]
pub mod mdmtest0;
#[doc = "MDMTEST1 (rw) register accessor: Test Register for Modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdmtest1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdmtest1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmtest1`]
module"]
pub type MDMTEST1 = crate::Reg<mdmtest1::MDMTEST1_SPEC>;
#[doc = "Test Register for Modem"]
pub mod mdmtest1;
#[doc = "DACTEST0 (rw) register accessor: DAC override value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dactest0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dactest0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dactest0`]
module"]
pub type DACTEST0 = crate::Reg<dactest0::DACTEST0_SPEC>;
#[doc = "DAC override value"]
pub mod dactest0;
#[doc = "DACTEST1 (rw) register accessor: DAC override value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dactest1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dactest1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dactest1`]
module"]
pub type DACTEST1 = crate::Reg<dactest1::DACTEST1_SPEC>;
#[doc = "DAC override value"]
pub mod dactest1;
#[doc = "DACTEST2 (rw) register accessor: DAC test setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dactest2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dactest2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dactest2`]
module"]
pub type DACTEST2 = crate::Reg<dactest2::DACTEST2_SPEC>;
#[doc = "DAC test setting"]
pub mod dactest2;
#[doc = "ATEST (rw) register accessor: Analog test control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atest::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atest::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atest`]
module"]
pub type ATEST = crate::Reg<atest::ATEST_SPEC>;
#[doc = "Analog test control"]
pub mod atest;
#[doc = "PTEST0 (rw) register accessor: Override power-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptest0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptest0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptest0`]
module"]
pub type PTEST0 = crate::Reg<ptest0::PTEST0_SPEC>;
#[doc = "Override power-down register"]
pub mod ptest0;
#[doc = "PTEST1 (rw) register accessor: Override power-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptest1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptest1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptest1`]
module"]
pub type PTEST1 = crate::Reg<ptest1::PTEST1_SPEC>;
#[doc = "Override power-down register"]
pub mod ptest1;
#[doc = "CSPPROG_0 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_0`]
module"]
pub type CSPPROG_0 = crate::Reg<cspprog_0::CSPPROG_0_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_0;
#[doc = "CSPPROG_1 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_1`]
module"]
pub type CSPPROG_1 = crate::Reg<cspprog_1::CSPPROG_1_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_1;
#[doc = "CSPPROG_2 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_2`]
module"]
pub type CSPPROG_2 = crate::Reg<cspprog_2::CSPPROG_2_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_2;
#[doc = "CSPPROG_3 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_3`]
module"]
pub type CSPPROG_3 = crate::Reg<cspprog_3::CSPPROG_3_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_3;
#[doc = "CSPPROG_4 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_4`]
module"]
pub type CSPPROG_4 = crate::Reg<cspprog_4::CSPPROG_4_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_4;
#[doc = "CSPPROG_5 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_5`]
module"]
pub type CSPPROG_5 = crate::Reg<cspprog_5::CSPPROG_5_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_5;
#[doc = "CSPPROG_6 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_6`]
module"]
pub type CSPPROG_6 = crate::Reg<cspprog_6::CSPPROG_6_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_6;
#[doc = "CSPPROG_7 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_7`]
module"]
pub type CSPPROG_7 = crate::Reg<cspprog_7::CSPPROG_7_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_7;
#[doc = "CSPPROG_8 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_8`]
module"]
pub type CSPPROG_8 = crate::Reg<cspprog_8::CSPPROG_8_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_8;
#[doc = "CSPPROG_9 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_9`]
module"]
pub type CSPPROG_9 = crate::Reg<cspprog_9::CSPPROG_9_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_9;
#[doc = "CSPPROG_10 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_10`]
module"]
pub type CSPPROG_10 = crate::Reg<cspprog_10::CSPPROG_10_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_10;
#[doc = "CSPPROG_11 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_11`]
module"]
pub type CSPPROG_11 = crate::Reg<cspprog_11::CSPPROG_11_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_11;
#[doc = "CSPPROG_12 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_12`]
module"]
pub type CSPPROG_12 = crate::Reg<cspprog_12::CSPPROG_12_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_12;
#[doc = "CSPPROG_13 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_13`]
module"]
pub type CSPPROG_13 = crate::Reg<cspprog_13::CSPPROG_13_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_13;
#[doc = "CSPPROG_14 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_14`]
module"]
pub type CSPPROG_14 = crate::Reg<cspprog_14::CSPPROG_14_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_14;
#[doc = "CSPPROG_15 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_15`]
module"]
pub type CSPPROG_15 = crate::Reg<cspprog_15::CSPPROG_15_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_15;
#[doc = "CSPPROG_16 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_16`]
module"]
pub type CSPPROG_16 = crate::Reg<cspprog_16::CSPPROG_16_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_16;
#[doc = "CSPPROG_17 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_17`]
module"]
pub type CSPPROG_17 = crate::Reg<cspprog_17::CSPPROG_17_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_17;
#[doc = "CSPPROG_18 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_18::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_18`]
module"]
pub type CSPPROG_18 = crate::Reg<cspprog_18::CSPPROG_18_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_18;
#[doc = "CSPPROG_19 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_19::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_19`]
module"]
pub type CSPPROG_19 = crate::Reg<cspprog_19::CSPPROG_19_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_19;
#[doc = "CSPPROG_20 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_20::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_20`]
module"]
pub type CSPPROG_20 = crate::Reg<cspprog_20::CSPPROG_20_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_20;
#[doc = "CSPPROG_21 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_21::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_21`]
module"]
pub type CSPPROG_21 = crate::Reg<cspprog_21::CSPPROG_21_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_21;
#[doc = "CSPPROG_22 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_22::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_22`]
module"]
pub type CSPPROG_22 = crate::Reg<cspprog_22::CSPPROG_22_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_22;
#[doc = "CSPPROG_23 (r) register accessor: CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_23::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_23`]
module"]
pub type CSPPROG_23 = crate::Reg<cspprog_23::CSPPROG_23_SPEC>;
#[doc = "CSP program"]
pub mod cspprog_23;
#[doc = "CSPCTRL (rw) register accessor: CSP control bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cspctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspctrl`]
module"]
pub type CSPCTRL = crate::Reg<cspctrl::CSPCTRL_SPEC>;
#[doc = "CSP control bit"]
pub mod cspctrl;
#[doc = "CSPSTAT (r) register accessor: CSP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspstat`]
module"]
pub type CSPSTAT = crate::Reg<cspstat::CSPSTAT_SPEC>;
#[doc = "CSP status register"]
pub mod cspstat;
#[doc = "CSPX (r) register accessor: CSP X data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspx`]
module"]
pub type CSPX = crate::Reg<cspx::CSPX_SPEC>;
#[doc = "CSP X data register"]
pub mod cspx;
#[doc = "CSPY (r) register accessor: CSP Y data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspy`]
module"]
pub type CSPY = crate::Reg<cspy::CSPY_SPEC>;
#[doc = "CSP Y data register"]
pub mod cspy;
#[doc = "CSPZ (r) register accessor: CSP Z data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspz::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspz`]
module"]
pub type CSPZ = crate::Reg<cspz::CSPZ_SPEC>;
#[doc = "CSP Z data register"]
pub mod cspz;
#[doc = "CSPT (r) register accessor: CSP T data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspt`]
module"]
pub type CSPT = crate::Reg<cspt::CSPT_SPEC>;
#[doc = "CSP T data register"]
pub mod cspt;
#[doc = "RFC_OBS_CTRL0 (rw) register accessor: RF observation mux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc_obs_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfc_obs_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc_obs_ctrl0`]
module"]
pub type RFC_OBS_CTRL0 = crate::Reg<rfc_obs_ctrl0::RFC_OBS_CTRL0_SPEC>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl0;
#[doc = "RFC_OBS_CTRL1 (rw) register accessor: RF observation mux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc_obs_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfc_obs_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc_obs_ctrl1`]
module"]
pub type RFC_OBS_CTRL1 = crate::Reg<rfc_obs_ctrl1::RFC_OBS_CTRL1_SPEC>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl1;
#[doc = "RFC_OBS_CTRL2 (rw) register accessor: RF observation mux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc_obs_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfc_obs_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc_obs_ctrl2`]
module"]
pub type RFC_OBS_CTRL2 = crate::Reg<rfc_obs_ctrl2::RFC_OBS_CTRL2_SPEC>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl2;
#[doc = "TXFILTCFG (rw) register accessor: TX filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfiltcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfiltcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfiltcfg`]
module"]
pub type TXFILTCFG = crate::Reg<txfiltcfg::TXFILTCFG_SPEC>;
#[doc = "TX filter configuration"]
pub mod txfiltcfg;
