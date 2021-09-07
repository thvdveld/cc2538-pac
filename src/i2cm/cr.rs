#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFE` reader - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
pub struct SFE_R(crate::FieldReader<bool, bool>);
impl SFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFE` writer - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
pub struct SFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `MFE` reader - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
pub struct MFE_R(crate::FieldReader<bool, bool>);
impl MFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFE` writer - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
pub struct MFE_W<'a> {
    w: &'a mut W,
}
impl<'a> MFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `LPBK` reader - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
pub struct LPBK_R(crate::FieldReader<bool, bool>);
impl LPBK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPBK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPBK` writer - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
pub struct LPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK_W<'a> {
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
    #[doc = "Bit 5 - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
    #[inline(always)]
    pub fn sfe(&self) -> SFE_R {
        SFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
    #[inline(always)]
    pub fn mfe(&self) -> MFE_R {
        MFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
    #[inline(always)]
    pub fn sfe(&mut self) -> SFE_W {
        SFE_W { w: self }
    }
    #[doc = "Bit 4 - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
    #[inline(always)]
    pub fn mfe(&mut self) -> MFE_W {
        MFE_W { w: self }
    }
    #[doc = "Bit 0 - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
    #[inline(always)]
    pub fn lpbk(&mut self) -> LPBK_W {
        LPBK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C master configuration This register configures the mode (master or slave) and sets the interface for test mode loopback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
