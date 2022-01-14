#[doc = "Register `AGCCTRL2` reader"]
pub struct R(crate::R<AGCCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGCCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGCCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGCCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGCCTRL2` writer"]
pub struct W(crate::W<AGCCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGCCTRL2_SPEC>;
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
impl From<crate::W<AGCCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGCCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNA1_CURRENT` reader - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
pub struct LNA1_CURRENT_R(crate::FieldReader<u8, u8>);
impl LNA1_CURRENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA1_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA1_CURRENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA1_CURRENT` writer - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
pub struct LNA1_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA1_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `LNA2_CURRENT` reader - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
pub struct LNA2_CURRENT_R(crate::FieldReader<u8, u8>);
impl LNA2_CURRENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA2_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA2_CURRENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA2_CURRENT` writer - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
pub struct LNA2_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA2_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `LNA3_CURRENT` reader - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
pub struct LNA3_CURRENT_R(crate::FieldReader<u8, u8>);
impl LNA3_CURRENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA3_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA3_CURRENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA3_CURRENT` writer - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
pub struct LNA3_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA3_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `LNA_CURRENT_OE` reader - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
pub struct LNA_CURRENT_OE_R(crate::FieldReader<bool, bool>);
impl LNA_CURRENT_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LNA_CURRENT_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_CURRENT_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_CURRENT_OE` writer - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
pub struct LNA_CURRENT_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CURRENT_OE_W<'a> {
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
    #[doc = "Bits 6:7 - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    pub fn lna1_current(&self) -> LNA1_CURRENT_R {
        LNA1_CURRENT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    pub fn lna2_current(&self) -> LNA2_CURRENT_R {
        LNA2_CURRENT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 1:2 - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    pub fn lna3_current(&self) -> LNA3_CURRENT_R {
        LNA3_CURRENT_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
    #[inline(always)]
    pub fn lna_current_oe(&self) -> LNA_CURRENT_OE_R {
        LNA_CURRENT_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    pub fn lna1_current(&mut self) -> LNA1_CURRENT_W {
        LNA1_CURRENT_W { w: self }
    }
    #[doc = "Bits 3:5 - Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    pub fn lna2_current(&mut self) -> LNA2_CURRENT_W {
        LNA2_CURRENT_W { w: self }
    }
    #[doc = "Bits 1:2 - Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    pub fn lna3_current(&mut self) -> LNA3_CURRENT_W {
        LNA3_CURRENT_W { w: self }
    }
    #[doc = "Bit 0 - Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
    #[inline(always)]
    pub fn lna_current_oe(&mut self) -> LNA_CURRENT_OE_W {
        LNA_CURRENT_OE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGC gain override\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agcctrl2](index.html) module"]
pub struct AGCCTRL2_SPEC;
impl crate::RegisterSpec for AGCCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [agcctrl2::R](R) reader structure"]
impl crate::Readable for AGCCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agcctrl2::W](W) writer structure"]
impl crate::Writable for AGCCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AGCCTRL2 to value 0"]
impl crate::Resettable for AGCCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
