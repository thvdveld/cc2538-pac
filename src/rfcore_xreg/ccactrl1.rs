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
pub type CCA_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCA_MODE` writer - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
pub type CCA_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCACTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CCA_HYST` reader - Sets the level of CCA hysteresis. Unsigned values given in dB"]
pub type CCA_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCA_HYST` writer - Sets the level of CCA hysteresis. Unsigned values given in dB"]
pub type CCA_HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCACTRL1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 3:4 - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&self) -> CCA_MODE_R {
        CCA_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 0:2 - Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&self) -> CCA_HYST_R {
        CCA_HYST_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - 00: CCA always set to 1 01: CCA = 1 when RSSI < CCA_THR - CCA_HYST; CCA = 0 when RSSI >= CCA_THR 10: CCA = 1 when not receiving a frame, else CCA = 0 11: CCA = 1 when RSSI < CCA_THR - CCA_HYST and not receiving a frame; CCA = 0 when RSSI >= CCA_THR or when receiving a frame"]
    #[inline(always)]
    pub fn cca_mode(&mut self) -> CCA_MODE_W<3> {
        CCA_MODE_W::new(self)
    }
    #[doc = "Bits 0:2 - Sets the level of CCA hysteresis. Unsigned values given in dB"]
    #[inline(always)]
    pub fn cca_hyst(&mut self) -> CCA_HYST_W<0> {
        CCA_HYST_W::new(self)
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
