#[doc = "Register `FREQEST` reader"]
pub type R = crate::R<FREQEST_SPEC>;
#[doc = "Field `FREQEST` reader - Signed 2's-complement value. Contains an estimate of the frequency offset between carrier and the receiver LO. The offset frequency is FREQEST x 7800 Hz. DEM_AVG_MODE controls when this estimate is updated. If DEM_AVG_MODE = 0, it is updated until sync is found. Then the frequency offset estimate is frozen until the end of the received frame. If DEM_AVG_MODE = 1, it is updated as long as the demodulator is enabled. To calculate the correct value, one must use an offset (FREQEST_offset), which can be found in the device data sheet. Real FREQEST value = FREQEST - FREQEST_offset."]
pub type FREQEST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Signed 2's-complement value. Contains an estimate of the frequency offset between carrier and the receiver LO. The offset frequency is FREQEST x 7800 Hz. DEM_AVG_MODE controls when this estimate is updated. If DEM_AVG_MODE = 0, it is updated until sync is found. Then the frequency offset estimate is frozen until the end of the received frame. If DEM_AVG_MODE = 1, it is updated as long as the demodulator is enabled. To calculate the correct value, one must use an offset (FREQEST_offset), which can be found in the device data sheet. Real FREQEST value = FREQEST - FREQEST_offset."]
    #[inline(always)]
    pub fn freqest(&self) -> FREQEST_R {
        FREQEST_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Estimated RF frequency offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqest::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREQEST_SPEC;
impl crate::RegisterSpec for FREQEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqest::R`](R) reader structure"]
impl crate::Readable for FREQEST_SPEC {}
#[doc = "`reset()` method sets FREQEST to value 0"]
impl crate::Resettable for FREQEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
