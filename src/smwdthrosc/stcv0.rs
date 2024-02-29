#[doc = "Register `STCV0` reader"]
pub type R = crate::R<Stcv0Spec>;
#[doc = "Field `STCV0` reader - Bits \\[7:0\\]
of Sleep Timer capture value"]
pub type Stcv0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits \\[7:0\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv0(&self) -> Stcv0R {
        Stcv0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcv0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stcv0Spec;
impl crate::RegisterSpec for Stcv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcv0::R`](R) reader structure"]
impl crate::Readable for Stcv0Spec {}
#[doc = "`reset()` method sets STCV0 to value 0"]
impl crate::Resettable for Stcv0Spec {
    const RESET_VALUE: u32 = 0;
}
