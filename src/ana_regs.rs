#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Analog control register"]
    pub ivctrl: IVCTRL,
}
#[doc = "IVCTRL (rw) register accessor: an alias for `Reg<IVCTRL_SPEC>`"]
pub type IVCTRL = crate::Reg<ivctrl::IVCTRL_SPEC>;
#[doc = "Analog control register"]
pub mod ivctrl;
