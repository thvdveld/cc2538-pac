#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function address"]
    pub addr: ADDR,
    #[doc = "0x04 - Power management and control register"]
    pub pow: POW,
    #[doc = "0x08 - Interrupt flags for endpoint 0 and IN endpoints 1-5"]
    pub iif: IIF,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Interrupt flags for OUT endpoints 1-5"]
    pub oif: OIF,
    _reserved4: [u8; 0x04],
    #[doc = "0x18 - Common USB interrupt flags"]
    pub cif: CIF,
    #[doc = "0x1c - Interrupt enable mask for IN endpoints 1-5 and endpoint 0"]
    pub iie: IIE,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - Interrupt enable mask for OUT endpoints 1-5"]
    pub oie: OIE,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - Common USB interrupt enable mask"]
    pub cie: CIE,
    #[doc = "0x30 - Frame number (low byte)"]
    pub frml: FRML,
    #[doc = "0x34 - Frame number (high byte)"]
    pub frmh: FRMH,
    #[doc = "0x38 - Index register for selecting the endpoint status and control registers"]
    pub index: INDEX,
    #[doc = "0x3c - USB peripheral control register"]
    pub ctrl: CTRL,
    #[doc = "0x40 - Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}"]
    pub maxi: MAXI,
    #[doc = "0x44 - Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)"]
    pub cs0_csil: CS0_CSIL,
    #[doc = "0x48 - Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)"]
    pub csih: CSIH,
    #[doc = "0x4c - Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}"]
    pub maxo: MAXO,
    #[doc = "0x50 - Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)"]
    pub csol: CSOL,
    #[doc = "0x54 - Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)"]
    pub csoh: CSOH,
    #[doc = "0x58 - Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)"]
    pub cnt0_cntl: CNT0_CNTL,
    #[doc = "0x5c - Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)"]
    pub cnth: CNTH,
    _reserved20: [u8; 0x20],
    #[doc = "0x80 - Endpoint 0 FIFO"]
    pub f0: F0,
    _reserved21: [u8; 0x04],
    #[doc = "0x88 - IN/OUT endpoint 1 FIFO"]
    pub f1: F1,
    _reserved22: [u8; 0x04],
    #[doc = "0x90 - IN/OUT endpoint 2 FIFO"]
    pub f2: F2,
    _reserved23: [u8; 0x04],
    #[doc = "0x98 - IN/OUT endpoint 3 FIFO"]
    pub f3: F3,
    _reserved24: [u8; 0x04],
    #[doc = "0xa0 - IN/OUT endpoint 4 FIFO"]
    pub f4: F4,
    _reserved25: [u8; 0x04],
    #[doc = "0xa8 - IN/OUT endpoint 5 FIFO"]
    pub f5: F5,
}
#[doc = "ADDR (rw) register accessor: Function address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Function address"]
pub mod addr;
#[doc = "POW (rw) register accessor: Power management and control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pow`]
module"]
pub type POW = crate::Reg<pow::POW_SPEC>;
#[doc = "Power management and control register"]
pub mod pow;
#[doc = "IIF (r) register accessor: Interrupt flags for endpoint 0 and IN endpoints 1-5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iif::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iif`]
module"]
pub type IIF = crate::Reg<iif::IIF_SPEC>;
#[doc = "Interrupt flags for endpoint 0 and IN endpoints 1-5"]
pub mod iif;
#[doc = "OIF (r) register accessor: Interrupt flags for OUT endpoints 1-5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oif::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oif`]
module"]
pub type OIF = crate::Reg<oif::OIF_SPEC>;
#[doc = "Interrupt flags for OUT endpoints 1-5"]
pub mod oif;
#[doc = "CIF (r) register accessor: Common USB interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cif::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cif`]
module"]
pub type CIF = crate::Reg<cif::CIF_SPEC>;
#[doc = "Common USB interrupt flags"]
pub mod cif;
#[doc = "IIE (rw) register accessor: Interrupt enable mask for IN endpoints 1-5 and endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iie`]
module"]
pub type IIE = crate::Reg<iie::IIE_SPEC>;
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0"]
pub mod iie;
#[doc = "OIE (rw) register accessor: Interrupt enable mask for OUT endpoints 1-5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oie`]
module"]
pub type OIE = crate::Reg<oie::OIE_SPEC>;
#[doc = "Interrupt enable mask for OUT endpoints 1-5"]
pub mod oie;
#[doc = "CIE (rw) register accessor: Common USB interrupt enable mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cie`]
module"]
pub type CIE = crate::Reg<cie::CIE_SPEC>;
#[doc = "Common USB interrupt enable mask"]
pub mod cie;
#[doc = "FRML (r) register accessor: Frame number (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frml::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`frml`]
module"]
pub type FRML = crate::Reg<frml::FRML_SPEC>;
#[doc = "Frame number (low byte)"]
pub mod frml;
#[doc = "FRMH (r) register accessor: Frame number (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`frmh`]
module"]
pub type FRMH = crate::Reg<frmh::FRMH_SPEC>;
#[doc = "Frame number (high byte)"]
pub mod frmh;
#[doc = "INDEX (rw) register accessor: Index register for selecting the endpoint status and control registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`index::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`index::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`index`]
module"]
pub type INDEX = crate::Reg<index::INDEX_SPEC>;
#[doc = "Index register for selecting the endpoint status and control registers"]
pub mod index;
#[doc = "CTRL (rw) register accessor: USB peripheral control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "USB peripheral control register"]
pub mod ctrl;
#[doc = "MAXI (rw) register accessor: Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maxi`]
module"]
pub type MAXI = crate::Reg<maxi::MAXI_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}"]
pub mod maxi;
#[doc = "CS0_CSIL (rw) register accessor: Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs0_csil::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs0_csil::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cs0_csil`]
module"]
pub type CS0_CSIL = crate::Reg<cs0_csil::CS0_CSIL_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)"]
pub mod cs0_csil;
#[doc = "CSIH (rw) register accessor: Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csih::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csih::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csih`]
module"]
pub type CSIH = crate::Reg<csih::CSIH_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)"]
pub mod csih;
#[doc = "MAXO (rw) register accessor: Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maxo`]
module"]
pub type MAXO = crate::Reg<maxo::MAXO_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}"]
pub mod maxo;
#[doc = "CSOL (rw) register accessor: Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csol`]
module"]
pub type CSOL = crate::Reg<csol::CSOL_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)"]
pub mod csol;
#[doc = "CSOH (rw) register accessor: Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csoh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csoh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csoh`]
module"]
pub type CSOH = crate::Reg<csoh::CSOH_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)"]
pub mod csoh;
#[doc = "CNT0_CNTL (r) register accessor: Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt0_cntl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt0_cntl`]
module"]
pub type CNT0_CNTL = crate::Reg<cnt0_cntl::CNT0_CNTL_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)"]
pub mod cnt0_cntl;
#[doc = "CNTH (r) register accessor: Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnth`]
module"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)"]
pub mod cnth;
#[doc = "F0 (rw) register accessor: Endpoint 0 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f0`]
module"]
pub type F0 = crate::Reg<f0::F0_SPEC>;
#[doc = "Endpoint 0 FIFO"]
pub mod f0;
#[doc = "F1 (rw) register accessor: IN/OUT endpoint 1 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f1`]
module"]
pub type F1 = crate::Reg<f1::F1_SPEC>;
#[doc = "IN/OUT endpoint 1 FIFO"]
pub mod f1;
#[doc = "F2 (rw) register accessor: IN/OUT endpoint 2 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f2`]
module"]
pub type F2 = crate::Reg<f2::F2_SPEC>;
#[doc = "IN/OUT endpoint 2 FIFO"]
pub mod f2;
#[doc = "F3 (rw) register accessor: IN/OUT endpoint 3 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f3`]
module"]
pub type F3 = crate::Reg<f3::F3_SPEC>;
#[doc = "IN/OUT endpoint 3 FIFO"]
pub mod f3;
#[doc = "F4 (rw) register accessor: IN/OUT endpoint 4 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f4`]
module"]
pub type F4 = crate::Reg<f4::F4_SPEC>;
#[doc = "IN/OUT endpoint 4 FIFO"]
pub mod f4;
#[doc = "F5 (rw) register accessor: IN/OUT endpoint 5 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f5`]
module"]
pub type F5 = crate::Reg<f5::F5_SPEC>;
#[doc = "IN/OUT endpoint 5 FIFO"]
pub mod f5;
