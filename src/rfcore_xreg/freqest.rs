#[doc = "Register `FREQEST` reader"]
pub struct R(crate::R<FREQEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FREQEST` reader - Signed 2's-complement value. Contains an estimate of the frequency offset between carrier and the receiver LO. The offset frequency is FREQEST x 7800 Hz. DEM_AVG_MODE controls when this estimate is updated. If DEM_AVG_MODE = 0, it is updated until sync is found. Then the frequency offset estimate is frozen until the end of the received frame. If DEM_AVG_MODE = 1, it is updated as long as the demodulator is enabled. To calculate the correct value, one must use an offset (FREQEST_offset), which can be found in the device data sheet. Real FREQEST value = FREQEST - FREQEST_offset."]
pub type FREQEST_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Signed 2's-complement value. Contains an estimate of the frequency offset between carrier and the receiver LO. The offset frequency is FREQEST x 7800 Hz. DEM_AVG_MODE controls when this estimate is updated. If DEM_AVG_MODE = 0, it is updated until sync is found. Then the frequency offset estimate is frozen until the end of the received frame. If DEM_AVG_MODE = 1, it is updated as long as the demodulator is enabled. To calculate the correct value, one must use an offset (FREQEST_offset), which can be found in the device data sheet. Real FREQEST value = FREQEST - FREQEST_offset."]
    #[inline(always)]
    pub fn freqest(&self) -> FREQEST_R {
        FREQEST_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Estimated RF frequency offset\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqest](index.html) module"]
pub struct FREQEST_SPEC;
impl crate::RegisterSpec for FREQEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqest::R](R) reader structure"]
impl crate::Readable for FREQEST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FREQEST to value 0"]
impl crate::Resettable for FREQEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
