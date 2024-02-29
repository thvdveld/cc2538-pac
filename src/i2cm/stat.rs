#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `BUSY` reader - I2C busy 0: The controller is idle. 1: The controller is busy. When the BUSY bit is set, the other status bits are not valid."]
pub type BusyR = crate::BitReader;
#[doc = "Field `ERROR` reader - Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ADRACK` reader - Acknowledge address 0: The transmited address was acknowledged. 1: The transmited address was not acknowledged."]
pub type AdrackR = crate::BitReader;
#[doc = "Field `DATACK` reader - Acknowledge data 0: The transmited data was acknowledged. 1: The transmited data was not acknowledged."]
pub type DatackR = crate::BitReader;
#[doc = "Field `ARBLST` reader - Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
pub type ArblstR = crate::BitReader;
#[doc = "Field `IDLE` reader - I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
pub type IdleR = crate::BitReader;
#[doc = "Field `BUSBSY` reader - Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the START and STOP conditions."]
pub type BusbsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C busy 0: The controller is idle. 1: The controller is busy. When the BUSY bit is set, the other status bits are not valid."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge address 0: The transmited address was acknowledged. 1: The transmited address was not acknowledged."]
    #[inline(always)]
    pub fn adrack(&self) -> AdrackR {
        AdrackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge data 0: The transmited data was acknowledged. 1: The transmited data was not acknowledged."]
    #[inline(always)]
    pub fn datack(&self) -> DatackR {
        DatackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    pub fn arblst(&self) -> ArblstR {
        ArblstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the START and STOP conditions."]
    #[inline(always)]
    pub fn busbsy(&self) -> BusbsyR {
        BusbsyR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
