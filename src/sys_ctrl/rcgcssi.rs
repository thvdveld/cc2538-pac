#[doc = "Register `RCGCSSI` reader"]
pub struct R(crate::R<RCGCSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCSSI` writer"]
pub struct W(crate::W<RCGCSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCSSI_SPEC>;
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
impl From<crate::W<RCGCSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI1` reader - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
pub struct SSI1_R(crate::FieldReader<bool, bool>);
impl SSI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSI1` writer - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
pub struct SSI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI1_W<'a> {
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
#[doc = "Field `SSI0` reader - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
pub struct SSI0_R(crate::FieldReader<bool, bool>);
impl SSI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSI0` writer - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
pub struct SSI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI0_W<'a> {
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
    #[doc = "Bit 1 - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
    #[inline(always)]
    pub fn ssi1(&self) -> SSI1_R {
        SSI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
    #[inline(always)]
    pub fn ssi0(&self) -> SSI0_R {
        SSI0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
    #[inline(always)]
    pub fn ssi1(&mut self) -> SSI1_W {
        SSI1_W { w: self }
    }
    #[doc = "Bit 0 - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
    #[inline(always)]
    pub fn ssi0(&mut self) -> SSI0_W {
        SSI0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcssi](index.html) module"]
pub struct RCGCSSI_SPEC;
impl crate::RegisterSpec for RCGCSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgcssi::R](R) reader structure"]
impl crate::Readable for RCGCSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgcssi::W](W) writer structure"]
impl crate::Writable for RCGCSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCSSI to value 0"]
impl crate::Resettable for RCGCSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
