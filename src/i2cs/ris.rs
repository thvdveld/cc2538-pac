#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `DATARIS` reader - Data raw interrupt status 1: A data received or data requested interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
pub type DatarisR = crate::BitReader;
#[doc = "Field `STARTRIS` reader - Start condition raw interrupt status 1: A START condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
pub type StartrisR = crate::BitReader;
#[doc = "Field `STOPRIS` reader - Stop condition raw interrupt status 1: A STOP condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
pub type StoprisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data raw interrupt status 1: A data received or data requested interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn dataris(&self) -> DatarisR {
        DatarisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start condition raw interrupt status 1: A START condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn startris(&self) -> StartrisR {
        StartrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop condition raw interrupt status 1: A STOP condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn stopris(&self) -> StoprisR {
        StoprisR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "I2C slave raw interrupt status This register specifies whether an interrupt is pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
