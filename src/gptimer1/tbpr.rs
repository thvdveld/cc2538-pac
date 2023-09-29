#[doc = "Register `TBPR` reader"]
pub type R = crate::R<TBPR_SPEC>;
#[doc = "Register `TBPR` writer"]
pub type W = crate::W<TBPR_SPEC>;
#[doc = "Field `TBPSR` reader - GPTM Timer B prescale"]
pub type TBPSR_R = crate::FieldReader;
#[doc = "Field `TBPSR` writer - GPTM Timer B prescale"]
pub type TBPSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer B prescale"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TBPSR_R {
        TBPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B prescale"]
    #[inline(always)]
    #[must_use]
    pub fn tbpsr(&mut self) -> TBPSR_W<TBPR_SPEC, 0> {
        TBPSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBPR_SPEC;
impl crate::RegisterSpec for TBPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpr::R`](R) reader structure"]
impl crate::Readable for TBPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbpr::W`](W) writer structure"]
impl crate::Writable for TBPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBPR to value 0"]
impl crate::Resettable for TBPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
