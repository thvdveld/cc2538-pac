#[doc = "Register `TAMATCHR` reader"]
pub struct R(crate::R<TAMATCHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMATCHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMATCHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMATCHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMATCHR` writer"]
pub struct W(crate::W<TAMATCHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMATCHR_SPEC>;
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
impl From<crate::W<TAMATCHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMATCHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMR` reader - GPTM Timer A match register"]
pub struct TAMR_R(crate::FieldReader<u32, u32>);
impl TAMR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TAMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMR` writer - GPTM Timer A match register"]
pub struct TAMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A match register"]
    #[inline(always)]
    pub fn tamr(&self) -> TAMR_R {
        TAMR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM Timer A match register"]
    #[inline(always)]
    pub fn tamr(&mut self) -> TAMR_W {
        TAMR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamatchr](index.html) module"]
pub struct TAMATCHR_SPEC;
impl crate::RegisterSpec for TAMATCHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamatchr::R](R) reader structure"]
impl crate::Readable for TAMATCHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamatchr::W](W) writer structure"]
impl crate::Writable for TAMATCHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMATCHR to value 0"]
impl crate::Resettable for TAMATCHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
