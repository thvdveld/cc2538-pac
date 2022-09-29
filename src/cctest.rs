#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output strength control"]
    pub io: IO,
    _reserved1: [u8; 0x10],
    #[doc = "0x14 - Select output signal on observation output 0"]
    pub obssel0: OBSSEL0,
    #[doc = "0x18 - Select output signal on observation output 1"]
    pub obssel1: OBSSEL1,
    #[doc = "0x1c - Select output signal on observation output 2"]
    pub obssel2: OBSSEL2,
    #[doc = "0x20 - Select output signal on observation output 3"]
    pub obssel3: OBSSEL3,
    #[doc = "0x24 - Select output signal on observation output 4"]
    pub obssel4: OBSSEL4,
    #[doc = "0x28 - Select output signal on observation output 5"]
    pub obssel5: OBSSEL5,
    #[doc = "0x2c - Select output signal on observation output 6"]
    pub obssel6: OBSSEL6,
    #[doc = "0x30 - Select output signal on observation output 7"]
    pub obssel7: OBSSEL7,
    #[doc = "0x34 - Test register 0"]
    pub tr0: TR0,
    _reserved10: [u8; 0x18],
    #[doc = "0x50 - USB PHY stand-by control"]
    pub usbctrl: USBCTRL,
}
#[doc = "IO (rw) register accessor: an alias for `Reg<IO_SPEC>`"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "Output strength control"]
pub mod io;
#[doc = "OBSSEL0 (rw) register accessor: an alias for `Reg<OBSSEL0_SPEC>`"]
pub type OBSSEL0 = crate::Reg<obssel0::OBSSEL0_SPEC>;
#[doc = "Select output signal on observation output 0"]
pub mod obssel0;
#[doc = "OBSSEL1 (rw) register accessor: an alias for `Reg<OBSSEL1_SPEC>`"]
pub type OBSSEL1 = crate::Reg<obssel1::OBSSEL1_SPEC>;
#[doc = "Select output signal on observation output 1"]
pub mod obssel1;
#[doc = "OBSSEL2 (rw) register accessor: an alias for `Reg<OBSSEL2_SPEC>`"]
pub type OBSSEL2 = crate::Reg<obssel2::OBSSEL2_SPEC>;
#[doc = "Select output signal on observation output 2"]
pub mod obssel2;
#[doc = "OBSSEL3 (rw) register accessor: an alias for `Reg<OBSSEL3_SPEC>`"]
pub type OBSSEL3 = crate::Reg<obssel3::OBSSEL3_SPEC>;
#[doc = "Select output signal on observation output 3"]
pub mod obssel3;
#[doc = "OBSSEL4 (rw) register accessor: an alias for `Reg<OBSSEL4_SPEC>`"]
pub type OBSSEL4 = crate::Reg<obssel4::OBSSEL4_SPEC>;
#[doc = "Select output signal on observation output 4"]
pub mod obssel4;
#[doc = "OBSSEL5 (rw) register accessor: an alias for `Reg<OBSSEL5_SPEC>`"]
pub type OBSSEL5 = crate::Reg<obssel5::OBSSEL5_SPEC>;
#[doc = "Select output signal on observation output 5"]
pub mod obssel5;
#[doc = "OBSSEL6 (rw) register accessor: an alias for `Reg<OBSSEL6_SPEC>`"]
pub type OBSSEL6 = crate::Reg<obssel6::OBSSEL6_SPEC>;
#[doc = "Select output signal on observation output 6"]
pub mod obssel6;
#[doc = "OBSSEL7 (rw) register accessor: an alias for `Reg<OBSSEL7_SPEC>`"]
pub type OBSSEL7 = crate::Reg<obssel7::OBSSEL7_SPEC>;
#[doc = "Select output signal on observation output 7"]
pub mod obssel7;
#[doc = "TR0 (rw) register accessor: an alias for `Reg<TR0_SPEC>`"]
pub type TR0 = crate::Reg<tr0::TR0_SPEC>;
#[doc = "Test register 0"]
pub mod tr0;
#[doc = "USBCTRL (rw) register accessor: an alias for `Reg<USBCTRL_SPEC>`"]
pub type USBCTRL = crate::Reg<usbctrl::USBCTRL_SPEC>;
#[doc = "USB PHY stand-by control"]
pub mod usbctrl;
