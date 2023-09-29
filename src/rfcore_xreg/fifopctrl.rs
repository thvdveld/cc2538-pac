#[doc = "Register `FIFOPCTRL` reader"]
pub type R = crate::R<FIFOPCTRL_SPEC>;
#[doc = "Register `FIFOPCTRL` writer"]
pub type W = crate::W<FIFOPCTRL_SPEC>;
#[doc = "Field `FIFOP_THR` reader - Threshold used when generating FIFOP signal"]
pub type FIFOP_THR_R = crate::FieldReader;
#[doc = "Field `FIFOP_THR` writer - Threshold used when generating FIFOP signal"]
pub type FIFOP_THR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Threshold used when generating FIFOP signal"]
    #[inline(always)]
    pub fn fifop_thr(&self) -> FIFOP_THR_R {
        FIFOP_THR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Threshold used when generating FIFOP signal"]
    #[inline(always)]
    #[must_use]
    pub fn fifop_thr(&mut self) -> FIFOP_THR_W<FIFOPCTRL_SPEC, 0> {
        FIFOP_THR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FIFOP threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifopctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifopctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOPCTRL_SPEC;
impl crate::RegisterSpec for FIFOPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifopctrl::R`](R) reader structure"]
impl crate::Readable for FIFOPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifopctrl::W`](W) writer structure"]
impl crate::Writable for FIFOPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOPCTRL to value 0"]
impl crate::Resettable for FIFOPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
