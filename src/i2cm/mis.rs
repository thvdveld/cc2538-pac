#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `MIS` reader - Masked interrupt status 1: An unmasked master interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the IC bit in the I2CMICR register."]
pub type MisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked interrupt status 1: An unmasked master interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the IC bit in the I2CMICR register."]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new((self.bits & 1) != 0)
    }
}
#[doc = "I2C master masked interrupt status This register specifies whether an interrupt was signaled.\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
