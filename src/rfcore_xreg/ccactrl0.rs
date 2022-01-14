#[doc = "Register `CCACTRL0` reader"]
pub struct R(crate::R<CCACTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCACTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCACTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCACTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCACTRL0` writer"]
pub struct W(crate::W<CCACTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCACTRL0_SPEC>;
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
impl From<crate::W<CCACTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCACTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCA_THR` reader - Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
pub struct CCA_THR_R(crate::FieldReader<u8, u8>);
impl CCA_THR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCA_THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCA_THR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCA_THR` writer - Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
pub struct CCA_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCA_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
    #[inline(always)]
    pub fn cca_thr(&self) -> CCA_THR_R {
        CCA_THR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
    #[inline(always)]
    pub fn cca_thr(&mut self) -> CCA_THR_W {
        CCA_THR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCA threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccactrl0](index.html) module"]
pub struct CCACTRL0_SPEC;
impl crate::RegisterSpec for CCACTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccactrl0::R](R) reader structure"]
impl crate::Readable for CCACTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccactrl0::W](W) writer structure"]
impl crate::Writable for CCACTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCACTRL0 to value 0"]
impl crate::Resettable for CCACTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
