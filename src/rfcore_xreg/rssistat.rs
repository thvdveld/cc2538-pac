#[doc = "Register `RSSISTAT` reader"]
pub type R = crate::R<RssistatSpec>;
#[doc = "Field `RSSI_VALID` reader - RSSI value is valid. Occurs eight symbol periods after entering RX."]
pub type RssiValidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RSSI value is valid. Occurs eight symbol periods after entering RX."]
    #[inline(always)]
    pub fn rssi_valid(&self) -> RssiValidR {
        RssiValidR::new((self.bits & 1) != 0)
    }
}
#[doc = "RSSI valid status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rssistat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssistatSpec;
impl crate::RegisterSpec for RssistatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rssistat::R`](R) reader structure"]
impl crate::Readable for RssistatSpec {}
#[doc = "`reset()` method sets RSSISTAT to value 0"]
impl crate::Resettable for RssistatSpec {
    const RESET_VALUE: u32 = 0;
}
