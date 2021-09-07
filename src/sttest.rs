#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTIMER Internal loopback"]
    pub lpbkgpt: crate::Reg<lpbkgpt::LPBKGPT_SPEC>,
    #[doc = "0x04 - UART internal loopback"]
    pub lpbkuart: crate::Reg<lpbkuart::LPBKUART_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - I2C internal loopback"]
    pub lpbki2c: crate::Reg<lpbki2c::LPBKI2C_SPEC>,
    _reserved3: [u8; 0x34],
    #[doc = "0x44 - Peripheral test mode enable 1"]
    pub ptme1: crate::Reg<ptme1::PTME1_SPEC>,
    #[doc = "0x48 - Peripheral test mode enable 2"]
    pub ptme2: crate::Reg<ptme2::PTME2_SPEC>,
    _reserved5: [u8; 0x74],
    #[doc = "0xc0 - GPTIMER override values"]
    pub gpt: crate::Reg<gpt::GPT_SPEC>,
}
#[doc = "LPBKGPT register accessor: an alias for `Reg<LPBKGPT_SPEC>`"]
pub type LPBKGPT = crate::Reg<lpbkgpt::LPBKGPT_SPEC>;
#[doc = "GPTIMER Internal loopback"]
pub mod lpbkgpt;
#[doc = "LPBKUART register accessor: an alias for `Reg<LPBKUART_SPEC>`"]
pub type LPBKUART = crate::Reg<lpbkuart::LPBKUART_SPEC>;
#[doc = "UART internal loopback"]
pub mod lpbkuart;
#[doc = "LPBKI2C register accessor: an alias for `Reg<LPBKI2C_SPEC>`"]
pub type LPBKI2C = crate::Reg<lpbki2c::LPBKI2C_SPEC>;
#[doc = "I2C internal loopback"]
pub mod lpbki2c;
#[doc = "PTME1 register accessor: an alias for `Reg<PTME1_SPEC>`"]
pub type PTME1 = crate::Reg<ptme1::PTME1_SPEC>;
#[doc = "Peripheral test mode enable 1"]
pub mod ptme1;
#[doc = "PTME2 register accessor: an alias for `Reg<PTME2_SPEC>`"]
pub type PTME2 = crate::Reg<ptme2::PTME2_SPEC>;
#[doc = "Peripheral test mode enable 2"]
pub mod ptme2;
#[doc = "GPT register accessor: an alias for `Reg<GPT_SPEC>`"]
pub type GPT = crate::Reg<gpt::GPT_SPEC>;
#[doc = "GPTIMER override values"]
pub mod gpt;
