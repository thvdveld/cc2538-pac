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
#[doc = "Field `BUSBSY` reader - Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the START and STOP conditions."]
pub struct BUSBSY_R(crate::FieldReader<bool, bool>);
impl BUSBSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSBSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSBSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE` reader - I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
pub struct IDLE_R(crate::FieldReader<bool, bool>);
impl IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBLST` reader - Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
pub struct ARBLST_R(crate::FieldReader<bool, bool>);
impl ARBLST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBLST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBLST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATACK` reader - Acknowledge data 0: The transmited data was acknowledged. 1: The transmited data was not acknowledged."]
pub struct DATACK_R(crate::FieldReader<bool, bool>);
impl DATACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADRACK` reader - Acknowledge address 0: The transmited address was acknowledged. 1: The transmited address was not acknowledged."]
pub struct ADRACK_R(crate::FieldReader<bool, bool>);
impl ADRACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADRACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADRACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` reader - Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
pub struct ERROR_R(crate::FieldReader<bool, bool>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - I2C busy 0: The controller is idle. 1: The controller is busy. When the BUSY bit is set, the other status bits are not valid."]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the START and STOP conditions."]
    #[inline(always)]
    pub fn busbsy(&self) -> BUSBSY_R {
        BUSBSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledge data 0: The transmited data was acknowledged. 1: The transmited data was not acknowledged."]
    #[inline(always)]
    pub fn datack(&self) -> DATACK_R {
        DATACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Acknowledge address 0: The transmited address was acknowledged. 1: The transmited address was not acknowledged."]
    #[inline(always)]
    pub fn adrack(&self) -> ADRACK_R {
        ADRACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C busy 0: The controller is idle. 1: The controller is busy. When the BUSY bit is set, the other status bits are not valid."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
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
