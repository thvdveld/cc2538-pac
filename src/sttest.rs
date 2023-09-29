#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTIMER Internal loopback"]
    pub lpbkgpt: LPBKGPT,
    #[doc = "0x04 - UART internal loopback"]
    pub lpbkuart: LPBKUART,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - I2C internal loopback"]
    pub lpbki2c: LPBKI2C,
    _reserved3: [u8; 0x34],
    #[doc = "0x44 - Peripheral test mode enable 1"]
    pub ptme1: PTME1,
    #[doc = "0x48 - Peripheral test mode enable 2"]
    pub ptme2: PTME2,
    _reserved5: [u8; 0x74],
    #[doc = "0xc0 - GPTIMER override values"]
    pub gpt: GPT,
}
#[doc = "LPBKGPT (rw) register accessor: GPTIMER Internal loopback\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbkgpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbkgpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpbkgpt`]
module"]
pub type LPBKGPT = crate::Reg<lpbkgpt::LPBKGPT_SPEC>;
#[doc = "GPTIMER Internal loopback"]
pub mod lpbkgpt;
#[doc = "LPBKUART (rw) register accessor: UART internal loopback\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbkuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbkuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpbkuart`]
module"]
pub type LPBKUART = crate::Reg<lpbkuart::LPBKUART_SPEC>;
#[doc = "UART internal loopback"]
pub mod lpbkuart;
#[doc = "LPBKI2C (rw) register accessor: I2C internal loopback\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbki2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbki2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpbki2c`]
module"]
pub type LPBKI2C = crate::Reg<lpbki2c::LPBKI2C_SPEC>;
#[doc = "I2C internal loopback"]
pub mod lpbki2c;
#[doc = "PTME1 (rw) register accessor: Peripheral test mode enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptme1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptme1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptme1`]
module"]
pub type PTME1 = crate::Reg<ptme1::PTME1_SPEC>;
#[doc = "Peripheral test mode enable 1"]
pub mod ptme1;
#[doc = "PTME2 (rw) register accessor: Peripheral test mode enable 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptme2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptme2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptme2`]
module"]
pub type PTME2 = crate::Reg<ptme2::PTME2_SPEC>;
#[doc = "Peripheral test mode enable 2"]
pub mod ptme2;
#[doc = "GPT (rw) register accessor: GPTIMER override values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpt`]
module"]
pub type GPT = crate::Reg<gpt::GPT_SPEC>;
#[doc = "GPTIMER override values"]
pub mod gpt;
