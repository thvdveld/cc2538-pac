#[doc = "Register `SRCRESMASK0` reader"]
pub type R = crate::R<SRCRESMASK0_SPEC>;
#[doc = "Register `SRCRESMASK0` writer"]
pub type W = crate::W<SRCRESMASK0_SPEC>;
#[doc = "Field `SRCRESMASK0` reader - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
pub type SRCRESMASK0_R = crate::FieldReader;
#[doc = "Field `SRCRESMASK0` writer - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
pub type SRCRESMASK0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask0(&self) -> SRCRESMASK0_R {
        SRCRESMASK0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
    #[inline(always)]
    #[must_use]
    pub fn srcresmask0(&mut self) -> SRCRESMASK0_W<SRCRESMASK0_SPEC, 0> {
        SRCRESMASK0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCRESMASK0_SPEC;
impl crate::RegisterSpec for SRCRESMASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcresmask0::R`](R) reader structure"]
impl crate::Readable for SRCRESMASK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcresmask0::W`](W) writer structure"]
impl crate::Writable for SRCRESMASK0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCRESMASK0 to value 0"]
impl crate::Resettable for SRCRESMASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
