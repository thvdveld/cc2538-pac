#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FBR` reader - First byte received 1: The first byte following the slave's own address has been received. 0: The first byte has not been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the I2CSDR register. Note: This bit is not used for slave transmit operations."]
pub type FBR_R = crate::BitReader<bool>;
#[doc = "Field `TREQ` reader - Transmit request 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the I2CSDR register. 0: No outstanding transmit request."]
pub type TREQ_R = crate::BitReader<bool>;
#[doc = "Field `RREQ` reader - Receive request 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the I2CSDR register. 0: No outstanding receive data"]
pub type RREQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - First byte received 1: The first byte following the slave's own address has been received. 0: The first byte has not been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the I2CSDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    pub fn fbr(&self) -> FBR_R {
        FBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit request 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the I2CSDR register. 0: No outstanding transmit request."]
    #[inline(always)]
    pub fn treq(&self) -> TREQ_R {
        TREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Receive request 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the I2CSDR register. 0: No outstanding receive data"]
    #[inline(always)]
    pub fn rreq(&self) -> RREQ_R {
        RREQ_R::new((self.bits & 1) != 0)
    }
}
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
