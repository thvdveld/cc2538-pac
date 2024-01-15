#[doc = "Register `FRML` reader"]
pub type R = crate::R<FRML_SPEC>;
#[doc = "Field `FRAMEL` reader - Bits 7:0 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
pub type FRAMEL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits 7:0 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
    #[inline(always)]
    pub fn framel(&self) -> FRAMEL_R {
        FRAMEL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Frame number (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frml::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRML_SPEC;
impl crate::RegisterSpec for FRML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frml::R`](R) reader structure"]
impl crate::Readable for FRML_SPEC {}
#[doc = "`reset()` method sets FRML to value 0"]
impl crate::Resettable for FRML_SPEC {
    const RESET_VALUE: u32 = 0;
}
