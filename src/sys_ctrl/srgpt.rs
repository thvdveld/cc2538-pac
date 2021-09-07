#[doc = "Register `SRGPT` reader"]
pub struct R(crate::R<SRGPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRGPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRGPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRGPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRGPT` writer"]
pub struct W(crate::W<SRGPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRGPT_SPEC>;
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
impl From<crate::W<SRGPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRGPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPT3` reader - 0: GPT3 module is not reset 1: GPT3 module is reset"]
pub struct GPT3_R(crate::FieldReader<bool, bool>);
impl GPT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPT3` writer - 0: GPT3 module is not reset 1: GPT3 module is reset"]
pub struct GPT3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `GPT2` reader - 0: GPT2 module is not reset 1: GPT2 module is reset"]
pub struct GPT2_R(crate::FieldReader<bool, bool>);
impl GPT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPT2` writer - 0: GPT2 module is not reset 1: GPT2 module is reset"]
pub struct GPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `GPT1` reader - 0: GPT1 module is not reset 1: GPT1 module is reset"]
pub struct GPT1_R(crate::FieldReader<bool, bool>);
impl GPT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPT1` writer - 0: GPT1 module is not reset 1: GPT1 module is reset"]
pub struct GPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `GPT0` reader - 0: GPT0 module is not reset 1: GPT0 module is reset"]
pub struct GPT0_R(crate::FieldReader<bool, bool>);
impl GPT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPT0` writer - 0: GPT0 module is not reset 1: GPT0 module is reset"]
pub struct GPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - 0: GPT3 module is not reset 1: GPT3 module is reset"]
    #[inline(always)]
    pub fn gpt3(&self) -> GPT3_R {
        GPT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0: GPT2 module is not reset 1: GPT2 module is reset"]
    #[inline(always)]
    pub fn gpt2(&self) -> GPT2_R {
        GPT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0: GPT1 module is not reset 1: GPT1 module is reset"]
    #[inline(always)]
    pub fn gpt1(&self) -> GPT1_R {
        GPT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0: GPT0 module is not reset 1: GPT0 module is reset"]
    #[inline(always)]
    pub fn gpt0(&self) -> GPT0_R {
        GPT0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: GPT3 module is not reset 1: GPT3 module is reset"]
    #[inline(always)]
    pub fn gpt3(&mut self) -> GPT3_W {
        GPT3_W { w: self }
    }
    #[doc = "Bit 2 - 0: GPT2 module is not reset 1: GPT2 module is reset"]
    #[inline(always)]
    pub fn gpt2(&mut self) -> GPT2_W {
        GPT2_W { w: self }
    }
    #[doc = "Bit 1 - 0: GPT1 module is not reset 1: GPT1 module is reset"]
    #[inline(always)]
    pub fn gpt1(&mut self) -> GPT1_W {
        GPT1_W { w: self }
    }
    #[doc = "Bit 0 - 0: GPT0 module is not reset 1: GPT0 module is reset"]
    #[inline(always)]
    pub fn gpt0(&mut self) -> GPT0_W {
        GPT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the reset for GPT\\[3:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srgpt](index.html) module"]
pub struct SRGPT_SPEC;
impl crate::RegisterSpec for SRGPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srgpt::R](R) reader structure"]
impl crate::Readable for SRGPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srgpt::W](W) writer structure"]
impl crate::Writable for SRGPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRGPT to value 0"]
impl crate::Resettable for SRGPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
