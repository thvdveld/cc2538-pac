#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function address"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x04 - Power management and control register"]
    pub pow: crate::Reg<pow::POW_SPEC>,
    #[doc = "0x08 - Interrupt flags for endpoint 0 and IN endpoints 1-5"]
    pub iif: crate::Reg<iif::IIF_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Interrupt flags for OUT endpoints 1-5"]
    pub oif: crate::Reg<oif::OIF_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x18 - Common USB interrupt flags"]
    pub cif: crate::Reg<cif::CIF_SPEC>,
    #[doc = "0x1c - Interrupt enable mask for IN endpoints 1-5 and endpoint 0"]
    pub iie: crate::Reg<iie::IIE_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - Interrupt enable mask for OUT endpoints 1-5"]
    pub oie: crate::Reg<oie::OIE_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - Common USB interrupt enable mask"]
    pub cie: crate::Reg<cie::CIE_SPEC>,
    #[doc = "0x30 - Frame number (low byte)"]
    pub frml: crate::Reg<frml::FRML_SPEC>,
    #[doc = "0x34 - Frame number (high byte)"]
    pub frmh: crate::Reg<frmh::FRMH_SPEC>,
    #[doc = "0x38 - Index register for selecting the endpoint status and control registers"]
    pub index: crate::Reg<index::INDEX_SPEC>,
    #[doc = "0x3c - USB peripheral control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x40 - Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}"]
    pub maxi: crate::Reg<maxi::MAXI_SPEC>,
    #[doc = "0x44 - Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)"]
    pub cs0_csil: crate::Reg<cs0_csil::CS0_CSIL_SPEC>,
    #[doc = "0x48 - Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)"]
    pub csih: crate::Reg<csih::CSIH_SPEC>,
    #[doc = "0x4c - Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}"]
    pub maxo: crate::Reg<maxo::MAXO_SPEC>,
    #[doc = "0x50 - Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)"]
    pub csol: crate::Reg<csol::CSOL_SPEC>,
    #[doc = "0x54 - Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)"]
    pub csoh: crate::Reg<csoh::CSOH_SPEC>,
    #[doc = "0x58 - Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)"]
    pub cnt0_cntl: crate::Reg<cnt0_cntl::CNT0_CNTL_SPEC>,
    #[doc = "0x5c - Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)"]
    pub cnth: crate::Reg<cnth::CNTH_SPEC>,
    _reserved20: [u8; 0x20],
    #[doc = "0x80 - Endpoint 0 FIFO"]
    pub f0: crate::Reg<f0::F0_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x88 - IN/OUT endpoint 1 FIFO"]
    pub f1: crate::Reg<f1::F1_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x90 - IN/OUT endpoint 2 FIFO"]
    pub f2: crate::Reg<f2::F2_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x98 - IN/OUT endpoint 3 FIFO"]
    pub f3: crate::Reg<f3::F3_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0xa0 - IN/OUT endpoint 4 FIFO"]
    pub f4: crate::Reg<f4::F4_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0xa8 - IN/OUT endpoint 5 FIFO"]
    pub f5: crate::Reg<f5::F5_SPEC>,
}
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Function address"]
pub mod addr;
#[doc = "POW register accessor: an alias for `Reg<POW_SPEC>`"]
pub type POW = crate::Reg<pow::POW_SPEC>;
#[doc = "Power management and control register"]
pub mod pow;
#[doc = "IIF register accessor: an alias for `Reg<IIF_SPEC>`"]
pub type IIF = crate::Reg<iif::IIF_SPEC>;
#[doc = "Interrupt flags for endpoint 0 and IN endpoints 1-5"]
pub mod iif;
#[doc = "OIF register accessor: an alias for `Reg<OIF_SPEC>`"]
pub type OIF = crate::Reg<oif::OIF_SPEC>;
#[doc = "Interrupt flags for OUT endpoints 1-5"]
pub mod oif;
#[doc = "CIF register accessor: an alias for `Reg<CIF_SPEC>`"]
pub type CIF = crate::Reg<cif::CIF_SPEC>;
#[doc = "Common USB interrupt flags"]
pub mod cif;
#[doc = "IIE register accessor: an alias for `Reg<IIE_SPEC>`"]
pub type IIE = crate::Reg<iie::IIE_SPEC>;
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0"]
pub mod iie;
#[doc = "OIE register accessor: an alias for `Reg<OIE_SPEC>`"]
pub type OIE = crate::Reg<oie::OIE_SPEC>;
#[doc = "Interrupt enable mask for OUT endpoints 1-5"]
pub mod oie;
#[doc = "CIE register accessor: an alias for `Reg<CIE_SPEC>`"]
pub type CIE = crate::Reg<cie::CIE_SPEC>;
#[doc = "Common USB interrupt enable mask"]
pub mod cie;
#[doc = "FRML register accessor: an alias for `Reg<FRML_SPEC>`"]
pub type FRML = crate::Reg<frml::FRML_SPEC>;
#[doc = "Frame number (low byte)"]
pub mod frml;
#[doc = "FRMH register accessor: an alias for `Reg<FRMH_SPEC>`"]
pub type FRMH = crate::Reg<frmh::FRMH_SPEC>;
#[doc = "Frame number (high byte)"]
pub mod frmh;
#[doc = "INDEX register accessor: an alias for `Reg<INDEX_SPEC>`"]
pub type INDEX = crate::Reg<index::INDEX_SPEC>;
#[doc = "Index register for selecting the endpoint status and control registers"]
pub mod index;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "USB peripheral control register"]
pub mod ctrl;
#[doc = "MAXI register accessor: an alias for `Reg<MAXI_SPEC>`"]
pub type MAXI = crate::Reg<maxi::MAXI_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}"]
pub mod maxi;
#[doc = "CS0_CSIL register accessor: an alias for `Reg<CS0_CSIL_SPEC>`"]
pub type CS0_CSIL = crate::Reg<cs0_csil::CS0_CSIL_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)"]
pub mod cs0_csil;
#[doc = "CSIH register accessor: an alias for `Reg<CSIH_SPEC>`"]
pub type CSIH = crate::Reg<csih::CSIH_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)"]
pub mod csih;
#[doc = "MAXO register accessor: an alias for `Reg<MAXO_SPEC>`"]
pub type MAXO = crate::Reg<maxo::MAXO_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}"]
pub mod maxo;
#[doc = "CSOL register accessor: an alias for `Reg<CSOL_SPEC>`"]
pub type CSOL = crate::Reg<csol::CSOL_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)"]
pub mod csol;
#[doc = "CSOH register accessor: an alias for `Reg<CSOH_SPEC>`"]
pub type CSOH = crate::Reg<csoh::CSOH_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)"]
pub mod csoh;
#[doc = "CNT0_CNTL register accessor: an alias for `Reg<CNT0_CNTL_SPEC>`"]
pub type CNT0_CNTL = crate::Reg<cnt0_cntl::CNT0_CNTL_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)"]
pub mod cnt0_cntl;
#[doc = "CNTH register accessor: an alias for `Reg<CNTH_SPEC>`"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)"]
pub mod cnth;
#[doc = "F0 register accessor: an alias for `Reg<F0_SPEC>`"]
pub type F0 = crate::Reg<f0::F0_SPEC>;
#[doc = "Endpoint 0 FIFO"]
pub mod f0;
#[doc = "F1 register accessor: an alias for `Reg<F1_SPEC>`"]
pub type F1 = crate::Reg<f1::F1_SPEC>;
#[doc = "IN/OUT endpoint 1 FIFO"]
pub mod f1;
#[doc = "F2 register accessor: an alias for `Reg<F2_SPEC>`"]
pub type F2 = crate::Reg<f2::F2_SPEC>;
#[doc = "IN/OUT endpoint 2 FIFO"]
pub mod f2;
#[doc = "F3 register accessor: an alias for `Reg<F3_SPEC>`"]
pub type F3 = crate::Reg<f3::F3_SPEC>;
#[doc = "IN/OUT endpoint 3 FIFO"]
pub mod f3;
#[doc = "F4 register accessor: an alias for `Reg<F4_SPEC>`"]
pub type F4 = crate::Reg<f4::F4_SPEC>;
#[doc = "IN/OUT endpoint 4 FIFO"]
pub mod f4;
#[doc = "F5 register accessor: an alias for `Reg<F5_SPEC>`"]
pub type F5 = crate::Reg<f5::F5_SPEC>;
#[doc = "IN/OUT endpoint 5 FIFO"]
pub mod f5;
