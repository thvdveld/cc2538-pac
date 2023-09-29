#[doc = "Register `TAMATCHR` reader"]
pub type R = crate::R<TAMATCHR_SPEC>;
#[doc = "Register `TAMATCHR` writer"]
pub type W = crate::W<TAMATCHR_SPEC>;
#[doc = "Field `TAMR` reader - GPTM Timer A match register"]
pub type TAMR_R = crate::FieldReader<u32>;
#[doc = "Field `TAMR` writer - GPTM Timer A match register"]
pub type TAMR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A match register"]
    #[inline(always)]
    pub fn tamr(&self) -> TAMR_R {
        TAMR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM Timer A match register"]
    #[inline(always)]
    #[must_use]
    pub fn tamr(&mut self) -> TAMR_W<TAMATCHR_SPEC, 0> {
        TAMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamatchr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamatchr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMATCHR_SPEC;
impl crate::RegisterSpec for TAMATCHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamatchr::R`](R) reader structure"]
impl crate::Readable for TAMATCHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamatchr::W`](W) writer structure"]
impl crate::Writable for TAMATCHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMATCHR to value 0"]
impl crate::Resettable for TAMATCHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
