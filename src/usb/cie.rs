#[doc = "Register `CIE` reader"]
pub struct R(crate::R<CIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIE` writer"]
pub struct W(crate::W<CIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIE_SPEC>;
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
impl From<crate::W<CIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIE` reader - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub struct SOFIE_R(crate::FieldReader<bool, bool>);
impl SOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFIE` writer - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
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
#[doc = "Field `RSTIE` reader - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub struct RSTIE_R(crate::FieldReader<bool, bool>);
impl RSTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTIE` writer - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub struct RSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIE_W<'a> {
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
#[doc = "Field `RESUMEIE` reader - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub struct RESUMEIE_R(crate::FieldReader<bool, bool>);
impl RESUMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUMEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUMEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUMEIE` writer - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub struct RESUMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIE_W<'a> {
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
#[doc = "Field `SUSPENDIE` reader - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub struct SUSPENDIE_R(crate::FieldReader<bool, bool>);
impl SUSPENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPENDIE` writer - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub struct SUSPENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPENDIE_W<'a> {
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
    #[doc = "Bit 3 - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn resumeie(&self) -> RESUMEIE_R {
        RESUMEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn suspendie(&self) -> SUSPENDIE_R {
        SUSPENDIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    #[doc = "Bit 2 - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W {
        RSTIE_W { w: self }
    }
    #[doc = "Bit 1 - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn resumeie(&mut self) -> RESUMEIE_W {
        RESUMEIE_W { w: self }
    }
    #[doc = "Bit 0 - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn suspendie(&mut self) -> SUSPENDIE_W {
        SUSPENDIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common USB interrupt enable mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cie](index.html) module"]
pub struct CIE_SPEC;
impl crate::RegisterSpec for CIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cie::R](R) reader structure"]
impl crate::Readable for CIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cie::W](W) writer structure"]
impl crate::Writable for CIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIE to value 0"]
impl crate::Resettable for CIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
