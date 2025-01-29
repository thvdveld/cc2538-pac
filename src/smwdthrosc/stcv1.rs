#[doc = "Register `STCV1` reader"]
pub type R = crate::R<Stcv1Spec>;
#[doc = "Field `STCV1` reader - Bits \\[15:8\\]
of Sleep Timer capture value"]
pub type Stcv1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits \\[15:8\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv1(&self) -> Stcv1R {
        Stcv1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 1\n\nYou can [`read`](crate::Reg::read) this register and get [`stcv1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stcv1Spec;
impl crate::RegisterSpec for Stcv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcv1::R`](R) reader structure"]
impl crate::Readable for Stcv1Spec {}
#[doc = "`reset()` method sets STCV1 to value 0"]
impl crate::Resettable for Stcv1Spec {
    const RESET_VALUE: u32 = 0;
}
