#[doc = "Register `TAPMR` reader"]
pub struct R(crate::R<TAPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAPMR` writer"]
pub struct W(crate::W<TAPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAPMR_SPEC>;
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
impl From<crate::W<TAPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAPSR` reader - GPTM Timer A prescale match"]
pub struct TAPSR_R(crate::FieldReader<u8, u8>);
impl TAPSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAPSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAPSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAPSR` writer - GPTM Timer A prescale match"]
pub struct TAPSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPTM Timer A prescale match"]
    #[inline(always)]
    pub fn tapsr(&self) -> TAPSR_R {
        TAPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer A prescale match"]
    #[inline(always)]
    pub fn tapsr(&mut self) -> TAPSR_W {
        TAPSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapmr](index.html) module"]
pub struct TAPMR_SPEC;
impl crate::RegisterSpec for TAPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tapmr::R](R) reader structure"]
impl crate::Readable for TAPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tapmr::W](W) writer structure"]
impl crate::Writable for TAPMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAPMR to value 0"]
impl crate::Resettable for TAPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
