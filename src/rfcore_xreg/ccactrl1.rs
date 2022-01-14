#[doc = "Register `CCACTRL1` reader"]
pub struct R(crate::R<CCACTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCACTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCACTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCACTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCACTRL1` writer"]
pub struct W(crate::W<CCACTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCACTRL1_SPEC>;
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
impl From<crate::W<CCACTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCACTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCA_MODE` reader - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
pub struct CCA_MODE_R(crate::FieldReader<u8, u8>);
impl CCA_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCA_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCA_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCA_MODE` writer - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
pub struct CCA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCA_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `CCA_HYST` reader - Sets the level of CCA hysteresis. Unsigned values given in dB"]
pub struct CCA_HYST_R(crate::FieldReader<u8, u8>);
impl CCA_HYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCA_HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCA_HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCA_HYST` writer - Sets the level of CCA hysteresis. Unsigned values given in dB"]
pub struct CCA_HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> CCA_HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&self) -> CCA_MODE_R {
        CCA_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2 - Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&self) -> CCA_HYST_R {
        CCA_HYST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&mut self) -> CCA_MODE_W {
        CCA_MODE_W { w: self }
    }
    #[doc = "Bits 0:2 - Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&mut self) -> CCA_HYST_W {
        CCA_HYST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Other CCA Options\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccactrl1](index.html) module"]
pub struct CCACTRL1_SPEC;
impl crate::RegisterSpec for CCACTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccactrl1::R](R) reader structure"]
impl crate::Readable for CCACTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccactrl1::W](W) writer structure"]
impl crate::Writable for CCACTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCACTRL1 to value 0"]
impl crate::Resettable for CCACTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
