#[doc = "Register `OIE` reader"]
pub struct R(crate::R<OIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OIE` writer"]
pub struct W(crate::W<OIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OIE_SPEC>;
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
impl From<crate::W<OIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTEP5IE` reader - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP5IE_R(crate::FieldReader<bool, bool>);
impl OUTEP5IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP5IE` writer - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP5IE_W<'a> {
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
#[doc = "Field `OUTEP4IE` reader - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP4IE_R(crate::FieldReader<bool, bool>);
impl OUTEP4IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP4IE` writer - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP4IE_W<'a> {
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
#[doc = "Field `OUTEP3IE` reader - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP3IE_R(crate::FieldReader<bool, bool>);
impl OUTEP3IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP3IE` writer - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP3IE_W<'a> {
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
#[doc = "Field `OUTEP2IE` reader - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP2IE_R(crate::FieldReader<bool, bool>);
impl OUTEP2IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP2IE` writer - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP2IE_W<'a> {
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
#[doc = "Field `OUTEP1IE` reader - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP1IE_R(crate::FieldReader<bool, bool>);
impl OUTEP1IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP1IE` writer - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub struct OUTEP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP1IE_W<'a> {
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
impl R {
    #[doc = "Bit 5 - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&self) -> OUTEP5IE_R {
        OUTEP5IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&self) -> OUTEP4IE_R {
        OUTEP4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&self) -> OUTEP3IE_R {
        OUTEP3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&self) -> OUTEP2IE_R {
        OUTEP2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&self) -> OUTEP1IE_R {
        OUTEP1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&mut self) -> OUTEP5IE_W {
        OUTEP5IE_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&mut self) -> OUTEP4IE_W {
        OUTEP4IE_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&mut self) -> OUTEP3IE_W {
        OUTEP3IE_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&mut self) -> OUTEP2IE_W {
        OUTEP2IE_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&mut self) -> OUTEP1IE_W {
        OUTEP1IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable mask for OUT endpoints 1-5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oie](index.html) module"]
pub struct OIE_SPEC;
impl crate::RegisterSpec for OIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oie::R](R) reader structure"]
impl crate::Readable for OIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oie::W](W) writer structure"]
impl crate::Writable for OIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OIE to value 0"]
impl crate::Resettable for OIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
