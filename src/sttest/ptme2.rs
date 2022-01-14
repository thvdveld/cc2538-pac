#[doc = "Register `PTME2` reader"]
pub struct R(crate::R<PTME2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTME2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTME2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTME2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTME2` writer"]
pub struct W(crate::W<PTME2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTME2_SPEC>;
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
impl From<crate::W<PTME2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTME2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T3TME` reader - Timer3 test mode enable"]
pub struct T3TME_R(crate::FieldReader<bool, bool>);
impl T3TME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T3TME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T3TME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T3TME` writer - Timer3 test mode enable"]
pub struct T3TME_W<'a> {
    w: &'a mut W,
}
impl<'a> T3TME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `MTTME` reader - MacTimer test mode enable"]
pub struct MTTME_R(crate::FieldReader<bool, bool>);
impl MTTME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTTME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTTME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTTME` writer - MacTimer test mode enable"]
pub struct MTTME_W<'a> {
    w: &'a mut W,
}
impl<'a> MTTME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `T1TME` reader - Timer1 test mode enable"]
pub struct T1TME_R(crate::FieldReader<bool, bool>);
impl T1TME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T1TME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1TME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1TME` writer - Timer1 test mode enable"]
pub struct T1TME_W<'a> {
    w: &'a mut W,
}
impl<'a> T1TME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `T0TME` reader - Timer0 test mode enable"]
pub struct T0TME_R(crate::FieldReader<bool, bool>);
impl T0TME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0TME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0TME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0TME` writer - Timer0 test mode enable"]
pub struct T0TME_W<'a> {
    w: &'a mut W,
}
impl<'a> T0TME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `I2C0TME` reader - I2C 0 test mode enable"]
pub struct I2C0TME_R(crate::FieldReader<bool, bool>);
impl I2C0TME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0TME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C0TME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0TME` writer - I2C 0 test mode enable"]
pub struct I2C0TME_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0TME_W<'a> {
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
    #[doc = "Bit 19 - Timer3 test mode enable"]
    #[inline(always)]
    pub fn t3tme(&self) -> T3TME_R {
        T3TME_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MacTimer test mode enable"]
    #[inline(always)]
    pub fn mttme(&self) -> MTTME_R {
        MTTME_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer1 test mode enable"]
    #[inline(always)]
    pub fn t1tme(&self) -> T1TME_R {
        T1TME_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer0 test mode enable"]
    #[inline(always)]
    pub fn t0tme(&self) -> T0TME_R {
        T0TME_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C 0 test mode enable"]
    #[inline(always)]
    pub fn i2c0tme(&self) -> I2C0TME_R {
        I2C0TME_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - Timer3 test mode enable"]
    #[inline(always)]
    pub fn t3tme(&mut self) -> T3TME_W {
        T3TME_W { w: self }
    }
    #[doc = "Bit 18 - MacTimer test mode enable"]
    #[inline(always)]
    pub fn mttme(&mut self) -> MTTME_W {
        MTTME_W { w: self }
    }
    #[doc = "Bit 17 - Timer1 test mode enable"]
    #[inline(always)]
    pub fn t1tme(&mut self) -> T1TME_W {
        T1TME_W { w: self }
    }
    #[doc = "Bit 16 - Timer0 test mode enable"]
    #[inline(always)]
    pub fn t0tme(&mut self) -> T0TME_W {
        T0TME_W { w: self }
    }
    #[doc = "Bit 0 - I2C 0 test mode enable"]
    #[inline(always)]
    pub fn i2c0tme(&mut self) -> I2C0TME_W {
        I2C0TME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral test mode enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptme2](index.html) module"]
pub struct PTME2_SPEC;
impl crate::RegisterSpec for PTME2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptme2::R](R) reader structure"]
impl crate::Readable for PTME2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptme2::W](W) writer structure"]
impl crate::Writable for PTME2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTME2 to value 0"]
impl crate::Resettable for PTME2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
