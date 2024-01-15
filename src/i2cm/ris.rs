#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Field `RIS` reader - Raw interrupt status 1: A master interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the IC bit in the I2CMICR register."]
pub type RIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw interrupt status 1: A master interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the IC bit in the I2CMICR register."]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "I2C master raw interrupt status This register specifies whether an interrupt is pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
