#[doc = "Register `GPT` reader"]
pub struct R(crate::R<GPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPT` writer"]
pub struct W(crate::W<GPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPT_SPEC>;
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
impl From<crate::W<GPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTIDOE` reader - GPTimer increment/decrement override enable"]
pub struct GPTIDOE_R(crate::FieldReader<bool, bool>);
impl GPTIDOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPTIDOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPTIDOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPTIDOE` writer - GPTimer increment/decrement override enable"]
pub struct GPTIDOE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTIDOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `GPTIDOV` reader - GPTimer increment/decrement override value"]
pub struct GPTIDOV_R(crate::FieldReader<u8, u8>);
impl GPTIDOV_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPTIDOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPTIDOV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPTIDOV` writer - GPTimer increment/decrement override value"]
pub struct GPTIDOV_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTIDOV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - GPTimer increment/decrement override enable"]
    #[inline(always)]
    pub fn gptidoe(&self) -> GPTIDOE_R {
        GPTIDOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - GPTimer increment/decrement override value"]
    #[inline(always)]
    pub fn gptidov(&self) -> GPTIDOV_R {
        GPTIDOV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - GPTimer increment/decrement override enable"]
    #[inline(always)]
    pub fn gptidoe(&mut self) -> GPTIDOE_W {
        GPTIDOE_W { w: self }
    }
    #[doc = "Bits 0:4 - GPTimer increment/decrement override value"]
    #[inline(always)]
    pub fn gptidov(&mut self) -> GPTIDOV_W {
        GPTIDOV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTIMER override values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt](index.html) module"]
pub struct GPT_SPEC;
impl crate::RegisterSpec for GPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpt::R](R) reader structure"]
impl crate::Readable for GPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpt::W](W) writer structure"]
impl crate::Writable for GPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPT to value 0"]
impl crate::Resettable for GPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
