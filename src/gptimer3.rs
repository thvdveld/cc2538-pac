#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers)."]
    pub cfg: CFG,
    #[doc = "0x04 - GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored."]
    pub tamr: TAMR,
    #[doc = "0x08 - GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B."]
    pub tbmr: TBMR,
    #[doc = "0x0c - GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall."]
    pub ctl: CTL,
    #[doc = "0x10 - GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers."]
    pub sync: SYNC,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it."]
    pub imr: IMR,
    #[doc = "0x1c - GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR."]
    pub ris: RIS,
    #[doc = "0x20 - GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR."]
    pub mis: MIS,
    #[doc = "0x24 - GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers."]
    pub icr: ICR,
    #[doc = "0x28 - GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR."]
    pub tailr: TAILR,
    #[doc = "0x2c - GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the load value. Bits \\[31:16\\]
are reserved in both cases."]
    pub tbilr: TBILR,
    #[doc = "0x30 - GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR."]
    pub tamatchr: TAMATCHR,
    #[doc = "0x34 - PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the match value. Bits \\[31:16\\]
are reserved in both cases."]
    pub tbmatchr: TBMATCHR,
    #[doc = "0x38 - GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
    pub tapr: TAPR,
    #[doc = "0x3c - GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
    pub tbpr: TBPR,
    #[doc = "0x40 - GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
    pub tapmr: TAPMR,
    #[doc = "0x44 - GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
    pub tbpmr: TBPMR,
    #[doc = "0x48 - GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\]
contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TAV register."]
    pub tar: TAR,
    #[doc = "0x4c - GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\]
contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TBV register."]
    pub tbr: TBR,
    #[doc = "0x50 - GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0."]
    pub tav: TAV,
    #[doc = "0x54 - GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0."]
    pub tbv: TBV,
    _reserved21: [u8; 0x04],
    #[doc = "0x5c - GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
    pub taps: TAPS,
    #[doc = "0x60 - GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
    pub tbps: TBPS,
    #[doc = "0x64 - GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
    pub tapv: TAPV,
    #[doc = "0x68 - GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
    pub tbpv: TBPV,
    _reserved25: [u8; 0x0f54],
    #[doc = "0xfc0 - GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module."]
    pub pp: PP,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers)."]
pub mod cfg;
#[doc = "TAMR (rw) register accessor: an alias for `Reg<TAMR_SPEC>`"]
pub type TAMR = crate::Reg<tamr::TAMR_SPEC>;
#[doc = "GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored."]
pub mod tamr;
#[doc = "TBMR (rw) register accessor: an alias for `Reg<TBMR_SPEC>`"]
pub type TBMR = crate::Reg<tbmr::TBMR_SPEC>;
#[doc = "GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B."]
pub mod tbmr;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall."]
pub mod ctl;
#[doc = "SYNC (rw) register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers."]
pub mod sync;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it."]
pub mod imr;
#[doc = "RIS (r) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR."]
pub mod ris;
#[doc = "MIS (r) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR."]
pub mod mis;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers."]
pub mod icr;
#[doc = "TAILR (rw) register accessor: an alias for `Reg<TAILR_SPEC>`"]
pub type TAILR = crate::Reg<tailr::TAILR_SPEC>;
#[doc = "GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR."]
pub mod tailr;
#[doc = "TBILR (rw) register accessor: an alias for `Reg<TBILR_SPEC>`"]
pub type TBILR = crate::Reg<tbilr::TBILR_SPEC>;
#[doc = "GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the load value. Bits \\[31:16\\]
are reserved in both cases."]
pub mod tbilr;
#[doc = "TAMATCHR (rw) register accessor: an alias for `Reg<TAMATCHR_SPEC>`"]
pub type TAMATCHR = crate::Reg<tamatchr::TAMATCHR_SPEC>;
#[doc = "GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR."]
pub mod tamatchr;
#[doc = "TBMATCHR (rw) register accessor: an alias for `Reg<TBMATCHR_SPEC>`"]
pub type TBMATCHR = crate::Reg<tbmatchr::TBMATCHR_SPEC>;
#[doc = "PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the match value. Bits \\[31:16\\]
are reserved in both cases."]
pub mod tbmatchr;
#[doc = "TAPR (rw) register accessor: an alias for `Reg<TAPR_SPEC>`"]
pub type TAPR = crate::Reg<tapr::TAPR_SPEC>;
#[doc = "GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
pub mod tapr;
#[doc = "TBPR (rw) register accessor: an alias for `Reg<TBPR_SPEC>`"]
pub type TBPR = crate::Reg<tbpr::TBPR_SPEC>;
#[doc = "GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
pub mod tbpr;
#[doc = "TAPMR (rw) register accessor: an alias for `Reg<TAPMR_SPEC>`"]
pub type TAPMR = crate::Reg<tapmr::TAPMR_SPEC>;
#[doc = "GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
pub mod tapmr;
#[doc = "TBPMR (rw) register accessor: an alias for `Reg<TBPMR_SPEC>`"]
pub type TBPMR = crate::Reg<tbpmr::TBPMR_SPEC>;
#[doc = "GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
pub mod tbpmr;
#[doc = "TAR (r) register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\]
contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TAV register."]
pub mod tar;
#[doc = "TBR (r) register accessor: an alias for `Reg<TBR_SPEC>`"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\]
contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TBV register."]
pub mod tbr;
#[doc = "TAV (rw) register accessor: an alias for `Reg<TAV_SPEC>`"]
pub type TAV = crate::Reg<tav::TAV_SPEC>;
#[doc = "GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0."]
pub mod tav;
#[doc = "TBV (rw) register accessor: an alias for `Reg<TBV_SPEC>`"]
pub type TBV = crate::Reg<tbv::TBV_SPEC>;
#[doc = "GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0."]
pub mod tbv;
#[doc = "TAPS (r) register accessor: an alias for `Reg<TAPS_SPEC>`"]
pub type TAPS = crate::Reg<taps::TAPS_SPEC>;
#[doc = "GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
pub mod taps;
#[doc = "TBPS (r) register accessor: an alias for `Reg<TBPS_SPEC>`"]
pub type TBPS = crate::Reg<tbps::TBPS_SPEC>;
#[doc = "GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
pub mod tbps;
#[doc = "TAPV (r) register accessor: an alias for `Reg<TAPV_SPEC>`"]
pub type TAPV = crate::Reg<tapv::TAPV_SPEC>;
#[doc = "GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
pub mod tapv;
#[doc = "TBPV (r) register accessor: an alias for `Reg<TBPV_SPEC>`"]
pub type TBPV = crate::Reg<tbpv::TBPV_SPEC>;
#[doc = "GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
pub mod tbpv;
#[doc = "PP (r) register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module."]
pub mod pp;
