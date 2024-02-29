#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    io: Io,
    _reserved1: [u8; 0x10],
    obssel0: Obssel0,
    obssel1: Obssel1,
    obssel2: Obssel2,
    obssel3: Obssel3,
    obssel4: Obssel4,
    obssel5: Obssel5,
    obssel6: Obssel6,
    obssel7: Obssel7,
    tr0: Tr0,
    _reserved10: [u8; 0x18],
    usbctrl: Usbctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Output strength control"]
    #[inline(always)]
    pub const fn io(&self) -> &Io {
        &self.io
    }
    #[doc = "0x14 - Select output signal on observation output 0"]
    #[inline(always)]
    pub const fn obssel0(&self) -> &Obssel0 {
        &self.obssel0
    }
    #[doc = "0x18 - Select output signal on observation output 1"]
    #[inline(always)]
    pub const fn obssel1(&self) -> &Obssel1 {
        &self.obssel1
    }
    #[doc = "0x1c - Select output signal on observation output 2"]
    #[inline(always)]
    pub const fn obssel2(&self) -> &Obssel2 {
        &self.obssel2
    }
    #[doc = "0x20 - Select output signal on observation output 3"]
    #[inline(always)]
    pub const fn obssel3(&self) -> &Obssel3 {
        &self.obssel3
    }
    #[doc = "0x24 - Select output signal on observation output 4"]
    #[inline(always)]
    pub const fn obssel4(&self) -> &Obssel4 {
        &self.obssel4
    }
    #[doc = "0x28 - Select output signal on observation output 5"]
    #[inline(always)]
    pub const fn obssel5(&self) -> &Obssel5 {
        &self.obssel5
    }
    #[doc = "0x2c - Select output signal on observation output 6"]
    #[inline(always)]
    pub const fn obssel6(&self) -> &Obssel6 {
        &self.obssel6
    }
    #[doc = "0x30 - Select output signal on observation output 7"]
    #[inline(always)]
    pub const fn obssel7(&self) -> &Obssel7 {
        &self.obssel7
    }
    #[doc = "0x34 - Test register 0"]
    #[inline(always)]
    pub const fn tr0(&self) -> &Tr0 {
        &self.tr0
    }
    #[doc = "0x50 - USB PHY stand-by control"]
    #[inline(always)]
    pub const fn usbctrl(&self) -> &Usbctrl {
        &self.usbctrl
    }
}
#[doc = "IO (rw) register accessor: Output strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io`]
module"]
#[doc(alias = "IO")]
pub type Io = crate::Reg<io::IoSpec>;
#[doc = "Output strength control"]
pub mod io;
#[doc = "OBSSEL0 (rw) register accessor: Select output signal on observation output 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel0`]
module"]
#[doc(alias = "OBSSEL0")]
pub type Obssel0 = crate::Reg<obssel0::Obssel0Spec>;
#[doc = "Select output signal on observation output 0"]
pub mod obssel0;
#[doc = "OBSSEL1 (rw) register accessor: Select output signal on observation output 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel1`]
module"]
#[doc(alias = "OBSSEL1")]
pub type Obssel1 = crate::Reg<obssel1::Obssel1Spec>;
#[doc = "Select output signal on observation output 1"]
pub mod obssel1;
#[doc = "OBSSEL2 (rw) register accessor: Select output signal on observation output 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel2`]
module"]
#[doc(alias = "OBSSEL2")]
pub type Obssel2 = crate::Reg<obssel2::Obssel2Spec>;
#[doc = "Select output signal on observation output 2"]
pub mod obssel2;
#[doc = "OBSSEL3 (rw) register accessor: Select output signal on observation output 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel3`]
module"]
#[doc(alias = "OBSSEL3")]
pub type Obssel3 = crate::Reg<obssel3::Obssel3Spec>;
#[doc = "Select output signal on observation output 3"]
pub mod obssel3;
#[doc = "OBSSEL4 (rw) register accessor: Select output signal on observation output 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel4`]
module"]
#[doc(alias = "OBSSEL4")]
pub type Obssel4 = crate::Reg<obssel4::Obssel4Spec>;
#[doc = "Select output signal on observation output 4"]
pub mod obssel4;
#[doc = "OBSSEL5 (rw) register accessor: Select output signal on observation output 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel5`]
module"]
#[doc(alias = "OBSSEL5")]
pub type Obssel5 = crate::Reg<obssel5::Obssel5Spec>;
#[doc = "Select output signal on observation output 5"]
pub mod obssel5;
#[doc = "OBSSEL6 (rw) register accessor: Select output signal on observation output 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel6`]
module"]
#[doc(alias = "OBSSEL6")]
pub type Obssel6 = crate::Reg<obssel6::Obssel6Spec>;
#[doc = "Select output signal on observation output 6"]
pub mod obssel6;
#[doc = "OBSSEL7 (rw) register accessor: Select output signal on observation output 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obssel7`]
module"]
#[doc(alias = "OBSSEL7")]
pub type Obssel7 = crate::Reg<obssel7::Obssel7Spec>;
#[doc = "Select output signal on observation output 7"]
pub mod obssel7;
#[doc = "TR0 (rw) register accessor: Test register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr0`]
module"]
#[doc(alias = "TR0")]
pub type Tr0 = crate::Reg<tr0::Tr0Spec>;
#[doc = "Test register 0"]
pub mod tr0;
#[doc = "USBCTRL (rw) register accessor: USB PHY stand-by control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbctrl`]
module"]
#[doc(alias = "USBCTRL")]
pub type Usbctrl = crate::Reg<usbctrl::UsbctrlSpec>;
#[doc = "USB PHY stand-by control"]
pub mod usbctrl;
