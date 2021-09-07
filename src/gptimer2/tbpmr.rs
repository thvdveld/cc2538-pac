#[doc = "Register `TBPMR` reader"]
pub struct R(crate::R<TBPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPMR` writer"]
pub struct W(crate::W<TBPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPMR_SPEC>;
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
impl From<crate::W<TBPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBPSR` reader - GPTM Timer B prescale match"]
pub struct TBPSR_R(crate::FieldReader<u8, u8>);
impl TBPSR_R {
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
#[doc = "Field `TBPSR` writer - GPTM Timer B prescale match"]
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
    #[doc = "Bits 0:7 - GPTM Timer B prescale match"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TBPSR_R {
        TBPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B prescale match"]
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
#[doc = "GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpmr](index.html) module"]
pub struct TBPMR_SPEC;
impl crate::RegisterSpec for TBPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpmr::R](R) reader structure"]
impl crate::Readable for TBPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbpmr::W](W) writer structure"]
impl crate::Writable for TBPMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBPMR to value 0"]
impl crate::Resettable for TBPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
