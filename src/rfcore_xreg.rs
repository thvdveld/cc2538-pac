#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    frmfilt0: Frmfilt0,
    frmfilt1: Frmfilt1,
    srcmatch: Srcmatch,
    srcshorten0: Srcshorten0,
    srcshorten1: Srcshorten1,
    srcshorten2: Srcshorten2,
    srcexten0: Srcexten0,
    srcexten1: Srcexten1,
    srcexten2: Srcexten2,
    frmctrl0: Frmctrl0,
    frmctrl1: Frmctrl1,
    rxenable: Rxenable,
    rxmaskset: Rxmaskset,
    rxmaskclr: Rxmaskclr,
    freqtune: Freqtune,
    freqctrl: Freqctrl,
    txpower: Txpower,
    txctrl: Txctrl,
    fsmstat0: Fsmstat0,
    fsmstat1: Fsmstat1,
    fifopctrl: Fifopctrl,
    fsmctrl: Fsmctrl,
    ccactrl0: Ccactrl0,
    ccactrl1: Ccactrl1,
    rssi: Rssi,
    rssistat: Rssistat,
    rxfirst: Rxfirst,
    rxfifocnt: Rxfifocnt,
    txfifocnt: Txfifocnt,
    rxfirst_ptr: RxfirstPtr,
    rxlast_ptr: RxlastPtr,
    rxp1_ptr: Rxp1Ptr,
    _reserved32: [u8; 0x04],
    txfirst_ptr: TxfirstPtr,
    txlast_ptr: TxlastPtr,
    rfirqm0: Rfirqm0,
    rfirqm1: Rfirqm1,
    rferrm: Rferrm,
    _reserved37: [u8; 0x04],
    rfrnd: Rfrnd,
    mdmctrl0: Mdmctrl0,
    mdmctrl1: Mdmctrl1,
    freqest: Freqest,
    rxctrl: Rxctrl,
    fsctrl: Fsctrl,
    fscal0: Fscal0,
    fscal1: Fscal1,
    fscal2: Fscal2,
    fscal3: Fscal3,
    agcctrl0: Agcctrl0,
    agcctrl1: Agcctrl1,
    agcctrl2: Agcctrl2,
    agcctrl3: Agcctrl3,
    adctest0: Adctest0,
    adctest1: Adctest1,
    adctest2: Adctest2,
    mdmtest0: Mdmtest0,
    mdmtest1: Mdmtest1,
    dactest0: Dactest0,
    dactest1: Dactest1,
    dactest2: Dactest2,
    atest: Atest,
    ptest0: Ptest0,
    ptest1: Ptest1,
    cspprog_0: Cspprog0,
    cspprog_1: Cspprog1,
    cspprog_2: Cspprog2,
    cspprog_3: Cspprog3,
    cspprog_4: Cspprog4,
    cspprog_5: Cspprog5,
    cspprog_6: Cspprog6,
    cspprog_7: Cspprog7,
    cspprog_8: Cspprog8,
    cspprog_9: Cspprog9,
    cspprog_10: Cspprog10,
    cspprog_11: Cspprog11,
    cspprog_12: Cspprog12,
    cspprog_13: Cspprog13,
    cspprog_14: Cspprog14,
    cspprog_15: Cspprog15,
    cspprog_16: Cspprog16,
    cspprog_17: Cspprog17,
    cspprog_18: Cspprog18,
    cspprog_19: Cspprog19,
    cspprog_20: Cspprog20,
    cspprog_21: Cspprog21,
    cspprog_22: Cspprog22,
    cspprog_23: Cspprog23,
    _reserved86: [u8; 0x20],
    cspctrl: Cspctrl,
    cspstat: Cspstat,
    cspx: Cspx,
    cspy: Cspy,
    cspz: Cspz,
    cspt: Cspt,
    _reserved92: [u8; 0x14],
    rfc_obs_ctrl0: RfcObsCtrl0,
    rfc_obs_ctrl1: RfcObsCtrl1,
    rfc_obs_ctrl2: RfcObsCtrl2,
    _reserved95: [u8; 0x30],
    txfiltcfg: Txfiltcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
    #[inline(always)]
    pub const fn frmfilt0(&self) -> &Frmfilt0 {
        &self.frmfilt0
    }
    #[doc = "0x04 - The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
    #[inline(always)]
    pub const fn frmfilt1(&self) -> &Frmfilt1 {
        &self.frmfilt1
    }
    #[doc = "0x08 - Source address matching and pending bits"]
    #[inline(always)]
    pub const fn srcmatch(&self) -> &Srcmatch {
        &self.srcmatch
    }
    #[doc = "0x0c - Short address matching"]
    #[inline(always)]
    pub const fn srcshorten0(&self) -> &Srcshorten0 {
        &self.srcshorten0
    }
    #[doc = "0x10 - Short address matching"]
    #[inline(always)]
    pub const fn srcshorten1(&self) -> &Srcshorten1 {
        &self.srcshorten1
    }
    #[doc = "0x14 - Short address matching"]
    #[inline(always)]
    pub const fn srcshorten2(&self) -> &Srcshorten2 {
        &self.srcshorten2
    }
    #[doc = "0x18 - Extended address matching"]
    #[inline(always)]
    pub const fn srcexten0(&self) -> &Srcexten0 {
        &self.srcexten0
    }
    #[doc = "0x1c - Extended address matching"]
    #[inline(always)]
    pub const fn srcexten1(&self) -> &Srcexten1 {
        &self.srcexten1
    }
    #[doc = "0x20 - Extended address matching"]
    #[inline(always)]
    pub const fn srcexten2(&self) -> &Srcexten2 {
        &self.srcexten2
    }
    #[doc = "0x24 - Frame handling"]
    #[inline(always)]
    pub const fn frmctrl0(&self) -> &Frmctrl0 {
        &self.frmctrl0
    }
    #[doc = "0x28 - Frame handling"]
    #[inline(always)]
    pub const fn frmctrl1(&self) -> &Frmctrl1 {
        &self.frmctrl1
    }
    #[doc = "0x2c - RX enabling"]
    #[inline(always)]
    pub const fn rxenable(&self) -> &Rxenable {
        &self.rxenable
    }
    #[doc = "0x30 - RX enabling"]
    #[inline(always)]
    pub const fn rxmaskset(&self) -> &Rxmaskset {
        &self.rxmaskset
    }
    #[doc = "0x34 - RX disabling"]
    #[inline(always)]
    pub const fn rxmaskclr(&self) -> &Rxmaskclr {
        &self.rxmaskclr
    }
    #[doc = "0x38 - Crystal oscillator frequency tuning"]
    #[inline(always)]
    pub const fn freqtune(&self) -> &Freqtune {
        &self.freqtune
    }
    #[doc = "0x3c - Controls the RF frequency"]
    #[inline(always)]
    pub const fn freqctrl(&self) -> &Freqctrl {
        &self.freqctrl
    }
    #[doc = "0x40 - Controls the output power"]
    #[inline(always)]
    pub const fn txpower(&self) -> &Txpower {
        &self.txpower
    }
    #[doc = "0x44 - Controls the TX settings"]
    #[inline(always)]
    pub const fn txctrl(&self) -> &Txctrl {
        &self.txctrl
    }
    #[doc = "0x48 - Radio status register"]
    #[inline(always)]
    pub const fn fsmstat0(&self) -> &Fsmstat0 {
        &self.fsmstat0
    }
    #[doc = "0x4c - Radio status register"]
    #[inline(always)]
    pub const fn fsmstat1(&self) -> &Fsmstat1 {
        &self.fsmstat1
    }
    #[doc = "0x50 - FIFOP threshold"]
    #[inline(always)]
    pub const fn fifopctrl(&self) -> &Fifopctrl {
        &self.fifopctrl
    }
    #[doc = "0x54 - FSM options"]
    #[inline(always)]
    pub const fn fsmctrl(&self) -> &Fsmctrl {
        &self.fsmctrl
    }
    #[doc = "0x58 - CCA threshold"]
    #[inline(always)]
    pub const fn ccactrl0(&self) -> &Ccactrl0 {
        &self.ccactrl0
    }
    #[doc = "0x5c - Other CCA Options"]
    #[inline(always)]
    pub const fn ccactrl1(&self) -> &Ccactrl1 {
        &self.ccactrl1
    }
    #[doc = "0x60 - RSSI status register"]
    #[inline(always)]
    pub const fn rssi(&self) -> &Rssi {
        &self.rssi
    }
    #[doc = "0x64 - RSSI valid status register"]
    #[inline(always)]
    pub const fn rssistat(&self) -> &Rssistat {
        &self.rssistat
    }
    #[doc = "0x68 - First byte in RX FIFO"]
    #[inline(always)]
    pub const fn rxfirst(&self) -> &Rxfirst {
        &self.rxfirst
    }
    #[doc = "0x6c - Number of bytes in RX FIFO"]
    #[inline(always)]
    pub const fn rxfifocnt(&self) -> &Rxfifocnt {
        &self.rxfifocnt
    }
    #[doc = "0x70 - Number of bytes in TX FIFO"]
    #[inline(always)]
    pub const fn txfifocnt(&self) -> &Txfifocnt {
        &self.txfifocnt
    }
    #[doc = "0x74 - RX FIFO pointer"]
    #[inline(always)]
    pub const fn rxfirst_ptr(&self) -> &RxfirstPtr {
        &self.rxfirst_ptr
    }
    #[doc = "0x78 - RX FIFO pointer"]
    #[inline(always)]
    pub const fn rxlast_ptr(&self) -> &RxlastPtr {
        &self.rxlast_ptr
    }
    #[doc = "0x7c - RX FIFO pointer"]
    #[inline(always)]
    pub const fn rxp1_ptr(&self) -> &Rxp1Ptr {
        &self.rxp1_ptr
    }
    #[doc = "0x84 - TX FIFO pointer"]
    #[inline(always)]
    pub const fn txfirst_ptr(&self) -> &TxfirstPtr {
        &self.txfirst_ptr
    }
    #[doc = "0x88 - TX FIFO pointer"]
    #[inline(always)]
    pub const fn txlast_ptr(&self) -> &TxlastPtr {
        &self.txlast_ptr
    }
    #[doc = "0x8c - RF interrupt masks"]
    #[inline(always)]
    pub const fn rfirqm0(&self) -> &Rfirqm0 {
        &self.rfirqm0
    }
    #[doc = "0x90 - RF interrupt masks"]
    #[inline(always)]
    pub const fn rfirqm1(&self) -> &Rfirqm1 {
        &self.rfirqm1
    }
    #[doc = "0x94 - RF error interrupt mask"]
    #[inline(always)]
    pub const fn rferrm(&self) -> &Rferrm {
        &self.rferrm
    }
    #[doc = "0x9c - Random data"]
    #[inline(always)]
    pub const fn rfrnd(&self) -> &Rfrnd {
        &self.rfrnd
    }
    #[doc = "0xa0 - Controls modem"]
    #[inline(always)]
    pub const fn mdmctrl0(&self) -> &Mdmctrl0 {
        &self.mdmctrl0
    }
    #[doc = "0xa4 - Controls modem"]
    #[inline(always)]
    pub const fn mdmctrl1(&self) -> &Mdmctrl1 {
        &self.mdmctrl1
    }
    #[doc = "0xa8 - Estimated RF frequency offset"]
    #[inline(always)]
    pub const fn freqest(&self) -> &Freqest {
        &self.freqest
    }
    #[doc = "0xac - Tune receive section"]
    #[inline(always)]
    pub const fn rxctrl(&self) -> &Rxctrl {
        &self.rxctrl
    }
    #[doc = "0xb0 - Tune frequency synthesizer"]
    #[inline(always)]
    pub const fn fsctrl(&self) -> &Fsctrl {
        &self.fsctrl
    }
    #[doc = "0xb4 - Tune frequency calibration"]
    #[inline(always)]
    pub const fn fscal0(&self) -> &Fscal0 {
        &self.fscal0
    }
    #[doc = "0xb8 - Tune frequency calibration"]
    #[inline(always)]
    pub const fn fscal1(&self) -> &Fscal1 {
        &self.fscal1
    }
    #[doc = "0xbc - Tune frequency calibration"]
    #[inline(always)]
    pub const fn fscal2(&self) -> &Fscal2 {
        &self.fscal2
    }
    #[doc = "0xc0 - Tune frequency calibration"]
    #[inline(always)]
    pub const fn fscal3(&self) -> &Fscal3 {
        &self.fscal3
    }
    #[doc = "0xc4 - AGC dynamic range control"]
    #[inline(always)]
    pub const fn agcctrl0(&self) -> &Agcctrl0 {
        &self.agcctrl0
    }
    #[doc = "0xc8 - AGC reference level"]
    #[inline(always)]
    pub const fn agcctrl1(&self) -> &Agcctrl1 {
        &self.agcctrl1
    }
    #[doc = "0xcc - AGC gain override"]
    #[inline(always)]
    pub const fn agcctrl2(&self) -> &Agcctrl2 {
        &self.agcctrl2
    }
    #[doc = "0xd0 - AGC control"]
    #[inline(always)]
    pub const fn agcctrl3(&self) -> &Agcctrl3 {
        &self.agcctrl3
    }
    #[doc = "0xd4 - ADC tuning"]
    #[inline(always)]
    pub const fn adctest0(&self) -> &Adctest0 {
        &self.adctest0
    }
    #[doc = "0xd8 - ADC tuning"]
    #[inline(always)]
    pub const fn adctest1(&self) -> &Adctest1 {
        &self.adctest1
    }
    #[doc = "0xdc - ADC tuning"]
    #[inline(always)]
    pub const fn adctest2(&self) -> &Adctest2 {
        &self.adctest2
    }
    #[doc = "0xe0 - Test register for modem"]
    #[inline(always)]
    pub const fn mdmtest0(&self) -> &Mdmtest0 {
        &self.mdmtest0
    }
    #[doc = "0xe4 - Test Register for Modem"]
    #[inline(always)]
    pub const fn mdmtest1(&self) -> &Mdmtest1 {
        &self.mdmtest1
    }
    #[doc = "0xe8 - DAC override value"]
    #[inline(always)]
    pub const fn dactest0(&self) -> &Dactest0 {
        &self.dactest0
    }
    #[doc = "0xec - DAC override value"]
    #[inline(always)]
    pub const fn dactest1(&self) -> &Dactest1 {
        &self.dactest1
    }
    #[doc = "0xf0 - DAC test setting"]
    #[inline(always)]
    pub const fn dactest2(&self) -> &Dactest2 {
        &self.dactest2
    }
    #[doc = "0xf4 - Analog test control"]
    #[inline(always)]
    pub const fn atest(&self) -> &Atest {
        &self.atest
    }
    #[doc = "0xf8 - Override power-down register"]
    #[inline(always)]
    pub const fn ptest0(&self) -> &Ptest0 {
        &self.ptest0
    }
    #[doc = "0xfc - Override power-down register"]
    #[inline(always)]
    pub const fn ptest1(&self) -> &Ptest1 {
        &self.ptest1
    }
    #[doc = "0x100 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_0(&self) -> &Cspprog0 {
        &self.cspprog_0
    }
    #[doc = "0x104 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_1(&self) -> &Cspprog1 {
        &self.cspprog_1
    }
    #[doc = "0x108 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_2(&self) -> &Cspprog2 {
        &self.cspprog_2
    }
    #[doc = "0x10c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_3(&self) -> &Cspprog3 {
        &self.cspprog_3
    }
    #[doc = "0x110 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_4(&self) -> &Cspprog4 {
        &self.cspprog_4
    }
    #[doc = "0x114 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_5(&self) -> &Cspprog5 {
        &self.cspprog_5
    }
    #[doc = "0x118 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_6(&self) -> &Cspprog6 {
        &self.cspprog_6
    }
    #[doc = "0x11c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_7(&self) -> &Cspprog7 {
        &self.cspprog_7
    }
    #[doc = "0x120 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_8(&self) -> &Cspprog8 {
        &self.cspprog_8
    }
    #[doc = "0x124 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_9(&self) -> &Cspprog9 {
        &self.cspprog_9
    }
    #[doc = "0x128 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_10(&self) -> &Cspprog10 {
        &self.cspprog_10
    }
    #[doc = "0x12c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_11(&self) -> &Cspprog11 {
        &self.cspprog_11
    }
    #[doc = "0x130 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_12(&self) -> &Cspprog12 {
        &self.cspprog_12
    }
    #[doc = "0x134 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_13(&self) -> &Cspprog13 {
        &self.cspprog_13
    }
    #[doc = "0x138 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_14(&self) -> &Cspprog14 {
        &self.cspprog_14
    }
    #[doc = "0x13c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_15(&self) -> &Cspprog15 {
        &self.cspprog_15
    }
    #[doc = "0x140 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_16(&self) -> &Cspprog16 {
        &self.cspprog_16
    }
    #[doc = "0x144 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_17(&self) -> &Cspprog17 {
        &self.cspprog_17
    }
    #[doc = "0x148 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_18(&self) -> &Cspprog18 {
        &self.cspprog_18
    }
    #[doc = "0x14c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_19(&self) -> &Cspprog19 {
        &self.cspprog_19
    }
    #[doc = "0x150 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_20(&self) -> &Cspprog20 {
        &self.cspprog_20
    }
    #[doc = "0x154 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_21(&self) -> &Cspprog21 {
        &self.cspprog_21
    }
    #[doc = "0x158 - CSP program"]
    #[inline(always)]
    pub const fn cspprog_22(&self) -> &Cspprog22 {
        &self.cspprog_22
    }
    #[doc = "0x15c - CSP program"]
    #[inline(always)]
    pub const fn cspprog_23(&self) -> &Cspprog23 {
        &self.cspprog_23
    }
    #[doc = "0x180 - CSP control bit"]
    #[inline(always)]
    pub const fn cspctrl(&self) -> &Cspctrl {
        &self.cspctrl
    }
    #[doc = "0x184 - CSP status register"]
    #[inline(always)]
    pub const fn cspstat(&self) -> &Cspstat {
        &self.cspstat
    }
    #[doc = "0x188 - CSP X data register"]
    #[inline(always)]
    pub const fn cspx(&self) -> &Cspx {
        &self.cspx
    }
    #[doc = "0x18c - CSP Y data register"]
    #[inline(always)]
    pub const fn cspy(&self) -> &Cspy {
        &self.cspy
    }
    #[doc = "0x190 - CSP Z data register"]
    #[inline(always)]
    pub const fn cspz(&self) -> &Cspz {
        &self.cspz
    }
    #[doc = "0x194 - CSP T data register"]
    #[inline(always)]
    pub const fn cspt(&self) -> &Cspt {
        &self.cspt
    }
    #[doc = "0x1ac - RF observation mux control"]
    #[inline(always)]
    pub const fn rfc_obs_ctrl0(&self) -> &RfcObsCtrl0 {
        &self.rfc_obs_ctrl0
    }
    #[doc = "0x1b0 - RF observation mux control"]
    #[inline(always)]
    pub const fn rfc_obs_ctrl1(&self) -> &RfcObsCtrl1 {
        &self.rfc_obs_ctrl1
    }
    #[doc = "0x1b4 - RF observation mux control"]
    #[inline(always)]
    pub const fn rfc_obs_ctrl2(&self) -> &RfcObsCtrl2 {
        &self.rfc_obs_ctrl2
    }
    #[doc = "0x1e8 - TX filter configuration"]
    #[inline(always)]
    pub const fn txfiltcfg(&self) -> &Txfiltcfg {
        &self.txfiltcfg
    }
}
#[doc = "FRMFILT0 (rw) register accessor: The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`frmfilt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmfilt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmfilt0`]
module"]
#[doc(alias = "FRMFILT0")]
pub type Frmfilt0 = crate::Reg<frmfilt0::Frmfilt0Spec>;
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
pub mod frmfilt0;
#[doc = "FRMFILT1 (rw) register accessor: The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`frmfilt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmfilt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmfilt1`]
module"]
#[doc(alias = "FRMFILT1")]
pub type Frmfilt1 = crate::Reg<frmfilt1::Frmfilt1Spec>;
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
pub mod frmfilt1;
#[doc = "SRCMATCH (rw) register accessor: Source address matching and pending bits\n\nYou can [`read`](crate::Reg::read) this register and get [`srcmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcmatch`]
module"]
#[doc(alias = "SRCMATCH")]
pub type Srcmatch = crate::Reg<srcmatch::SrcmatchSpec>;
#[doc = "Source address matching and pending bits"]
pub mod srcmatch;
#[doc = "SRCSHORTEN0 (rw) register accessor: Short address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`srcshorten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcshorten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshorten0`]
module"]
#[doc(alias = "SRCSHORTEN0")]
pub type Srcshorten0 = crate::Reg<srcshorten0::Srcshorten0Spec>;
#[doc = "Short address matching"]
pub mod srcshorten0;
#[doc = "SRCSHORTEN1 (rw) register accessor: Short address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`srcshorten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcshorten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshorten1`]
module"]
#[doc(alias = "SRCSHORTEN1")]
pub type Srcshorten1 = crate::Reg<srcshorten1::Srcshorten1Spec>;
#[doc = "Short address matching"]
pub mod srcshorten1;
#[doc = "SRCSHORTEN2 (rw) register accessor: Short address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`srcshorten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcshorten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcshorten2`]
module"]
#[doc(alias = "SRCSHORTEN2")]
pub type Srcshorten2 = crate::Reg<srcshorten2::Srcshorten2Spec>;
#[doc = "Short address matching"]
pub mod srcshorten2;
#[doc = "SRCEXTEN0 (rw) register accessor: Extended address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`srcexten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcexten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcexten0`]
module"]
#[doc(alias = "SRCEXTEN0")]
pub type Srcexten0 = crate::Reg<srcexten0::Srcexten0Spec>;
#[doc = "Extended address matching"]
pub mod srcexten0;
#[doc = "SRCEXTEN1 (rw) register accessor: Extended address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`srcexten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcexten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcexten1`]
module"]
#[doc(alias = "SRCEXTEN1")]
pub type Srcexten1 = crate::Reg<srcexten1::Srcexten1Spec>;
#[doc = "Extended address matching"]
pub mod srcexten1;
#[doc = "SRCEXTEN2 (rw) register accessor: Extended address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`srcexten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcexten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcexten2`]
module"]
#[doc(alias = "SRCEXTEN2")]
pub type Srcexten2 = crate::Reg<srcexten2::Srcexten2Spec>;
#[doc = "Extended address matching"]
pub mod srcexten2;
#[doc = "FRMCTRL0 (rw) register accessor: Frame handling\n\nYou can [`read`](crate::Reg::read) this register and get [`frmctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmctrl0`]
module"]
#[doc(alias = "FRMCTRL0")]
pub type Frmctrl0 = crate::Reg<frmctrl0::Frmctrl0Spec>;
#[doc = "Frame handling"]
pub mod frmctrl0;
#[doc = "FRMCTRL1 (rw) register accessor: Frame handling\n\nYou can [`read`](crate::Reg::read) this register and get [`frmctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmctrl1`]
module"]
#[doc(alias = "FRMCTRL1")]
pub type Frmctrl1 = crate::Reg<frmctrl1::Frmctrl1Spec>;
#[doc = "Frame handling"]
pub mod frmctrl1;
#[doc = "RXENABLE (r) register accessor: RX enabling\n\nYou can [`read`](crate::Reg::read) this register and get [`rxenable::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxenable`]
module"]
#[doc(alias = "RXENABLE")]
pub type Rxenable = crate::Reg<rxenable::RxenableSpec>;
#[doc = "RX enabling"]
pub mod rxenable;
#[doc = "RXMASKSET (rw) register accessor: RX enabling\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmaskset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmaskset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaskset`]
module"]
#[doc(alias = "RXMASKSET")]
pub type Rxmaskset = crate::Reg<rxmaskset::RxmasksetSpec>;
#[doc = "RX enabling"]
pub mod rxmaskset;
#[doc = "RXMASKCLR (rw) register accessor: RX disabling\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmaskclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmaskclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaskclr`]
module"]
#[doc(alias = "RXMASKCLR")]
pub type Rxmaskclr = crate::Reg<rxmaskclr::RxmaskclrSpec>;
#[doc = "RX disabling"]
pub mod rxmaskclr;
#[doc = "FREQTUNE (rw) register accessor: Crystal oscillator frequency tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`freqtune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqtune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqtune`]
module"]
#[doc(alias = "FREQTUNE")]
pub type Freqtune = crate::Reg<freqtune::FreqtuneSpec>;
#[doc = "Crystal oscillator frequency tuning"]
pub mod freqtune;
#[doc = "FREQCTRL (rw) register accessor: Controls the RF frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`freqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqctrl`]
module"]
#[doc(alias = "FREQCTRL")]
pub type Freqctrl = crate::Reg<freqctrl::FreqctrlSpec>;
#[doc = "Controls the RF frequency"]
pub mod freqctrl;
#[doc = "TXPOWER (rw) register accessor: Controls the output power\n\nYou can [`read`](crate::Reg::read) this register and get [`txpower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpower`]
module"]
#[doc(alias = "TXPOWER")]
pub type Txpower = crate::Reg<txpower::TxpowerSpec>;
#[doc = "Controls the output power"]
pub mod txpower;
#[doc = "TXCTRL (rw) register accessor: Controls the TX settings\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctrl`]
module"]
#[doc(alias = "TXCTRL")]
pub type Txctrl = crate::Reg<txctrl::TxctrlSpec>;
#[doc = "Controls the TX settings"]
pub mod txctrl;
#[doc = "FSMSTAT0 (r) register accessor: Radio status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmstat0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmstat0`]
module"]
#[doc(alias = "FSMSTAT0")]
pub type Fsmstat0 = crate::Reg<fsmstat0::Fsmstat0Spec>;
#[doc = "Radio status register"]
pub mod fsmstat0;
#[doc = "FSMSTAT1 (r) register accessor: Radio status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmstat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmstat1`]
module"]
#[doc(alias = "FSMSTAT1")]
pub type Fsmstat1 = crate::Reg<fsmstat1::Fsmstat1Spec>;
#[doc = "Radio status register"]
pub mod fsmstat1;
#[doc = "FIFOPCTRL (rw) register accessor: FIFOP threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`fifopctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifopctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifopctrl`]
module"]
#[doc(alias = "FIFOPCTRL")]
pub type Fifopctrl = crate::Reg<fifopctrl::FifopctrlSpec>;
#[doc = "FIFOP threshold"]
pub mod fifopctrl;
#[doc = "FSMCTRL (rw) register accessor: FSM options\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsmctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmctrl`]
module"]
#[doc(alias = "FSMCTRL")]
pub type Fsmctrl = crate::Reg<fsmctrl::FsmctrlSpec>;
#[doc = "FSM options"]
pub mod fsmctrl;
#[doc = "CCACTRL0 (rw) register accessor: CCA threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`ccactrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccactrl0`]
module"]
#[doc(alias = "CCACTRL0")]
pub type Ccactrl0 = crate::Reg<ccactrl0::Ccactrl0Spec>;
#[doc = "CCA threshold"]
pub mod ccactrl0;
#[doc = "CCACTRL1 (rw) register accessor: Other CCA Options\n\nYou can [`read`](crate::Reg::read) this register and get [`ccactrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccactrl1`]
module"]
#[doc(alias = "CCACTRL1")]
pub type Ccactrl1 = crate::Reg<ccactrl1::Ccactrl1Spec>;
#[doc = "Other CCA Options"]
pub mod ccactrl1;
#[doc = "RSSI (r) register accessor: RSSI status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rssi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rssi`]
module"]
#[doc(alias = "RSSI")]
pub type Rssi = crate::Reg<rssi::RssiSpec>;
#[doc = "RSSI status register"]
pub mod rssi;
#[doc = "RSSISTAT (r) register accessor: RSSI valid status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rssistat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rssistat`]
module"]
#[doc(alias = "RSSISTAT")]
pub type Rssistat = crate::Reg<rssistat::RssistatSpec>;
#[doc = "RSSI valid status register"]
pub mod rssistat;
#[doc = "RXFIRST (r) register accessor: First byte in RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfirst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfirst`]
module"]
#[doc(alias = "RXFIRST")]
pub type Rxfirst = crate::Reg<rxfirst::RxfirstSpec>;
#[doc = "First byte in RX FIFO"]
pub mod rxfirst;
#[doc = "RXFIFOCNT (r) register accessor: Number of bytes in RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifocnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifocnt`]
module"]
#[doc(alias = "RXFIFOCNT")]
pub type Rxfifocnt = crate::Reg<rxfifocnt::RxfifocntSpec>;
#[doc = "Number of bytes in RX FIFO"]
pub mod rxfifocnt;
#[doc = "TXFIFOCNT (r) register accessor: Number of bytes in TX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifocnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifocnt`]
module"]
#[doc(alias = "TXFIFOCNT")]
pub type Txfifocnt = crate::Reg<txfifocnt::TxfifocntSpec>;
#[doc = "Number of bytes in TX FIFO"]
pub mod txfifocnt;
#[doc = "RXFIRST_PTR (r) register accessor: RX FIFO pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfirst_ptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfirst_ptr`]
module"]
#[doc(alias = "RXFIRST_PTR")]
pub type RxfirstPtr = crate::Reg<rxfirst_ptr::RxfirstPtrSpec>;
#[doc = "RX FIFO pointer"]
pub mod rxfirst_ptr;
#[doc = "RXLAST_PTR (r) register accessor: RX FIFO pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlast_ptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlast_ptr`]
module"]
#[doc(alias = "RXLAST_PTR")]
pub type RxlastPtr = crate::Reg<rxlast_ptr::RxlastPtrSpec>;
#[doc = "RX FIFO pointer"]
pub mod rxlast_ptr;
#[doc = "RXP1_PTR (r) register accessor: RX FIFO pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`rxp1_ptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxp1_ptr`]
module"]
#[doc(alias = "RXP1_PTR")]
pub type Rxp1Ptr = crate::Reg<rxp1_ptr::Rxp1PtrSpec>;
#[doc = "RX FIFO pointer"]
pub mod rxp1_ptr;
#[doc = "TXFIRST_PTR (r) register accessor: TX FIFO pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`txfirst_ptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfirst_ptr`]
module"]
#[doc(alias = "TXFIRST_PTR")]
pub type TxfirstPtr = crate::Reg<txfirst_ptr::TxfirstPtrSpec>;
#[doc = "TX FIFO pointer"]
pub mod txfirst_ptr;
#[doc = "TXLAST_PTR (r) register accessor: TX FIFO pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`txlast_ptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlast_ptr`]
module"]
#[doc(alias = "TXLAST_PTR")]
pub type TxlastPtr = crate::Reg<txlast_ptr::TxlastPtrSpec>;
#[doc = "TX FIFO pointer"]
pub mod txlast_ptr;
#[doc = "RFIRQM0 (rw) register accessor: RF interrupt masks\n\nYou can [`read`](crate::Reg::read) this register and get [`rfirqm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfirqm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfirqm0`]
module"]
#[doc(alias = "RFIRQM0")]
pub type Rfirqm0 = crate::Reg<rfirqm0::Rfirqm0Spec>;
#[doc = "RF interrupt masks"]
pub mod rfirqm0;
#[doc = "RFIRQM1 (rw) register accessor: RF interrupt masks\n\nYou can [`read`](crate::Reg::read) this register and get [`rfirqm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfirqm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfirqm1`]
module"]
#[doc(alias = "RFIRQM1")]
pub type Rfirqm1 = crate::Reg<rfirqm1::Rfirqm1Spec>;
#[doc = "RF interrupt masks"]
pub mod rfirqm1;
#[doc = "RFERRM (rw) register accessor: RF error interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`rferrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rferrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rferrm`]
module"]
#[doc(alias = "RFERRM")]
pub type Rferrm = crate::Reg<rferrm::RferrmSpec>;
#[doc = "RF error interrupt mask"]
pub mod rferrm;
#[doc = "RFRND (r) register accessor: Random data\n\nYou can [`read`](crate::Reg::read) this register and get [`rfrnd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfrnd`]
module"]
#[doc(alias = "RFRND")]
pub type Rfrnd = crate::Reg<rfrnd::RfrndSpec>;
#[doc = "Random data"]
pub mod rfrnd;
#[doc = "MDMCTRL0 (rw) register accessor: Controls modem\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmctrl0`]
module"]
#[doc(alias = "MDMCTRL0")]
pub type Mdmctrl0 = crate::Reg<mdmctrl0::Mdmctrl0Spec>;
#[doc = "Controls modem"]
pub mod mdmctrl0;
#[doc = "MDMCTRL1 (rw) register accessor: Controls modem\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmctrl1`]
module"]
#[doc(alias = "MDMCTRL1")]
pub type Mdmctrl1 = crate::Reg<mdmctrl1::Mdmctrl1Spec>;
#[doc = "Controls modem"]
pub mod mdmctrl1;
#[doc = "FREQEST (r) register accessor: Estimated RF frequency offset\n\nYou can [`read`](crate::Reg::read) this register and get [`freqest::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqest`]
module"]
#[doc(alias = "FREQEST")]
pub type Freqest = crate::Reg<freqest::FreqestSpec>;
#[doc = "Estimated RF frequency offset"]
pub mod freqest;
#[doc = "RXCTRL (rw) register accessor: Tune receive section\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctrl`]
module"]
#[doc(alias = "RXCTRL")]
pub type Rxctrl = crate::Reg<rxctrl::RxctrlSpec>;
#[doc = "Tune receive section"]
pub mod rxctrl;
#[doc = "FSCTRL (rw) register accessor: Tune frequency synthesizer\n\nYou can [`read`](crate::Reg::read) this register and get [`fsctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsctrl`]
module"]
#[doc(alias = "FSCTRL")]
pub type Fsctrl = crate::Reg<fsctrl::FsctrlSpec>;
#[doc = "Tune frequency synthesizer"]
pub mod fsctrl;
#[doc = "FSCAL0 (rw) register accessor: Tune frequency calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`fscal0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fscal0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscal0`]
module"]
#[doc(alias = "FSCAL0")]
pub type Fscal0 = crate::Reg<fscal0::Fscal0Spec>;
#[doc = "Tune frequency calibration"]
pub mod fscal0;
#[doc = "FSCAL1 (rw) register accessor: Tune frequency calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`fscal1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fscal1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscal1`]
module"]
#[doc(alias = "FSCAL1")]
pub type Fscal1 = crate::Reg<fscal1::Fscal1Spec>;
#[doc = "Tune frequency calibration"]
pub mod fscal1;
#[doc = "FSCAL2 (rw) register accessor: Tune frequency calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`fscal2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fscal2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscal2`]
module"]
#[doc(alias = "FSCAL2")]
pub type Fscal2 = crate::Reg<fscal2::Fscal2Spec>;
#[doc = "Tune frequency calibration"]
pub mod fscal2;
#[doc = "FSCAL3 (rw) register accessor: Tune frequency calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`fscal3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fscal3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscal3`]
module"]
#[doc(alias = "FSCAL3")]
pub type Fscal3 = crate::Reg<fscal3::Fscal3Spec>;
#[doc = "Tune frequency calibration"]
pub mod fscal3;
#[doc = "AGCCTRL0 (rw) register accessor: AGC dynamic range control\n\nYou can [`read`](crate::Reg::read) this register and get [`agcctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agcctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agcctrl0`]
module"]
#[doc(alias = "AGCCTRL0")]
pub type Agcctrl0 = crate::Reg<agcctrl0::Agcctrl0Spec>;
#[doc = "AGC dynamic range control"]
pub mod agcctrl0;
#[doc = "AGCCTRL1 (rw) register accessor: AGC reference level\n\nYou can [`read`](crate::Reg::read) this register and get [`agcctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agcctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agcctrl1`]
module"]
#[doc(alias = "AGCCTRL1")]
pub type Agcctrl1 = crate::Reg<agcctrl1::Agcctrl1Spec>;
#[doc = "AGC reference level"]
pub mod agcctrl1;
#[doc = "AGCCTRL2 (rw) register accessor: AGC gain override\n\nYou can [`read`](crate::Reg::read) this register and get [`agcctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agcctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agcctrl2`]
module"]
#[doc(alias = "AGCCTRL2")]
pub type Agcctrl2 = crate::Reg<agcctrl2::Agcctrl2Spec>;
#[doc = "AGC gain override"]
pub mod agcctrl2;
#[doc = "AGCCTRL3 (rw) register accessor: AGC control\n\nYou can [`read`](crate::Reg::read) this register and get [`agcctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agcctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agcctrl3`]
module"]
#[doc(alias = "AGCCTRL3")]
pub type Agcctrl3 = crate::Reg<agcctrl3::Agcctrl3Spec>;
#[doc = "AGC control"]
pub mod agcctrl3;
#[doc = "ADCTEST0 (rw) register accessor: ADC tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`adctest0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adctest0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctest0`]
module"]
#[doc(alias = "ADCTEST0")]
pub type Adctest0 = crate::Reg<adctest0::Adctest0Spec>;
#[doc = "ADC tuning"]
pub mod adctest0;
#[doc = "ADCTEST1 (rw) register accessor: ADC tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`adctest1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adctest1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctest1`]
module"]
#[doc(alias = "ADCTEST1")]
pub type Adctest1 = crate::Reg<adctest1::Adctest1Spec>;
#[doc = "ADC tuning"]
pub mod adctest1;
#[doc = "ADCTEST2 (rw) register accessor: ADC tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`adctest2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adctest2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctest2`]
module"]
#[doc(alias = "ADCTEST2")]
pub type Adctest2 = crate::Reg<adctest2::Adctest2Spec>;
#[doc = "ADC tuning"]
pub mod adctest2;
#[doc = "MDMTEST0 (rw) register accessor: Test register for modem\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmtest0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmtest0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmtest0`]
module"]
#[doc(alias = "MDMTEST0")]
pub type Mdmtest0 = crate::Reg<mdmtest0::Mdmtest0Spec>;
#[doc = "Test register for modem"]
pub mod mdmtest0;
#[doc = "MDMTEST1 (rw) register accessor: Test Register for Modem\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmtest1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmtest1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmtest1`]
module"]
#[doc(alias = "MDMTEST1")]
pub type Mdmtest1 = crate::Reg<mdmtest1::Mdmtest1Spec>;
#[doc = "Test Register for Modem"]
pub mod mdmtest1;
#[doc = "DACTEST0 (rw) register accessor: DAC override value\n\nYou can [`read`](crate::Reg::read) this register and get [`dactest0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dactest0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dactest0`]
module"]
#[doc(alias = "DACTEST0")]
pub type Dactest0 = crate::Reg<dactest0::Dactest0Spec>;
#[doc = "DAC override value"]
pub mod dactest0;
#[doc = "DACTEST1 (rw) register accessor: DAC override value\n\nYou can [`read`](crate::Reg::read) this register and get [`dactest1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dactest1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dactest1`]
module"]
#[doc(alias = "DACTEST1")]
pub type Dactest1 = crate::Reg<dactest1::Dactest1Spec>;
#[doc = "DAC override value"]
pub mod dactest1;
#[doc = "DACTEST2 (rw) register accessor: DAC test setting\n\nYou can [`read`](crate::Reg::read) this register and get [`dactest2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dactest2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dactest2`]
module"]
#[doc(alias = "DACTEST2")]
pub type Dactest2 = crate::Reg<dactest2::Dactest2Spec>;
#[doc = "DAC test setting"]
pub mod dactest2;
#[doc = "ATEST (rw) register accessor: Analog test control\n\nYou can [`read`](crate::Reg::read) this register and get [`atest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atest`]
module"]
#[doc(alias = "ATEST")]
pub type Atest = crate::Reg<atest::AtestSpec>;
#[doc = "Analog test control"]
pub mod atest;
#[doc = "PTEST0 (rw) register accessor: Override power-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptest0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptest0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptest0`]
module"]
#[doc(alias = "PTEST0")]
pub type Ptest0 = crate::Reg<ptest0::Ptest0Spec>;
#[doc = "Override power-down register"]
pub mod ptest0;
#[doc = "PTEST1 (rw) register accessor: Override power-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptest1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptest1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptest1`]
module"]
#[doc(alias = "PTEST1")]
pub type Ptest1 = crate::Reg<ptest1::Ptest1Spec>;
#[doc = "Override power-down register"]
pub mod ptest1;
#[doc = "CSPPROG_0 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_0`]
module"]
#[doc(alias = "CSPPROG_0")]
pub type Cspprog0 = crate::Reg<cspprog_0::Cspprog0Spec>;
#[doc = "CSP program"]
pub mod cspprog_0;
#[doc = "CSPPROG_1 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_1`]
module"]
#[doc(alias = "CSPPROG_1")]
pub type Cspprog1 = crate::Reg<cspprog_1::Cspprog1Spec>;
#[doc = "CSP program"]
pub mod cspprog_1;
#[doc = "CSPPROG_2 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_2`]
module"]
#[doc(alias = "CSPPROG_2")]
pub type Cspprog2 = crate::Reg<cspprog_2::Cspprog2Spec>;
#[doc = "CSP program"]
pub mod cspprog_2;
#[doc = "CSPPROG_3 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_3`]
module"]
#[doc(alias = "CSPPROG_3")]
pub type Cspprog3 = crate::Reg<cspprog_3::Cspprog3Spec>;
#[doc = "CSP program"]
pub mod cspprog_3;
#[doc = "CSPPROG_4 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_4`]
module"]
#[doc(alias = "CSPPROG_4")]
pub type Cspprog4 = crate::Reg<cspprog_4::Cspprog4Spec>;
#[doc = "CSP program"]
pub mod cspprog_4;
#[doc = "CSPPROG_5 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_5`]
module"]
#[doc(alias = "CSPPROG_5")]
pub type Cspprog5 = crate::Reg<cspprog_5::Cspprog5Spec>;
#[doc = "CSP program"]
pub mod cspprog_5;
#[doc = "CSPPROG_6 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_6`]
module"]
#[doc(alias = "CSPPROG_6")]
pub type Cspprog6 = crate::Reg<cspprog_6::Cspprog6Spec>;
#[doc = "CSP program"]
pub mod cspprog_6;
#[doc = "CSPPROG_7 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_7`]
module"]
#[doc(alias = "CSPPROG_7")]
pub type Cspprog7 = crate::Reg<cspprog_7::Cspprog7Spec>;
#[doc = "CSP program"]
pub mod cspprog_7;
#[doc = "CSPPROG_8 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_8`]
module"]
#[doc(alias = "CSPPROG_8")]
pub type Cspprog8 = crate::Reg<cspprog_8::Cspprog8Spec>;
#[doc = "CSP program"]
pub mod cspprog_8;
#[doc = "CSPPROG_9 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_9`]
module"]
#[doc(alias = "CSPPROG_9")]
pub type Cspprog9 = crate::Reg<cspprog_9::Cspprog9Spec>;
#[doc = "CSP program"]
pub mod cspprog_9;
#[doc = "CSPPROG_10 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_10`]
module"]
#[doc(alias = "CSPPROG_10")]
pub type Cspprog10 = crate::Reg<cspprog_10::Cspprog10Spec>;
#[doc = "CSP program"]
pub mod cspprog_10;
#[doc = "CSPPROG_11 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_11`]
module"]
#[doc(alias = "CSPPROG_11")]
pub type Cspprog11 = crate::Reg<cspprog_11::Cspprog11Spec>;
#[doc = "CSP program"]
pub mod cspprog_11;
#[doc = "CSPPROG_12 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_12`]
module"]
#[doc(alias = "CSPPROG_12")]
pub type Cspprog12 = crate::Reg<cspprog_12::Cspprog12Spec>;
#[doc = "CSP program"]
pub mod cspprog_12;
#[doc = "CSPPROG_13 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_13`]
module"]
#[doc(alias = "CSPPROG_13")]
pub type Cspprog13 = crate::Reg<cspprog_13::Cspprog13Spec>;
#[doc = "CSP program"]
pub mod cspprog_13;
#[doc = "CSPPROG_14 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_14`]
module"]
#[doc(alias = "CSPPROG_14")]
pub type Cspprog14 = crate::Reg<cspprog_14::Cspprog14Spec>;
#[doc = "CSP program"]
pub mod cspprog_14;
#[doc = "CSPPROG_15 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_15`]
module"]
#[doc(alias = "CSPPROG_15")]
pub type Cspprog15 = crate::Reg<cspprog_15::Cspprog15Spec>;
#[doc = "CSP program"]
pub mod cspprog_15;
#[doc = "CSPPROG_16 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_16`]
module"]
#[doc(alias = "CSPPROG_16")]
pub type Cspprog16 = crate::Reg<cspprog_16::Cspprog16Spec>;
#[doc = "CSP program"]
pub mod cspprog_16;
#[doc = "CSPPROG_17 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_17`]
module"]
#[doc(alias = "CSPPROG_17")]
pub type Cspprog17 = crate::Reg<cspprog_17::Cspprog17Spec>;
#[doc = "CSP program"]
pub mod cspprog_17;
#[doc = "CSPPROG_18 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_18`]
module"]
#[doc(alias = "CSPPROG_18")]
pub type Cspprog18 = crate::Reg<cspprog_18::Cspprog18Spec>;
#[doc = "CSP program"]
pub mod cspprog_18;
#[doc = "CSPPROG_19 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_19`]
module"]
#[doc(alias = "CSPPROG_19")]
pub type Cspprog19 = crate::Reg<cspprog_19::Cspprog19Spec>;
#[doc = "CSP program"]
pub mod cspprog_19;
#[doc = "CSPPROG_20 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_20`]
module"]
#[doc(alias = "CSPPROG_20")]
pub type Cspprog20 = crate::Reg<cspprog_20::Cspprog20Spec>;
#[doc = "CSP program"]
pub mod cspprog_20;
#[doc = "CSPPROG_21 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_21`]
module"]
#[doc(alias = "CSPPROG_21")]
pub type Cspprog21 = crate::Reg<cspprog_21::Cspprog21Spec>;
#[doc = "CSP program"]
pub mod cspprog_21;
#[doc = "CSPPROG_22 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_22`]
module"]
#[doc(alias = "CSPPROG_22")]
pub type Cspprog22 = crate::Reg<cspprog_22::Cspprog22Spec>;
#[doc = "CSP program"]
pub mod cspprog_22;
#[doc = "CSPPROG_23 (r) register accessor: CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspprog_23`]
module"]
#[doc(alias = "CSPPROG_23")]
pub type Cspprog23 = crate::Reg<cspprog_23::Cspprog23Spec>;
#[doc = "CSP program"]
pub mod cspprog_23;
#[doc = "CSPCTRL (rw) register accessor: CSP control bit\n\nYou can [`read`](crate::Reg::read) this register and get [`cspctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cspctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspctrl`]
module"]
#[doc(alias = "CSPCTRL")]
pub type Cspctrl = crate::Reg<cspctrl::CspctrlSpec>;
#[doc = "CSP control bit"]
pub mod cspctrl;
#[doc = "CSPSTAT (r) register accessor: CSP status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspstat`]
module"]
#[doc(alias = "CSPSTAT")]
pub type Cspstat = crate::Reg<cspstat::CspstatSpec>;
#[doc = "CSP status register"]
pub mod cspstat;
#[doc = "CSPX (r) register accessor: CSP X data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspx`]
module"]
#[doc(alias = "CSPX")]
pub type Cspx = crate::Reg<cspx::CspxSpec>;
#[doc = "CSP X data register"]
pub mod cspx;
#[doc = "CSPY (r) register accessor: CSP Y data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspy`]
module"]
#[doc(alias = "CSPY")]
pub type Cspy = crate::Reg<cspy::CspySpec>;
#[doc = "CSP Y data register"]
pub mod cspy;
#[doc = "CSPZ (r) register accessor: CSP Z data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspz`]
module"]
#[doc(alias = "CSPZ")]
pub type Cspz = crate::Reg<cspz::CspzSpec>;
#[doc = "CSP Z data register"]
pub mod cspz;
#[doc = "CSPT (r) register accessor: CSP T data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspt`]
module"]
#[doc(alias = "CSPT")]
pub type Cspt = crate::Reg<cspt::CsptSpec>;
#[doc = "CSP T data register"]
pub mod cspt;
#[doc = "RFC_OBS_CTRL0 (rw) register accessor: RF observation mux control\n\nYou can [`read`](crate::Reg::read) this register and get [`rfc_obs_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfc_obs_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc_obs_ctrl0`]
module"]
#[doc(alias = "RFC_OBS_CTRL0")]
pub type RfcObsCtrl0 = crate::Reg<rfc_obs_ctrl0::RfcObsCtrl0Spec>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl0;
#[doc = "RFC_OBS_CTRL1 (rw) register accessor: RF observation mux control\n\nYou can [`read`](crate::Reg::read) this register and get [`rfc_obs_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfc_obs_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc_obs_ctrl1`]
module"]
#[doc(alias = "RFC_OBS_CTRL1")]
pub type RfcObsCtrl1 = crate::Reg<rfc_obs_ctrl1::RfcObsCtrl1Spec>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl1;
#[doc = "RFC_OBS_CTRL2 (rw) register accessor: RF observation mux control\n\nYou can [`read`](crate::Reg::read) this register and get [`rfc_obs_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfc_obs_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc_obs_ctrl2`]
module"]
#[doc(alias = "RFC_OBS_CTRL2")]
pub type RfcObsCtrl2 = crate::Reg<rfc_obs_ctrl2::RfcObsCtrl2Spec>;
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl2;
#[doc = "TXFILTCFG (rw) register accessor: TX filter configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txfiltcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfiltcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfiltcfg`]
module"]
#[doc(alias = "TXFILTCFG")]
pub type Txfiltcfg = crate::Reg<txfiltcfg::TxfiltcfgSpec>;
#[doc = "TX filter configuration"]
pub mod txfiltcfg;
