#[doc = "Register `TAILR` reader"]
pub type R = crate::R<TAILR_SPEC>;
#[doc = "Register `TAILR` writer"]
pub type W = crate::W<TAILR_SPEC>;
#[doc = "Field `TAILR` reader - GPTM A interval load register"]
pub type TAILR_R = crate::FieldReader<u32>;
#[doc = "Field `TAILR` writer - GPTM A interval load register"]
pub type TAILR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - GPTM A interval load register"]
    #[inline(always)]
    pub fn tailr(&self) -> TAILR_R {
        TAILR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM A interval load register"]
    #[inline(always)]
    #[must_use]
    pub fn tailr(&mut self) -> TAILR_W<TAILR_SPEC, 0> {
        TAILR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tailr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tailr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAILR_SPEC;
impl crate::RegisterSpec for TAILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tailr::R`](R) reader structure"]
impl crate::Readable for TAILR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tailr::W`](W) writer structure"]
impl crate::Writable for TAILR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAILR to value 0"]
impl crate::Resettable for TAILR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
