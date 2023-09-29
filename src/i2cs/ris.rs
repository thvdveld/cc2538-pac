#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Field `DATARIS` reader - Data raw interrupt status 1: A data received or data requested interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
pub type DATARIS_R = crate::BitReader;
#[doc = "Field `STARTRIS` reader - Start condition raw interrupt status 1: A START condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
pub type STARTRIS_R = crate::BitReader;
#[doc = "Field `STOPRIS` reader - Stop condition raw interrupt status 1: A STOP condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
pub type STOPRIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data raw interrupt status 1: A data received or data requested interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn dataris(&self) -> DATARIS_R {
        DATARIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start condition raw interrupt status 1: A START condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn startris(&self) -> STARTRIS_R {
        STARTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop condition raw interrupt status 1: A STOP condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn stopris(&self) -> STOPRIS_R {
        STOPRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "I2C slave raw interrupt status This register specifies whether an interrupt is pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
