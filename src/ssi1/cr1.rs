#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `LBM` reader - SSI loop-back mode (R/W) Reset value: 0x0 0: Normal serial port operation is enabled. 1: The output of the transmit serial shifter is connected to the input of the receive serial shift register internally."]
pub type LBM_R = crate::BitReader;
#[doc = "Field `LBM` writer - SSI loop-back mode (R/W) Reset value: 0x0 0: Normal serial port operation is enabled. 1: The output of the transmit serial shifter is connected to the input of the receive serial shift register internally."]
pub type LBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE` reader - SSI synchronous serial port enable (R/W) Reset value: 0x0 0: SSI operation is disabled. 1: SSI operation is enabled."]
pub type SSE_R = crate::BitReader;
#[doc = "Field `SSE` writer - SSI synchronous serial port enable (R/W) Reset value: 0x0 0: SSI operation is disabled. 1: SSI operation is enabled."]
pub type SSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - SSI master and slave select (R/W) Reset value: 0x0 This bit can be modified only when the SSI is disabled (SSE = 0). 0: Device configured as a master (default) 1: Device configured as a slave"]
pub type MS_R = crate::BitReader;
#[doc = "Field `MS` writer - SSI master and slave select (R/W) Reset value: 0x0 This bit can be modified only when the SSI is disabled (SSE = 0). 0: Device configured as a master (default) 1: Device configured as a slave"]
pub type MS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOD` reader - SSI slave mode output disable (R/W) Reset value: 0x0 This bit is relevant only in the slave mode (MS = 1). In multiple-slave systems, it is possible for the SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto the serial output line. In such systems, the RXD lines from multiple slaves could be tied together. To operate in such a system, the SOD bit can be set if the SSI slave is not suppose to drive the SSITXD line. 0: SSI can drive SSITXD in slave output mode 1: SSI must not drive the SSITXD output in slave mode"]
pub type SOD_R = crate::BitReader;
#[doc = "Field `SOD` writer - SSI slave mode output disable (R/W) Reset value: 0x0 This bit is relevant only in the slave mode (MS = 1). In multiple-slave systems, it is possible for the SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto the serial output line. In such systems, the RXD lines from multiple slaves could be tied together. To operate in such a system, the SOD bit can be set if the SSI slave is not suppose to drive the SSITXD line. 0: SSI can drive SSITXD in slave output mode 1: SSI must not drive the SSITXD output in slave mode"]
pub type SOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SSI loop-back mode (R/W) Reset value: 0x0 0: Normal serial port operation is enabled. 1: The output of the transmit serial shifter is connected to the input of the receive serial shift register internally."]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI synchronous serial port enable (R/W) Reset value: 0x0 0: SSI operation is disabled. 1: SSI operation is enabled."]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI master and slave select (R/W) Reset value: 0x0 This bit can be modified only when the SSI is disabled (SSE = 0). 0: Device configured as a master (default) 1: Device configured as a slave"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI slave mode output disable (R/W) Reset value: 0x0 This bit is relevant only in the slave mode (MS = 1). In multiple-slave systems, it is possible for the SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto the serial output line. In such systems, the RXD lines from multiple slaves could be tied together. To operate in such a system, the SOD bit can be set if the SSI slave is not suppose to drive the SSITXD line. 0: SSI can drive SSITXD in slave output mode 1: SSI must not drive the SSITXD output in slave mode"]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI loop-back mode (R/W) Reset value: 0x0 0: Normal serial port operation is enabled. 1: The output of the transmit serial shifter is connected to the input of the receive serial shift register internally."]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<CR1_SPEC> {
        LBM_W::new(self, 0)
    }
    #[doc = "Bit 1 - SSI synchronous serial port enable (R/W) Reset value: 0x0 0: SSI operation is disabled. 1: SSI operation is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SSE_W<CR1_SPEC> {
        SSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - SSI master and slave select (R/W) Reset value: 0x0 This bit can be modified only when the SSI is disabled (SSE = 0). 0: Device configured as a master (default) 1: Device configured as a slave"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<CR1_SPEC> {
        MS_W::new(self, 2)
    }
    #[doc = "Bit 3 - SSI slave mode output disable (R/W) Reset value: 0x0 This bit is relevant only in the slave mode (MS = 1). In multiple-slave systems, it is possible for the SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto the serial output line. In such systems, the RXD lines from multiple slaves could be tied together. To operate in such a system, the SOD bit can be set if the SSI slave is not suppose to drive the SSITXD line. 0: SSI can drive SSITXD in slave output mode 1: SSI must not drive the SSITXD output in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SOD_W<CR1_SPEC> {
        SOD_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The CR1 register contains bit fields that control various functions within the SSI module. Master and slave mode functionality is controlled by this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
