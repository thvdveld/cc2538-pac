#[doc = "Register `SRCRESMASK0` reader"]
pub struct R(crate::R<SRCRESMASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCRESMASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCRESMASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCRESMASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCRESMASK0` writer"]
pub struct W(crate::W<SRCRESMASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCRESMASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SRCRESMASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCRESMASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCRESMASK0` reader - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
pub type SRCRESMASK0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCRESMASK0` writer - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
pub type SRCRESMASK0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRCRESMASK0_SPEC, u8, u8, 8, O>;
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
    pub fn srcresmask0(&mut self) -> SRCRESMASK0_W<0> {
        SRCRESMASK0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcresmask0](index.html) module"]
pub struct SRCRESMASK0_SPEC;
impl crate::RegisterSpec for SRCRESMASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcresmask0::R](R) reader structure"]
impl crate::Readable for SRCRESMASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcresmask0::W](W) writer structure"]
impl crate::Writable for SRCRESMASK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCRESMASK0 to value 0"]
impl crate::Resettable for SRCRESMASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
