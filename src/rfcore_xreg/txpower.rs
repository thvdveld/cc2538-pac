#[doc = "Register `TXPOWER` reader"]
pub struct R(crate::R<TXPOWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPOWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPOWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPOWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPOWER` writer"]
pub struct W(crate::W<TXPOWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPOWER_SPEC>;
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
impl From<crate::W<TXPOWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPOWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_POWER` reader - PA power control"]
pub struct PA_POWER_R(crate::FieldReader<u8, u8>);
impl PA_POWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_POWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_POWER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_POWER` writer - PA power control"]
pub struct PA_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_POWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PA_BIAS` reader - PA bias control"]
pub struct PA_BIAS_R(crate::FieldReader<u8, u8>);
impl PA_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_BIAS` writer - PA bias control"]
pub struct PA_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - PA power control"]
    #[inline(always)]
    pub fn pa_power(&self) -> PA_POWER_R {
        PA_POWER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - PA bias control"]
    #[inline(always)]
    pub fn pa_bias(&self) -> PA_BIAS_R {
        PA_BIAS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - PA power control"]
    #[inline(always)]
    pub fn pa_power(&mut self) -> PA_POWER_W {
        PA_POWER_W { w: self }
    }
    #[doc = "Bits 0:3 - PA bias control"]
    #[inline(always)]
    pub fn pa_bias(&mut self) -> PA_BIAS_W {
        PA_BIAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the output power\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpower](index.html) module"]
pub struct TXPOWER_SPEC;
impl crate::RegisterSpec for TXPOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpower::R](R) reader structure"]
impl crate::Readable for TXPOWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpower::W](W) writer structure"]
impl crate::Writable for TXPOWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPOWER to value 0"]
impl crate::Resettable for TXPOWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
