#[doc = "Register `STCV1` reader"]
pub type R = crate::R<STCV1_SPEC>;
#[doc = "Field `STCV1` reader - Bits \\[15:8\\]
of Sleep Timer capture value"]
pub type STCV1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits \\[15:8\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv1(&self) -> STCV1_R {
        STCV1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCV1_SPEC;
impl crate::RegisterSpec for STCV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcv1::R`](R) reader structure"]
impl crate::Readable for STCV1_SPEC {}
#[doc = "`reset()` method sets STCV1 to value 0"]
impl crate::Resettable for STCV1_SPEC {
    const RESET_VALUE: u32 = 0;
}
