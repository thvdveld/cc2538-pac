#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    lpbkgpt: LPBKGPT,
    lpbkuart: LPBKUART,
    _reserved2: [u8; 0x04],
    lpbki2c: LPBKI2C,
    _reserved3: [u8; 0x34],
    ptme1: PTME1,
    ptme2: PTME2,
    _reserved5: [u8; 0x74],
    gpt: GPT,
}
impl RegisterBlock {
    #[doc = "0x00 - GPTIMER Internal loopback"]
    #[inline(always)]
    pub const fn lpbkgpt(&self) -> &LPBKGPT {
        &self.lpbkgpt
    }
    #[doc = "0x04 - UART internal loopback"]
    #[inline(always)]
    pub const fn lpbkuart(&self) -> &LPBKUART {
        &self.lpbkuart
    }
    #[doc = "0x0c - I2C internal loopback"]
    #[inline(always)]
    pub const fn lpbki2c(&self) -> &LPBKI2C {
        &self.lpbki2c
    }
    #[doc = "0x44 - Peripheral test mode enable 1"]
    #[inline(always)]
    pub const fn ptme1(&self) -> &PTME1 {
        &self.ptme1
    }
    #[doc = "0x48 - Peripheral test mode enable 2"]
    #[inline(always)]
    pub const fn ptme2(&self) -> &PTME2 {
        &self.ptme2
    }
    #[doc = "0xc0 - GPTIMER override values"]
    #[inline(always)]
    pub const fn gpt(&self) -> &GPT {
        &self.gpt
    }
}
#[doc = "LPBKGPT (rw) register accessor: GPTIMER Internal loopback\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbkgpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbkgpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpbkgpt`]
module"]
pub type LPBKGPT = crate::Reg<lpbkgpt::LPBKGPT_SPEC>;
#[doc = "GPTIMER Internal loopback"]
pub mod lpbkgpt;
#[doc = "LPBKUART (rw) register accessor: UART internal loopback\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbkuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbkuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpbkuart`]
module"]
pub type LPBKUART = crate::Reg<lpbkuart::LPBKUART_SPEC>;
#[doc = "UART internal loopback"]
pub mod lpbkuart;
#[doc = "LPBKI2C (rw) register accessor: I2C internal loopback\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbki2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbki2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpbki2c`]
module"]
pub type LPBKI2C = crate::Reg<lpbki2c::LPBKI2C_SPEC>;
#[doc = "I2C internal loopback"]
pub mod lpbki2c;
#[doc = "PTME1 (rw) register accessor: Peripheral test mode enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptme1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptme1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptme1`]
module"]
pub type PTME1 = crate::Reg<ptme1::PTME1_SPEC>;
#[doc = "Peripheral test mode enable 1"]
pub mod ptme1;
#[doc = "PTME2 (rw) register accessor: Peripheral test mode enable 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptme2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptme2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptme2`]
module"]
pub type PTME2 = crate::Reg<ptme2::PTME2_SPEC>;
#[doc = "Peripheral test mode enable 2"]
pub mod ptme2;
#[doc = "GPT (rw) register accessor: GPTIMER override values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt`]
module"]
pub type GPT = crate::Reg<gpt::GPT_SPEC>;
#[doc = "GPTIMER override values"]
pub mod gpt;
