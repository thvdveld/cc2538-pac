#[doc = "Register `RSSISTAT` reader"]
pub type R = crate::R<RSSISTAT_SPEC>;
#[doc = "Field `RSSI_VALID` reader - RSSI value is valid. Occurs eight symbol periods after entering RX."]
pub type RSSI_VALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RSSI value is valid. Occurs eight symbol periods after entering RX."]
    #[inline(always)]
    pub fn rssi_valid(&self) -> RSSI_VALID_R {
        RSSI_VALID_R::new((self.bits & 1) != 0)
    }
}
#[doc = "RSSI valid status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rssistat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSSISTAT_SPEC;
impl crate::RegisterSpec for RSSISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rssistat::R`](R) reader structure"]
impl crate::Readable for RSSISTAT_SPEC {}
#[doc = "`reset()` method sets RSSISTAT to value 0"]
impl crate::Resettable for RSSISTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
