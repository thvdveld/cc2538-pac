#[doc = "Register `STCV2` reader"]
pub type R = crate::R<Stcv2Spec>;
#[doc = "Field `STCV2` reader - Bits \\[23:16\\]
of Sleep Timer capture value"]
pub type Stcv2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits \\[23:16\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv2(&self) -> Stcv2R {
        Stcv2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 2\n\nYou can [`read`](crate::Reg::read) this register and get [`stcv2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stcv2Spec;
impl crate::RegisterSpec for Stcv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcv2::R`](R) reader structure"]
impl crate::Readable for Stcv2Spec {}
#[doc = "`reset()` method sets STCV2 to value 0"]
impl crate::Resettable for Stcv2Spec {
    const RESET_VALUE: u32 = 0;
}
