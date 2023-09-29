#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Field `MIS` reader - Masked interrupt status 1: An unmasked master interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the IC bit in the I2CMICR register."]
pub type MIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked interrupt status 1: An unmasked master interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the IC bit in the I2CMICR register."]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "I2C master masked interrupt status This register specifies whether an interrupt was signaled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
