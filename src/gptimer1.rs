#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    tamr: Tamr,
    tbmr: Tbmr,
    ctl: Ctl,
    sync: Sync,
    _reserved5: [u8; 0x04],
    imr: Imr,
    ris: Ris,
    mis: Mis,
    icr: Icr,
    tailr: Tailr,
    tbilr: Tbilr,
    tamatchr: Tamatchr,
    tbmatchr: Tbmatchr,
    tapr: Tapr,
    tbpr: Tbpr,
    tapmr: Tapmr,
    tbpmr: Tbpmr,
    tar: Tar,
    tbr: Tbr,
    tav: Tav,
    tbv: Tbv,
    _reserved21: [u8; 0x04],
    taps: Taps,
    tbps: Tbps,
    tapv: Tapv,
    tbpv: Tbpv,
    _reserved25: [u8; 0x0f54],
    pp: Pp,
}
impl RegisterBlock {
    #[doc = "0x00 - GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers)."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored."]
    #[inline(always)]
    pub const fn tamr(&self) -> &Tamr {
        &self.tamr
    }
    #[doc = "0x08 - GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B."]
    #[inline(always)]
    pub const fn tbmr(&self) -> &Tbmr {
        &self.tbmr
    }
    #[doc = "0x0c - GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall."]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x10 - GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers."]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x18 - GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it."]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1c - GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x20 - GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x24 - GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers."]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x28 - GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR."]
    #[inline(always)]
    pub const fn tailr(&self) -> &Tailr {
        &self.tailr
    }
    #[doc = "0x2c - GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the load value. Bits \\[31:16\\]
are reserved in both cases."]
    #[inline(always)]
    pub const fn tbilr(&self) -> &Tbilr {
        &self.tbilr
    }
    #[doc = "0x30 - GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR."]
    #[inline(always)]
    pub const fn tamatchr(&self) -> &Tamatchr {
        &self.tamatchr
    }
    #[doc = "0x34 - PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the match value. Bits \\[31:16\\]
are reserved in both cases."]
    #[inline(always)]
    pub const fn tbmatchr(&self) -> &Tbmatchr {
        &self.tbmatchr
    }
    #[doc = "0x38 - GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
    #[inline(always)]
    pub const fn tapr(&self) -> &Tapr {
        &self.tapr
    }
    #[doc = "0x3c - GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
    #[inline(always)]
    pub const fn tbpr(&self) -> &Tbpr {
        &self.tbpr
    }
    #[doc = "0x40 - GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
    #[inline(always)]
    pub const fn tapmr(&self) -> &Tapmr {
        &self.tapmr
    }
    #[doc = "0x44 - GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
    #[inline(always)]
    pub const fn tbpmr(&self) -> &Tbpmr {
        &self.tbpmr
    }
    #[doc = "0x48 - GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\]
contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TAV register."]
    #[inline(always)]
    pub const fn tar(&self) -> &Tar {
        &self.tar
    }
    #[doc = "0x4c - GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\]
contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TBV register."]
    #[inline(always)]
    pub const fn tbr(&self) -> &Tbr {
        &self.tbr
    }
    #[doc = "0x50 - GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0."]
    #[inline(always)]
    pub const fn tav(&self) -> &Tav {
        &self.tav
    }
    #[doc = "0x54 - GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0."]
    #[inline(always)]
    pub const fn tbv(&self) -> &Tbv {
        &self.tbv
    }
    #[doc = "0x5c - GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
    #[inline(always)]
    pub const fn taps(&self) -> &Taps {
        &self.taps
    }
    #[doc = "0x60 - GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
    #[inline(always)]
    pub const fn tbps(&self) -> &Tbps {
        &self.tbps
    }
    #[doc = "0x64 - GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
    #[inline(always)]
    pub const fn tapv(&self) -> &Tapv {
        &self.tapv
    }
    #[doc = "0x68 - GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
    #[inline(always)]
    pub const fn tbpv(&self) -> &Tbpv {
        &self.tbpv
    }
    #[doc = "0xfc0 - GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module."]
    #[inline(always)]
    pub const fn pp(&self) -> &Pp {
        &self.pp
    }
}
#[doc = "CFG (rw) register accessor: GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers).\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers)."]
pub mod cfg;
#[doc = "TAMR (rw) register accessor: GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored.\n\nYou can [`read`](crate::Reg::read) this register and get [`tamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamr`]
module"]
#[doc(alias = "TAMR")]
pub type Tamr = crate::Reg<tamr::TamrSpec>;
#[doc = "GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored."]
pub mod tamr;
#[doc = "TBMR (rw) register accessor: GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbmr`]
module"]
#[doc(alias = "TBMR")]
pub type Tbmr = crate::Reg<tbmr::TbmrSpec>;
#[doc = "GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B."]
pub mod tbmr;
#[doc = "CTL (rw) register accessor: GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall."]
pub mod ctl;
#[doc = "SYNC (rw) register accessor: GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers.\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
#[doc(alias = "SYNC")]
pub type Sync = crate::Reg<sync::SyncSpec>;
#[doc = "GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers."]
pub mod sync;
#[doc = "IMR (rw) register accessor: GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it.\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it."]
pub mod imr;
#[doc = "RIS (r) register accessor: GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR.\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR."]
pub mod ris;
#[doc = "MIS (r) register accessor: GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR.\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR."]
pub mod mis;
#[doc = "ICR (rw) register accessor: GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers."]
pub mod icr;
#[doc = "TAILR (rw) register accessor: GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tailr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tailr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tailr`]
module"]
#[doc(alias = "TAILR")]
pub type Tailr = crate::Reg<tailr::TailrSpec>;
#[doc = "GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR."]
pub mod tailr;
#[doc = "TBILR (rw) register accessor: GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the load value. Bits \\[31:16\\]
are reserved in both cases.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbilr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbilr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbilr`]
module"]
#[doc(alias = "TBILR")]
pub type Tbilr = crate::Reg<tbilr::TbilrSpec>;
#[doc = "GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the load value. Bits \\[31:16\\]
are reserved in both cases."]
pub mod tbilr;
#[doc = "TAMATCHR (rw) register accessor: GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tamatchr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamatchr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamatchr`]
module"]
#[doc(alias = "TAMATCHR")]
pub type Tamatchr = crate::Reg<tamatchr::TamatchrSpec>;
#[doc = "GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR."]
pub mod tamatchr;
#[doc = "TBMATCHR (rw) register accessor: PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the match value. Bits \\[31:16\\]
are reserved in both cases.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbmatchr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbmatchr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbmatchr`]
module"]
#[doc(alias = "TBMATCHR")]
pub type Tbmatchr = crate::Reg<tbmatchr::TbmatchrSpec>;
#[doc = "PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the match value. Bits \\[31:16\\]
are reserved in both cases."]
pub mod tbmatchr;
#[doc = "TAPR (rw) register accessor: GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`tapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tapr`]
module"]
#[doc(alias = "TAPR")]
pub type Tapr = crate::Reg<tapr::TaprSpec>;
#[doc = "GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
pub mod tapr;
#[doc = "TBPR (rw) register accessor: GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbpr`]
module"]
#[doc(alias = "TBPR")]
pub type Tbpr = crate::Reg<tbpr::TbprSpec>;
#[doc = "GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
pub mod tbpr;
#[doc = "TAPMR (rw) register accessor: GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`tapmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tapmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tapmr`]
module"]
#[doc(alias = "TAPMR")]
pub type Tapmr = crate::Reg<tapmr::TapmrSpec>;
#[doc = "GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
pub mod tapmr;
#[doc = "TBPMR (rw) register accessor: GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbpmr`]
module"]
#[doc(alias = "TBPMR")]
pub type Tbpmr = crate::Reg<tbpmr::TbpmrSpec>;
#[doc = "GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
pub mod tbpmr;
#[doc = "TAR (r) register accessor: GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\]
contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TAV register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
#[doc(alias = "TAR")]
pub type Tar = crate::Reg<tar::TarSpec>;
#[doc = "GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\]
contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TAV register."]
pub mod tar;
#[doc = "TBR (r) register accessor: GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\]
contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TBV register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbr`]
module"]
#[doc(alias = "TBR")]
pub type Tbr = crate::Reg<tbr::TbrSpec>;
#[doc = "GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\]
contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TBV register."]
pub mod tbr;
#[doc = "TAV (rw) register accessor: GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tav::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tav::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tav`]
module"]
#[doc(alias = "TAV")]
pub type Tav = crate::Reg<tav::TavSpec>;
#[doc = "GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0."]
pub mod tav;
#[doc = "TBV (rw) register accessor: GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbv`]
module"]
#[doc(alias = "TBV")]
pub type Tbv = crate::Reg<tbv::TbvSpec>;
#[doc = "GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0."]
pub mod tbv;
#[doc = "TAPS (r) register accessor: GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`taps::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taps`]
module"]
#[doc(alias = "TAPS")]
pub type Taps = crate::Reg<taps::TapsSpec>;
#[doc = "GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
pub mod taps;
#[doc = "TBPS (r) register accessor: GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbps::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbps`]
module"]
#[doc(alias = "TBPS")]
pub type Tbps = crate::Reg<tbps::TbpsSpec>;
#[doc = "GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
pub mod tbps;
#[doc = "TAPV (r) register accessor: GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`tapv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tapv`]
module"]
#[doc(alias = "TAPV")]
pub type Tapv = crate::Reg<tapv::TapvSpec>;
#[doc = "GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
pub mod tapv;
#[doc = "TBPV (r) register accessor: GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbpv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbpv`]
module"]
#[doc(alias = "TBPV")]
pub type Tbpv = crate::Reg<tbpv::TbpvSpec>;
#[doc = "GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
pub mod tbpv;
#[doc = "PP (r) register accessor: GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module.\n\nYou can [`read`](crate::Reg::read) this register and get [`pp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
#[doc(alias = "PP")]
pub type Pp = crate::Reg<pp::PpSpec>;
#[doc = "GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module."]
pub mod pp;
