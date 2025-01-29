#[doc = "Register `STCV3` reader"]
pub type R = crate::R<Stcv3Spec>;
#[doc = "Field `STCV3` reader - Bits \\[32:24\\]
of Sleep Timer capture value"]
pub type Stcv3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits \\[32:24\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv3(&self) -> Stcv3R {
        Stcv3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 3\n\nYou can [`read`](crate::Reg::read) this register and get [`stcv3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stcv3Spec;
impl crate::RegisterSpec for Stcv3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcv3::R`](R) reader structure"]
impl crate::Readable for Stcv3Spec {}
#[doc = "`reset()` method sets STCV3 to value 0"]
impl crate::Resettable for Stcv3Spec {
    const RESET_VALUE: u32 = 0;
}
