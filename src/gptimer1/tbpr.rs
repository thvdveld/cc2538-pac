#[doc = "Register `TBPR` reader"]
pub struct R(crate::R<TBPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPR` writer"]
pub struct W(crate::W<TBPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPR_SPEC>;
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
impl From<crate::W<TBPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBPSR` reader - GPTM Timer B prescale"]
pub struct TBPSR_R(crate::FieldReader<u8, u8>);
impl TBPSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TBPSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBPSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBPSR` writer - GPTM Timer B prescale"]
pub struct TBPSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn tbpsr(&mut self) -> TBPSR_W {
        TBPSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpr](index.html) module"]
pub struct TBPR_SPEC;
impl crate::RegisterSpec for TBPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpr::R](R) reader structure"]
impl crate::Readable for TBPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbpr::W](W) writer structure"]
impl crate::Writable for TBPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBPR to value 0"]
impl crate::Resettable for TBPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
