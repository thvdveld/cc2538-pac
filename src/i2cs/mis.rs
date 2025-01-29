#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `DATAMIS` reader - Data masked interrupt status 1: An unmasked data received or data requested interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
pub type DatamisR = crate::BitReader;
#[doc = "Field `STARTMIS` reader - Start condition masked interrupt status 1: An unmasked START condition interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
pub type StartmisR = crate::BitReader;
#[doc = "Field `STOPMIS` reader - Stop condition masked interrupt status 1: An unmasked STOP condition interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
pub type StopmisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data masked interrupt status 1: An unmasked data received or data requested interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn datamis(&self) -> DatamisR {
        DatamisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start condition masked interrupt status 1: An unmasked START condition interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn startmis(&self) -> StartmisR {
        StartmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop condition masked interrupt status 1: An unmasked STOP condition interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn stopmis(&self) -> StopmisR {
        StopmisR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "I2C slave masked interrupt status This register specifies whether an interrupt was signaled.\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
