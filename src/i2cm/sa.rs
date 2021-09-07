#[doc = "Register `SA` reader"]
pub struct R(crate::R<SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SA` writer"]
pub struct W(crate::W<SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SA_SPEC>;
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
impl From<crate::W<SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SA` reader - I2C slave address"]
pub struct SA_R(crate::FieldReader<u8, u8>);
impl SA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA` writer - I2C slave address"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
#[doc = "Field `RS` reader - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
pub struct RS_R(crate::FieldReader<bool, bool>);
impl RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS` writer - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
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
    #[doc = "Bits 1:7 - I2C slave address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - I2C slave address"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Bit 0 - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C master slave address This register consists of eight bits, seven address bits (A6-A0), and a receive and send bit, which determines if the next operation is a receive (high) or transmit (low).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa](index.html) module"]
pub struct SA_SPEC;
impl crate::RegisterSpec for SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa::R](R) reader structure"]
impl crate::Readable for SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa::W](W) writer structure"]
impl crate::Writable for SA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SA to value 0"]
impl crate::Resettable for SA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
