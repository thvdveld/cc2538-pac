#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `MIS` reader - Masked value of interrupt due to corresponding pin Bits clear: GPIO line interrupt not active Bits set: GPIO line asserting interrupt"]
pub type MisR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Masked value of interrupt due to corresponding pin Bits clear: GPIO line interrupt not active Bits set: GPIO line asserting interrupt"]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking.\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
