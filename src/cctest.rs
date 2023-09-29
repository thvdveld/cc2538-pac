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
#[doc = "IO (rw) register accessor: Output strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`io`]
module"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "Output strength control"]
pub mod io;
#[doc = "OBSSEL0 (rw) register accessor: Select output signal on observation output 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obssel0`]
module"]
pub type OBSSEL0 = crate::Reg<obssel0::OBSSEL0_SPEC>;
#[doc = "Select output signal on observation output 0"]
pub mod obssel0;
#[doc = "OBSSEL1 (rw) register accessor: Select output signal on observation output 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obssel1`]
module"]
pub type OBSSEL1 = crate::Reg<obssel1::OBSSEL1_SPEC>;
#[doc = "Select output signal on observation output 1"]
pub mod obssel1;
#[doc = "OBSSEL2 (rw) register accessor: Select output signal on observation output 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obssel2`]
module"]
pub type OBSSEL2 = crate::Reg<obssel2::OBSSEL2_SPEC>;
#[doc = "Select output signal on observation output 2"]
pub mod obssel2;
#[doc = "OBSSEL3 (rw) register accessor: Select output signal on observation output 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obssel3`]
module"]
pub type OBSSEL3 = crate::Reg<obssel3::OBSSEL3_SPEC>;
#[doc = "Select output signal on observation output 3"]
pub mod obssel3;
#[doc = "OBSSEL4 (rw) register accessor: Select output signal on observation output 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obssel4`]
module"]
pub type OBSSEL4 = crate::Reg<obssel4::OBSSEL4_SPEC>;
#[doc = "Select output signal on observation output 4"]
pub mod obssel4;
#[doc = "OBSSEL5 (rw) register accessor: Select output signal on observation output 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obssel5`]
module"]
pub type OBSSEL5 = crate::Reg<obssel5::OBSSEL5_SPEC>;
#[doc = "Select output signal on observation output 5"]
pub mod obssel5;
#[doc = "OBSSEL6 (rw) register accessor: Select output signal on observation output 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obssel6`]
module"]
pub type OBSSEL6 = crate::Reg<obssel6::OBSSEL6_SPEC>;
#[doc = "Select output signal on observation output 6"]
pub mod obssel6;
#[doc = "OBSSEL7 (rw) register accessor: Select output signal on observation output 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`obssel7`]
module"]
pub type OBSSEL7 = crate::Reg<obssel7::OBSSEL7_SPEC>;
#[doc = "Select output signal on observation output 7"]
pub mod obssel7;
#[doc = "TR0 (rw) register accessor: Test register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tr0`]
module"]
pub type TR0 = crate::Reg<tr0::TR0_SPEC>;
#[doc = "Test register 0"]
pub mod tr0;
#[doc = "USBCTRL (rw) register accessor: USB PHY stand-by control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usbctrl`]
module"]
pub type USBCTRL = crate::Reg<usbctrl::USBCTRL_SPEC>;
#[doc = "USB PHY stand-by control"]
pub mod usbctrl;
