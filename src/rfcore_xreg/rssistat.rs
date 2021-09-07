#[doc = "Register `RSSISTAT` reader"]
pub struct R(crate::R<RSSISTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSSISTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSSISTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSSISTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSSI_VALID` reader - RSSI value is valid. Occurs eight symbol periods after entering RX."]
pub struct RSSI_VALID_R(crate::FieldReader<bool, bool>);
impl RSSI_VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSSI_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSI_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RSSI value is valid. Occurs eight symbol periods after entering RX."]
    #[inline(always)]
    pub fn rssi_valid(&self) -> RSSI_VALID_R {
        RSSI_VALID_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RSSI valid status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rssistat](index.html) module"]
pub struct RSSISTAT_SPEC;
impl crate::RegisterSpec for RSSISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rssistat::R](R) reader structure"]
impl crate::Readable for RSSISTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSSISTAT to value 0"]
impl crate::Resettable for RSSISTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
