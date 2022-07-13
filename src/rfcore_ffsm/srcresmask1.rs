#[doc = "Register `SRCRESMASK1` reader"]
pub struct R(crate::R<SRCRESMASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCRESMASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCRESMASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCRESMASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCRESMASK1` writer"]
pub struct W(crate::W<SRCRESMASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCRESMASK1_SPEC>;
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
impl From<crate::W<SRCRESMASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCRESMASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCRESMASK1` reader - Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
pub type SRCRESMASK1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCRESMASK1` writer - Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
pub type SRCRESMASK1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRCRESMASK1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask1(&self) -> SRCRESMASK1_R {
        SRCRESMASK1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask1(&mut self) -> SRCRESMASK1_W<0> {
        SRCRESMASK1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcresmask1](index.html) module"]
pub struct SRCRESMASK1_SPEC;
impl crate::RegisterSpec for SRCRESMASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcresmask1::R](R) reader structure"]
impl crate::Readable for SRCRESMASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcresmask1::W](W) writer structure"]
impl crate::Writable for SRCRESMASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCRESMASK1 to value 0"]
impl crate::Resettable for SRCRESMASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
