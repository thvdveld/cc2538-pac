#[doc = "Register `FSCAL2` reader"]
pub struct R(crate::R<FSCAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSCAL2` writer"]
pub struct W(crate::W<FSCAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSCAL2_SPEC>;
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
impl From<crate::W<FSCAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSCAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCO_CAPARR_OE` reader - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
pub struct VCO_CAPARR_OE_R(crate::FieldReader<bool, bool>);
impl VCO_CAPARR_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCO_CAPARR_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_CAPARR_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_CAPARR_OE` writer - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
pub struct VCO_CAPARR_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CAPARR_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `VCO_CAPARR` reader - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
pub struct VCO_CAPARR_R(crate::FieldReader<u8, u8>);
impl VCO_CAPARR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_CAPARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_CAPARR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_CAPARR` writer - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
pub struct VCO_CAPARR_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CAPARR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
    #[inline(always)]
    pub fn vco_caparr_oe(&self) -> VCO_CAPARR_OE_R {
        VCO_CAPARR_OE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
    #[inline(always)]
    pub fn vco_caparr(&self) -> VCO_CAPARR_R {
        VCO_CAPARR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
    #[inline(always)]
    pub fn vco_caparr_oe(&mut self) -> VCO_CAPARR_OE_W {
        VCO_CAPARR_OE_W { w: self }
    }
    #[doc = "Bits 0:5 - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
    #[inline(always)]
    pub fn vco_caparr(&mut self) -> VCO_CAPARR_W {
        VCO_CAPARR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscal2](index.html) module"]
pub struct FSCAL2_SPEC;
impl crate::RegisterSpec for FSCAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fscal2::R](R) reader structure"]
impl crate::Readable for FSCAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fscal2::W](W) writer structure"]
impl crate::Writable for FSCAL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSCAL2 to value 0"]
impl crate::Resettable for FSCAL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
