#[doc = "Register `RSSI` reader"]
pub struct R(crate::R<RSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSSI_VAL` reader - RSSI estimate on a logarithmic scale, signed number on 2's complement Unit is 1 dB, offset is 73dB. The RSSI value is averaged over eight symbol periods. The RSSI_VALID status bit should be checked before reading RSSI_VAL for the first time. The reset value of -128 also indicates that the RSSI value is invalid."]
pub type RSSI_VAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RSSI estimate on a logarithmic scale, signed number on 2's complement Unit is 1 dB, offset is 73dB. The RSSI value is averaged over eight symbol periods. The RSSI_VALID status bit should be checked before reading RSSI_VAL for the first time. The reset value of -128 also indicates that the RSSI value is invalid."]
    #[inline(always)]
    pub fn rssi_val(&self) -> RSSI_VAL_R {
        RSSI_VAL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RSSI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rssi](index.html) module"]
pub struct RSSI_SPEC;
impl crate::RegisterSpec for RSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rssi::R](R) reader structure"]
impl crate::Readable for RSSI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSSI to value 0"]
impl crate::Resettable for RSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
