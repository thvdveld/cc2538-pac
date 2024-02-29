#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ivctrl: Ivctrl,
}
impl RegisterBlock {
    #[doc = "0x04 - Analog control register"]
    #[inline(always)]
    pub const fn ivctrl(&self) -> &Ivctrl {
        &self.ivctrl
    }
}
#[doc = "IVCTRL (rw) register accessor: Analog control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivctrl`]
module"]
#[doc(alias = "IVCTRL")]
pub type Ivctrl = crate::Reg<ivctrl::IvctrlSpec>;
#[doc = "Analog control register"]
pub mod ivctrl;
