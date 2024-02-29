#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RIS` reader - Reflects the status of interrupts trigger conditions detected on pins (raw, before masking) Bits set: Requirements met by corresponding pins Bits clear: Requirements not met"]
pub type RisR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reflects the status of interrupts trigger conditions detected on pins (raw, before masking) Bits set: Requirements met by corresponding pins Bits clear: Requirements not met"]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
