#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    addr: Addr,
    pow: Pow,
    iif: Iif,
    _reserved3: [u8; 0x04],
    oif: Oif,
    _reserved4: [u8; 0x04],
    cif: Cif,
    iie: Iie,
    _reserved6: [u8; 0x04],
    oie: Oie,
    _reserved7: [u8; 0x04],
    cie: Cie,
    frml: Frml,
    frmh: Frmh,
    index: Index,
    ctrl: Ctrl,
    maxi: Maxi,
    cs0_csil: Cs0Csil,
    csih: Csih,
    maxo: Maxo,
    csol: Csol,
    csoh: Csoh,
    cnt0_cntl: Cnt0Cntl,
    cnth: Cnth,
    _reserved20: [u8; 0x20],
    f0: F0,
    _reserved21: [u8; 0x04],
    f1: F1,
    _reserved22: [u8; 0x04],
    f2: F2,
    _reserved23: [u8; 0x04],
    f3: F3,
    _reserved24: [u8; 0x04],
    f4: F4,
    _reserved25: [u8; 0x04],
    f5: F5,
}
impl RegisterBlock {
    #[doc = "0x00 - Function address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x04 - Power management and control register"]
    #[inline(always)]
    pub const fn pow(&self) -> &Pow {
        &self.pow
    }
    #[doc = "0x08 - Interrupt flags for endpoint 0 and IN endpoints 1-5"]
    #[inline(always)]
    pub const fn iif(&self) -> &Iif {
        &self.iif
    }
    #[doc = "0x10 - Interrupt flags for OUT endpoints 1-5"]
    #[inline(always)]
    pub const fn oif(&self) -> &Oif {
        &self.oif
    }
    #[doc = "0x18 - Common USB interrupt flags"]
    #[inline(always)]
    pub const fn cif(&self) -> &Cif {
        &self.cif
    }
    #[doc = "0x1c - Interrupt enable mask for IN endpoints 1-5 and endpoint 0"]
    #[inline(always)]
    pub const fn iie(&self) -> &Iie {
        &self.iie
    }
    #[doc = "0x24 - Interrupt enable mask for OUT endpoints 1-5"]
    #[inline(always)]
    pub const fn oie(&self) -> &Oie {
        &self.oie
    }
    #[doc = "0x2c - Common USB interrupt enable mask"]
    #[inline(always)]
    pub const fn cie(&self) -> &Cie {
        &self.cie
    }
    #[doc = "0x30 - Frame number (low byte)"]
    #[inline(always)]
    pub const fn frml(&self) -> &Frml {
        &self.frml
    }
    #[doc = "0x34 - Frame number (high byte)"]
    #[inline(always)]
    pub const fn frmh(&self) -> &Frmh {
        &self.frmh
    }
    #[doc = "0x38 - Index register for selecting the endpoint status and control registers"]
    #[inline(always)]
    pub const fn index(&self) -> &Index {
        &self.index
    }
    #[doc = "0x3c - USB peripheral control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x40 - Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}"]
    #[inline(always)]
    pub const fn maxi(&self) -> &Maxi {
        &self.maxi
    }
    #[doc = "0x44 - Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)"]
    #[inline(always)]
    pub const fn cs0_csil(&self) -> &Cs0Csil {
        &self.cs0_csil
    }
    #[doc = "0x48 - Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)"]
    #[inline(always)]
    pub const fn csih(&self) -> &Csih {
        &self.csih
    }
    #[doc = "0x4c - Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}"]
    #[inline(always)]
    pub const fn maxo(&self) -> &Maxo {
        &self.maxo
    }
    #[doc = "0x50 - Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)"]
    #[inline(always)]
    pub const fn csol(&self) -> &Csol {
        &self.csol
    }
    #[doc = "0x54 - Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)"]
    #[inline(always)]
    pub const fn csoh(&self) -> &Csoh {
        &self.csoh
    }
    #[doc = "0x58 - Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)"]
    #[inline(always)]
    pub const fn cnt0_cntl(&self) -> &Cnt0Cntl {
        &self.cnt0_cntl
    }
    #[doc = "0x5c - Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)"]
    #[inline(always)]
    pub const fn cnth(&self) -> &Cnth {
        &self.cnth
    }
    #[doc = "0x80 - Endpoint 0 FIFO"]
    #[inline(always)]
    pub const fn f0(&self) -> &F0 {
        &self.f0
    }
    #[doc = "0x88 - IN/OUT endpoint 1 FIFO"]
    #[inline(always)]
    pub const fn f1(&self) -> &F1 {
        &self.f1
    }
    #[doc = "0x90 - IN/OUT endpoint 2 FIFO"]
    #[inline(always)]
    pub const fn f2(&self) -> &F2 {
        &self.f2
    }
    #[doc = "0x98 - IN/OUT endpoint 3 FIFO"]
    #[inline(always)]
    pub const fn f3(&self) -> &F3 {
        &self.f3
    }
    #[doc = "0xa0 - IN/OUT endpoint 4 FIFO"]
    #[inline(always)]
    pub const fn f4(&self) -> &F4 {
        &self.f4
    }
    #[doc = "0xa8 - IN/OUT endpoint 5 FIFO"]
    #[inline(always)]
    pub const fn f5(&self) -> &F5 {
        &self.f5
    }
}
#[doc = "ADDR (rw) register accessor: Function address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Function address"]
pub mod addr;
#[doc = "POW (rw) register accessor: Power management and control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pow`]
module"]
#[doc(alias = "POW")]
pub type Pow = crate::Reg<pow::PowSpec>;
#[doc = "Power management and control register"]
pub mod pow;
#[doc = "IIF (r) register accessor: Interrupt flags for endpoint 0 and IN endpoints 1-5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iif::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iif`]
module"]
#[doc(alias = "IIF")]
pub type Iif = crate::Reg<iif::IifSpec>;
#[doc = "Interrupt flags for endpoint 0 and IN endpoints 1-5"]
pub mod iif;
#[doc = "OIF (r) register accessor: Interrupt flags for OUT endpoints 1-5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oif::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oif`]
module"]
#[doc(alias = "OIF")]
pub type Oif = crate::Reg<oif::OifSpec>;
#[doc = "Interrupt flags for OUT endpoints 1-5"]
pub mod oif;
#[doc = "CIF (r) register accessor: Common USB interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cif::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cif`]
module"]
#[doc(alias = "CIF")]
pub type Cif = crate::Reg<cif::CifSpec>;
#[doc = "Common USB interrupt flags"]
pub mod cif;
#[doc = "IIE (rw) register accessor: Interrupt enable mask for IN endpoints 1-5 and endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iie`]
module"]
#[doc(alias = "IIE")]
pub type Iie = crate::Reg<iie::IieSpec>;
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0"]
pub mod iie;
#[doc = "OIE (rw) register accessor: Interrupt enable mask for OUT endpoints 1-5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oie`]
module"]
#[doc(alias = "OIE")]
pub type Oie = crate::Reg<oie::OieSpec>;
#[doc = "Interrupt enable mask for OUT endpoints 1-5"]
pub mod oie;
#[doc = "CIE (rw) register accessor: Common USB interrupt enable mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cie`]
module"]
#[doc(alias = "CIE")]
pub type Cie = crate::Reg<cie::CieSpec>;
#[doc = "Common USB interrupt enable mask"]
pub mod cie;
#[doc = "FRML (r) register accessor: Frame number (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frml::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frml`]
module"]
#[doc(alias = "FRML")]
pub type Frml = crate::Reg<frml::FrmlSpec>;
#[doc = "Frame number (low byte)"]
pub mod frml;
#[doc = "FRMH (r) register accessor: Frame number (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmh`]
module"]
#[doc(alias = "FRMH")]
pub type Frmh = crate::Reg<frmh::FrmhSpec>;
#[doc = "Frame number (high byte)"]
pub mod frmh;
#[doc = "INDEX (rw) register accessor: Index register for selecting the endpoint status and control registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`index::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`index::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@index`]
module"]
#[doc(alias = "INDEX")]
pub type Index = crate::Reg<index::IndexSpec>;
#[doc = "Index register for selecting the endpoint status and control registers"]
pub mod index;
#[doc = "CTRL (rw) register accessor: USB peripheral control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "USB peripheral control register"]
pub mod ctrl;
#[doc = "MAXI (rw) register accessor: Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxi`]
module"]
#[doc(alias = "MAXI")]
pub type Maxi = crate::Reg<maxi::MaxiSpec>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}"]
pub mod maxi;
#[doc = "CS0_CSIL (rw) register accessor: Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs0_csil::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs0_csil::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs0_csil`]
module"]
#[doc(alias = "CS0_CSIL")]
pub type Cs0Csil = crate::Reg<cs0_csil::Cs0CsilSpec>;
#[doc = "Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)"]
pub mod cs0_csil;
#[doc = "CSIH (rw) register accessor: Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csih::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csih::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csih`]
module"]
#[doc(alias = "CSIH")]
pub type Csih = crate::Reg<csih::CsihSpec>;
#[doc = "Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)"]
pub mod csih;
#[doc = "MAXO (rw) register accessor: Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxo`]
module"]
#[doc(alias = "MAXO")]
pub type Maxo = crate::Reg<maxo::MaxoSpec>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}"]
pub mod maxo;
#[doc = "CSOL (rw) register accessor: Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csol`]
module"]
#[doc(alias = "CSOL")]
pub type Csol = crate::Reg<csol::CsolSpec>;
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)"]
pub mod csol;
#[doc = "CSOH (rw) register accessor: Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csoh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csoh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csoh`]
module"]
#[doc(alias = "CSOH")]
pub type Csoh = crate::Reg<csoh::CsohSpec>;
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)"]
pub mod csoh;
#[doc = "CNT0_CNTL (r) register accessor: Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt0_cntl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt0_cntl`]
module"]
#[doc(alias = "CNT0_CNTL")]
pub type Cnt0Cntl = crate::Reg<cnt0_cntl::Cnt0CntlSpec>;
#[doc = "Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)"]
pub mod cnt0_cntl;
#[doc = "CNTH (r) register accessor: Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`]
module"]
#[doc(alias = "CNTH")]
pub type Cnth = crate::Reg<cnth::CnthSpec>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)"]
pub mod cnth;
#[doc = "F0 (rw) register accessor: Endpoint 0 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0`]
module"]
pub type F0 = crate::Reg<f0::F0Spec>;
#[doc = "Endpoint 0 FIFO"]
pub mod f0;
#[doc = "F1 (rw) register accessor: IN/OUT endpoint 1 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1`]
module"]
pub type F1 = crate::Reg<f1::F1Spec>;
#[doc = "IN/OUT endpoint 1 FIFO"]
pub mod f1;
#[doc = "F2 (rw) register accessor: IN/OUT endpoint 2 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2`]
module"]
pub type F2 = crate::Reg<f2::F2Spec>;
#[doc = "IN/OUT endpoint 2 FIFO"]
pub mod f2;
#[doc = "F3 (rw) register accessor: IN/OUT endpoint 3 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3`]
module"]
pub type F3 = crate::Reg<f3::F3Spec>;
#[doc = "IN/OUT endpoint 3 FIFO"]
pub mod f3;
#[doc = "F4 (rw) register accessor: IN/OUT endpoint 4 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4`]
module"]
pub type F4 = crate::Reg<f4::F4Spec>;
#[doc = "IN/OUT endpoint 4 FIFO"]
pub mod f4;
#[doc = "F5 (rw) register accessor: IN/OUT endpoint 5 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5`]
module"]
pub type F5 = crate::Reg<f5::F5Spec>;
#[doc = "IN/OUT endpoint 5 FIFO"]
pub mod f5;
