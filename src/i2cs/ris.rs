#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STOPRIS` reader - Stop condition raw interrupt status 1: A STOP condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
pub type STOPRIS_R = crate::BitReader<bool>;
#[doc = "Field `STARTRIS` reader - Start condition raw interrupt status 1: A START condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
pub type STARTRIS_R = crate::BitReader<bool>;
#[doc = "Field `DATARIS` reader - Data raw interrupt status 1: A data received or data requested interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
pub type DATARIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - Stop condition raw interrupt status 1: A STOP condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn stopris(&self) -> STOPRIS_R {
        STOPRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Start condition raw interrupt status 1: A START condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn startris(&self) -> STARTRIS_R {
        STARTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Data raw interrupt status 1: A data received or data requested interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn dataris(&self) -> DATARIS_R {
        DATARIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "I2C slave raw interrupt status This register specifies whether an interrupt is pending.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
