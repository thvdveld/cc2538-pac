#[doc = "Register `FRMH` reader"]
pub type R = crate::R<FRMH_SPEC>;
#[doc = "Field `FRAMEH` reader - Bits 10:8 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
pub type FRAMEH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Bits 10:8 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
    #[inline(always)]
    pub fn frameh(&self) -> FRAMEH_R {
        FRAMEH_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Frame number (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRMH_SPEC;
impl crate::RegisterSpec for FRMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmh::R`](R) reader structure"]
impl crate::Readable for FRMH_SPEC {}
#[doc = "`reset()` method sets FRMH to value 0"]
impl crate::Resettable for FRMH_SPEC {
    const RESET_VALUE: u32 = 0;
}
