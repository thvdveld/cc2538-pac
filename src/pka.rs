#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aptr: Aptr,
    bptr: Bptr,
    cptr: Cptr,
    dptr: Dptr,
    alength: Alength,
    blength: Blength,
    shift: Shift,
    function: Function,
    compare: Compare,
    msw: Msw,
    divmsw: Divmsw,
    _reserved11: [u8; 0x9c],
    seq_ctrl: SeqCtrl,
    _reserved12: [u8; 0x28],
    options: Options,
    sw_rev: SwRev,
    revision: Revision,
}
impl RegisterBlock {
    #[doc = "0x00 - PKA vector A address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    #[inline(always)]
    pub const fn aptr(&self) -> &Aptr {
        &self.aptr
    }
    #[doc = "0x04 - PKA vector B address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    #[inline(always)]
    pub const fn bptr(&self) -> &Bptr {
        &self.bptr
    }
    #[doc = "0x08 - PKA vector C address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    #[inline(always)]
    pub const fn cptr(&self) -> &Cptr {
        &self.cptr
    }
    #[doc = "0x0c - PKA vector D address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    #[inline(always)]
    pub const fn dptr(&self) -> &Dptr {
        &self.dptr
    }
    #[doc = "0x10 - PKA vector A length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    #[inline(always)]
    pub const fn alength(&self) -> &Alength {
        &self.alength
    }
    #[doc = "0x14 - PKA vector B length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    #[inline(always)]
    pub const fn blength(&self) -> &Blength {
        &self.blength
    }
    #[doc = "0x18 - PKA bit shift value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code."]
    #[inline(always)]
    pub const fn shift(&self) -> &Shift {
        &self.shift
    }
    #[doc = "0x1c - PKA function This register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. During the execution of sequencer-controlled complex operations, this register is modified; the run and stall result bits are set to zero at the conclusion, but other bits are undefined. Attention: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations."]
    #[inline(always)]
    pub const fn function(&self) -> &Function {
        &self.function
    }
    #[doc = "0x20 - PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the run bit in the PKA_FUNCTION register is reset at the end of that operation. Status after a complex sequencer operation is unknown"]
    #[inline(always)]
    pub const fn compare(&self) -> &Compare {
        &self.compare
    }
    #[doc = "0x24 - PKA most-significant-word of result vector This register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated when the run bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, the PKA_MSW register will provide information for the x-coordinate of the result point only."]
    #[inline(always)]
    pub const fn msw(&self) -> &Msw {
        &self.msw
    }
    #[doc = "0x28 - PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when the RUN bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; the PKA_DIVMSW register provides information for the x-coordinate of the result point only."]
    #[inline(always)]
    pub const fn divmsw(&self) -> &Divmsw {
        &self.divmsw
    }
    #[doc = "0xc8 - PKA sequencer control and status register The sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESTE bit depends upon the option chosen for sequencer program storage."]
    #[inline(always)]
    pub const fn seq_ctrl(&self) -> &SeqCtrl {
        &self.seq_ctrl
    }
    #[doc = "0xf4 - PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module. Note: (32 x (1st LNME nr. of PEs + 1st LNME FIFO RAM depth - 10)) equals the maximum modulus vector length (in bits) that can be handled by the modular exponentiation and ECC operations executed on a PKA engine that includes an LNME."]
    #[inline(always)]
    pub const fn options(&self) -> &Options {
        &self.options
    }
    #[doc = "0xf8 - PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. The PKA_SW_REV register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet."]
    #[inline(always)]
    pub const fn sw_rev(&self) -> &SwRev {
        &self.sw_rev
    }
    #[doc = "0xfc - PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
}
#[doc = "APTR (rw) register accessor: PKA vector A address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`aptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aptr`]
module"]
#[doc(alias = "APTR")]
pub type Aptr = crate::Reg<aptr::AptrSpec>;
#[doc = "PKA vector A address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod aptr;
#[doc = "BPTR (rw) register accessor: PKA vector B address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`bptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bptr`]
module"]
#[doc(alias = "BPTR")]
pub type Bptr = crate::Reg<bptr::BptrSpec>;
#[doc = "PKA vector B address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod bptr;
#[doc = "CPTR (rw) register accessor: PKA vector C address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`cptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cptr`]
module"]
#[doc(alias = "CPTR")]
pub type Cptr = crate::Reg<cptr::CptrSpec>;
#[doc = "PKA vector C address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod cptr;
#[doc = "DPTR (rw) register accessor: PKA vector D address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`dptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dptr`]
module"]
#[doc(alias = "DPTR")]
pub type Dptr = crate::Reg<dptr::DptrSpec>;
#[doc = "PKA vector D address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod dptr;
#[doc = "ALENGTH (rw) register accessor: PKA vector A length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`alength::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alength::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alength`]
module"]
#[doc(alias = "ALENGTH")]
pub type Alength = crate::Reg<alength::AlengthSpec>;
#[doc = "PKA vector A length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod alength;
#[doc = "BLENGTH (rw) register accessor: PKA vector B length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`blength::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blength::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blength`]
module"]
#[doc(alias = "BLENGTH")]
pub type Blength = crate::Reg<blength::BlengthSpec>;
#[doc = "PKA vector B length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod blength;
#[doc = "SHIFT (rw) register accessor: PKA bit shift value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code.\n\nYou can [`read`](crate::Reg::read) this register and get [`shift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shift`]
module"]
#[doc(alias = "SHIFT")]
pub type Shift = crate::Reg<shift::ShiftSpec>;
#[doc = "PKA bit shift value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code."]
pub mod shift;
#[doc = "FUNCTION (rw) register accessor: PKA function This register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. During the execution of sequencer-controlled complex operations, this register is modified; the run and stall result bits are set to zero at the conclusion, but other bits are undefined. Attention: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations.\n\nYou can [`read`](crate::Reg::read) this register and get [`function::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`function::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function`]
module"]
#[doc(alias = "FUNCTION")]
pub type Function = crate::Reg<function::FunctionSpec>;
#[doc = "PKA function This register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. During the execution of sequencer-controlled complex operations, this register is modified; the run and stall result bits are set to zero at the conclusion, but other bits are undefined. Attention: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations."]
pub mod function;
#[doc = "COMPARE (r) register accessor: PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the run bit in the PKA_FUNCTION register is reset at the end of that operation. Status after a complex sequencer operation is unknown\n\nYou can [`read`](crate::Reg::read) this register and get [`compare::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compare`]
module"]
#[doc(alias = "COMPARE")]
pub type Compare = crate::Reg<compare::CompareSpec>;
#[doc = "PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the run bit in the PKA_FUNCTION register is reset at the end of that operation. Status after a complex sequencer operation is unknown"]
pub mod compare;
#[doc = "MSW (r) register accessor: PKA most-significant-word of result vector This register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated when the run bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, the PKA_MSW register will provide information for the x-coordinate of the result point only.\n\nYou can [`read`](crate::Reg::read) this register and get [`msw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msw`]
module"]
#[doc(alias = "MSW")]
pub type Msw = crate::Reg<msw::MswSpec>;
#[doc = "PKA most-significant-word of result vector This register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated when the run bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, the PKA_MSW register will provide information for the x-coordinate of the result point only."]
pub mod msw;
#[doc = "DIVMSW (r) register accessor: PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when the RUN bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; the PKA_DIVMSW register provides information for the x-coordinate of the result point only.\n\nYou can [`read`](crate::Reg::read) this register and get [`divmsw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divmsw`]
module"]
#[doc(alias = "DIVMSW")]
pub type Divmsw = crate::Reg<divmsw::DivmswSpec>;
#[doc = "PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when the RUN bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; the PKA_DIVMSW register provides information for the x-coordinate of the result point only."]
pub mod divmsw;
#[doc = "SEQ_CTRL (rw) register accessor: PKA sequencer control and status register The sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESTE bit depends upon the option chosen for sequencer program storage.\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_ctrl`]
module"]
#[doc(alias = "SEQ_CTRL")]
pub type SeqCtrl = crate::Reg<seq_ctrl::SeqCtrlSpec>;
#[doc = "PKA sequencer control and status register The sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESTE bit depends upon the option chosen for sequencer program storage."]
pub mod seq_ctrl;
#[doc = "OPTIONS (r) register accessor: PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module. Note: (32 x (1st LNME nr. of PEs + 1st LNME FIFO RAM depth - 10)) equals the maximum modulus vector length (in bits) that can be handled by the modular exponentiation and ECC operations executed on a PKA engine that includes an LNME.\n\nYou can [`read`](crate::Reg::read) this register and get [`options::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@options`]
module"]
#[doc(alias = "OPTIONS")]
pub type Options = crate::Reg<options::OptionsSpec>;
#[doc = "PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module. Note: (32 x (1st LNME nr. of PEs + 1st LNME FIFO RAM depth - 10)) equals the maximum modulus vector length (in bits) that can be handled by the modular exponentiation and ECC operations executed on a PKA engine that includes an LNME."]
pub mod options;
#[doc = "SW_REV (r) register accessor: PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. The PKA_SW_REV register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet.\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_rev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_rev`]
module"]
#[doc(alias = "SW_REV")]
pub type SwRev = crate::Reg<sw_rev::SwRevSpec>;
#[doc = "PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. The PKA_SW_REV register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet."]
pub mod sw_rev;
#[doc = "REVISION (r) register accessor: PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`]
module"]
#[doc(alias = "REVISION")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
pub mod revision;
