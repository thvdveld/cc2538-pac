#[doc = "Register `TBPMR` reader"]
pub type R = crate::R<TBPMR_SPEC>;
#[doc = "Register `TBPMR` writer"]
pub type W = crate::W<TBPMR_SPEC>;
#[doc = "Field `TBPSR` reader - GPTM Timer B prescale match"]
pub type TBPSR_R = crate::FieldReader;
#[doc = "Field `TBPSR` writer - GPTM Timer B prescale match"]
pub type TBPSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer B prescale match"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TBPSR_R {
        TBPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B prescale match"]
    #[inline(always)]
    #[must_use]
    pub fn tbpsr(&mut self) -> TBPSR_W<TBPMR_SPEC, 0> {
        TBPSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBPMR_SPEC;
impl crate::RegisterSpec for TBPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpmr::R`](R) reader structure"]
impl crate::Readable for TBPMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbpmr::W`](W) writer structure"]
impl crate::Writable for TBPMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBPMR to value 0"]
impl crate::Resettable for TBPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
