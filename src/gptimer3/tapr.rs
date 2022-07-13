#[doc = "Register `TAPR` reader"]
pub struct R(crate::R<TAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAPR` writer"]
pub struct W(crate::W<TAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAPR_SPEC>;
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
impl From<crate::W<TAPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAPSR` reader - GPTM Timer A prescale"]
pub type TAPSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAPSR` writer - GPTM Timer A prescale"]
pub type TAPSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer A prescale"]
    #[inline(always)]
    pub fn tapsr(&self) -> TAPSR_R {
        TAPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer A prescale"]
    #[inline(always)]
    pub fn tapsr(&mut self) -> TAPSR_W<0> {
        TAPSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapr](index.html) module"]
pub struct TAPR_SPEC;
impl crate::RegisterSpec for TAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tapr::R](R) reader structure"]
impl crate::Readable for TAPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tapr::W](W) writer structure"]
impl crate::Writable for TAPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAPR to value 0"]
impl crate::Resettable for TAPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
