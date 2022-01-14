#[doc = "Register `MDMCTRL1` reader"]
pub struct R(crate::R<MDMCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMCTRL1` writer"]
pub struct W(crate::W<MDMCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMCTRL1_SPEC>;
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
impl From<crate::W<MDMCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORR_THR_SFD` reader - Defines requirements for SFD detection: 0: The correlation value of one of the zero symbols of the preamble must be above the correlation threshold. 1: The correlation value of one zero symbol of the preamble and both symbols in the SFD must be above the correlation threshold."]
pub struct CORR_THR_SFD_R(crate::FieldReader<bool, bool>);
impl CORR_THR_SFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORR_THR_SFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORR_THR_SFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORR_THR_SFD` writer - Defines requirements for SFD detection: 0: The correlation value of one of the zero symbols of the preamble must be above the correlation threshold. 1: The correlation value of one zero symbol of the preamble and both symbols in the SFD must be above the correlation threshold."]
pub struct CORR_THR_SFD_W<'a> {
    w: &'a mut W,
}
impl<'a> CORR_THR_SFD_W<'a> {
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
#[doc = "Field `CORR_THR` reader - Demodulator correlator threshold value, required before SFD search. Threshold value adjusts how the receiver synchronizes to data from the radio. If the threshold is set too low, sync can more easily be found on noise. If set too high, the sensitivity is reduced, but sync is not likely to be found on noise. In combination with DEM_NUM_ZEROS, the system can be tuned so sensitivity is high with less sync found on noise."]
pub struct CORR_THR_R(crate::FieldReader<u8, u8>);
impl CORR_THR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORR_THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORR_THR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORR_THR` writer - Demodulator correlator threshold value, required before SFD search. Threshold value adjusts how the receiver synchronizes to data from the radio. If the threshold is set too low, sync can more easily be found on noise. If set too high, the sensitivity is reduced, but sync is not likely to be found on noise. In combination with DEM_NUM_ZEROS, the system can be tuned so sensitivity is high with less sync found on noise."]
pub struct CORR_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORR_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Defines requirements for SFD detection: 0: The correlation value of one of the zero symbols of the preamble must be above the correlation threshold. 1: The correlation value of one zero symbol of the preamble and both symbols in the SFD must be above the correlation threshold."]
    #[inline(always)]
    pub fn corr_thr_sfd(&self) -> CORR_THR_SFD_R {
        CORR_THR_SFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Demodulator correlator threshold value, required before SFD search. Threshold value adjusts how the receiver synchronizes to data from the radio. If the threshold is set too low, sync can more easily be found on noise. If set too high, the sensitivity is reduced, but sync is not likely to be found on noise. In combination with DEM_NUM_ZEROS, the system can be tuned so sensitivity is high with less sync found on noise."]
    #[inline(always)]
    pub fn corr_thr(&self) -> CORR_THR_R {
        CORR_THR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Defines requirements for SFD detection: 0: The correlation value of one of the zero symbols of the preamble must be above the correlation threshold. 1: The correlation value of one zero symbol of the preamble and both symbols in the SFD must be above the correlation threshold."]
    #[inline(always)]
    pub fn corr_thr_sfd(&mut self) -> CORR_THR_SFD_W {
        CORR_THR_SFD_W { w: self }
    }
    #[doc = "Bits 0:4 - Demodulator correlator threshold value, required before SFD search. Threshold value adjusts how the receiver synchronizes to data from the radio. If the threshold is set too low, sync can more easily be found on noise. If set too high, the sensitivity is reduced, but sync is not likely to be found on noise. In combination with DEM_NUM_ZEROS, the system can be tuned so sensitivity is high with less sync found on noise."]
    #[inline(always)]
    pub fn corr_thr(&mut self) -> CORR_THR_W {
        CORR_THR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdmctrl1](index.html) module"]
pub struct MDMCTRL1_SPEC;
impl crate::RegisterSpec for MDMCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdmctrl1::R](R) reader structure"]
impl crate::Readable for MDMCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdmctrl1::W](W) writer structure"]
impl crate::Writable for MDMCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMCTRL1 to value 0"]
impl crate::Resettable for MDMCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
