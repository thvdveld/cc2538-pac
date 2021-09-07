#[doc = "Register `IIE` reader"]
pub struct R(crate::R<IIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIE` writer"]
pub struct W(crate::W<IIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIE_SPEC>;
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
impl From<crate::W<IIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEP5IE` reader - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP5IE_R(crate::FieldReader<bool, bool>);
impl INEP5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEP5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEP5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEP5IE` writer - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP5IE_W<'a> {
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
#[doc = "Field `INEP4IE` reader - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP4IE_R(crate::FieldReader<bool, bool>);
impl INEP4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEP4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEP4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEP4IE` writer - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP4IE_W<'a> {
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
#[doc = "Field `INEP3IE` reader - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP3IE_R(crate::FieldReader<bool, bool>);
impl INEP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEP3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEP3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEP3IE` writer - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP3IE_W<'a> {
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
#[doc = "Field `INEP2IE` reader - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP2IE_R(crate::FieldReader<bool, bool>);
impl INEP2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEP2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEP2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEP2IE` writer - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP2IE_W<'a> {
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
#[doc = "Field `INEP1IE` reader - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP1IE_R(crate::FieldReader<bool, bool>);
impl INEP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEP1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEP1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEP1IE` writer - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub struct INEP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP1IE_W<'a> {
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
#[doc = "Field `EP0IE` reader - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
pub struct EP0IE_R(crate::FieldReader<bool, bool>);
impl EP0IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP0IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0IE` writer - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
pub struct EP0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0IE_W<'a> {
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
    #[doc = "Bit 5 - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep5ie(&self) -> INEP5IE_R {
        INEP5IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep4ie(&self) -> INEP4IE_R {
        INEP4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep3ie(&self) -> INEP3IE_R {
        INEP3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep2ie(&self) -> INEP2IE_R {
        INEP2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep1ie(&self) -> INEP1IE_R {
        INEP1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn ep0ie(&self) -> EP0IE_R {
        EP0IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep5ie(&mut self) -> INEP5IE_W {
        INEP5IE_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep4ie(&mut self) -> INEP4IE_W {
        INEP4IE_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep3ie(&mut self) -> INEP3IE_W {
        INEP3IE_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep2ie(&mut self) -> INEP2IE_W {
        INEP2IE_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep1ie(&mut self) -> INEP1IE_W {
        INEP1IE_W { w: self }
    }
    #[doc = "Bit 0 - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn ep0ie(&mut self) -> EP0IE_W {
        EP0IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iie](index.html) module"]
pub struct IIE_SPEC;
impl crate::RegisterSpec for IIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iie::R](R) reader structure"]
impl crate::Readable for IIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iie::W](W) writer structure"]
impl crate::Writable for IIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IIE to value 0"]
impl crate::Resettable for IIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
