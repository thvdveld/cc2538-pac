#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `RUN` writer - I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data. When the BUSY bit is set, the other status bits are not valid."]
pub type RUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START` writer - Generate START 0: The controller does not generate the START condition. 1: The controller generates the START condition."]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` writer - Generate STOP 0: The controller does not generate the STOP condition. 1: The controller generates the STOP condition."]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACK` writer - Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master."]
pub type ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data. When the BUSY bit is set, the other status bits are not valid."]
    #[inline(always)]
    #[must_use]
    pub fn run(&mut self) -> RUN_W<CTRL_SPEC, 0> {
        RUN_W::new(self)
    }
    #[doc = "Bit 1 - Generate START 0: The controller does not generate the START condition. 1: The controller generates the START condition."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTRL_SPEC, 1> {
        START_W::new(self)
    }
    #[doc = "Bit 2 - Generate STOP 0: The controller does not generate the STOP condition. 1: The controller generates the STOP condition."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CTRL_SPEC, 2> {
        STOP_W::new(self)
    }
    #[doc = "Bit 3 - Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<CTRL_SPEC, 3> {
        ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
