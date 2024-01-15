#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    io: IO,
    _reserved1: [u8; 0x10],
    obssel0: OBSSEL0,
    obssel1: OBSSEL1,
    obssel2: OBSSEL2,
    obssel3: OBSSEL3,
    obssel4: OBSSEL4,
    obssel5: OBSSEL5,
    obssel6: OBSSEL6,
    obssel7: OBSSEL7,
    tr0: TR0,
    _reserved10: [u8; 0x18],
    usbctrl: USBCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Output strength control"]
    #[inline(always)]
    pub const fn io(&self) -> &IO {
        &self.io
    }
    #[doc = "0x14 - Select output signal on observation output 0"]
    #[inline(always)]
    pub const fn obssel0(&self) -> &OBSSEL0 {
        &self.obssel0
    }
    #[doc = "0x18 - Select output signal on observation output 1"]
    #[inline(always)]
    pub const fn obssel1(&self) -> &OBSSEL1 {
        &self.obssel1
    }
    #[doc = "0x1c - Select output signal on observation output 2"]
    #[inline(always)]
    pub const fn obssel2(&self) -> &OBSSEL2 {
        &self.obssel2
    }
    #[doc = "0x20 - Select output signal on observation output 3"]
    #[inline(always)]
    pub const fn obssel3(&self) -> &OBSSEL3 {
        &self.obssel3
    }
    #[doc = "0x24 - Select output signal on observation output 4"]
    #[inline(always)]
    pub const fn obssel4(&self) -> &OBSSEL4 {
        &self.obssel4
    }
    #[doc = "0x28 - Select output signal on observation output 5"]
    #[inline(always)]
    pub const fn obssel5(&self) -> &OBSSEL5 {
        &self.obssel5
    }
    #[doc = "0x2c - Select output signal on observation output 6"]
    #[inline(always)]
    pub const fn obssel6(&self) -> &OBSSEL6 {
        &self.obssel6
    }
    #[doc = "0x30 - Select output signal on observation output 7"]
    #[inline(always)]
    pub const fn obssel7(&self) -> &OBSSEL7 {
        &self.obssel7
    }
    #[doc = "0x34 - Test register 0"]
    #[inline(always)]
    pub const fn tr0(&self) -> &TR0 {
        &self.tr0
    }
    #[doc = "0x50 - USB PHY stand-by control"]
    #[inline(always)]
    pub const fn usbctrl(&self) -> &USBCTRL {
        &self.usbctrl
    }
}
#[doc = "IO (rw) register accessor: Output strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io`]
module"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "Output strength control"]
pub mod io;
#[doc = "OBSSEL0 (rw) register accessor: Select output signal on observation output 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel0`]
module"]
pub type OBSSEL0 = crate::Reg<obssel0::OBSSEL0_SPEC>;
#[doc = "Select output signal on observation output 0"]
pub mod obssel0;
#[doc = "OBSSEL1 (rw) register accessor: Select output signal on observation output 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel1`]
module"]
pub type OBSSEL1 = crate::Reg<obssel1::OBSSEL1_SPEC>;
#[doc = "Select output signal on observation output 1"]
pub mod obssel1;
#[doc = "OBSSEL2 (rw) register accessor: Select output signal on observation output 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel2`]
module"]
pub type OBSSEL2 = crate::Reg<obssel2::OBSSEL2_SPEC>;
#[doc = "Select output signal on observation output 2"]
pub mod obssel2;
#[doc = "OBSSEL3 (rw) register accessor: Select output signal on observation output 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel3`]
module"]
pub type OBSSEL3 = crate::Reg<obssel3::OBSSEL3_SPEC>;
#[doc = "Select output signal on observation output 3"]
pub mod obssel3;
#[doc = "OBSSEL4 (rw) register accessor: Select output signal on observation output 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel4`]
module"]
pub type OBSSEL4 = crate::Reg<obssel4::OBSSEL4_SPEC>;
#[doc = "Select output signal on observation output 4"]
pub mod obssel4;
#[doc = "OBSSEL5 (rw) register accessor: Select output signal on observation output 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel5`]
module"]
pub type OBSSEL5 = crate::Reg<obssel5::OBSSEL5_SPEC>;
#[doc = "Select output signal on observation output 5"]
pub mod obssel5;
#[doc = "OBSSEL6 (rw) register accessor: Select output signal on observation output 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel6`]
module"]
pub type OBSSEL6 = crate::Reg<obssel6::OBSSEL6_SPEC>;
#[doc = "Select output signal on observation output 6"]
pub mod obssel6;
#[doc = "OBSSEL7 (rw) register accessor: Select output signal on observation output 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel7`]
module"]
pub type OBSSEL7 = crate::Reg<obssel7::OBSSEL7_SPEC>;
#[doc = "Select output signal on observation output 7"]
pub mod obssel7;
#[doc = "TR0 (rw) register accessor: Test register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr0`]
module"]
pub type TR0 = crate::Reg<tr0::TR0_SPEC>;
#[doc = "Test register 0"]
pub mod tr0;
#[doc = "USBCTRL (rw) register accessor: USB PHY stand-by control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbctrl`]
module"]
pub type USBCTRL = crate::Reg<usbctrl::USBCTRL_SPEC>;
#[doc = "USB PHY stand-by control"]
pub mod usbctrl;
