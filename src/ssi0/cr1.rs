#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `LBM` reader - SSI loop-back mode (R/W) Reset value: 0x0 0: Normal serial port operation is enabled. 1: The output of the transmit serial shifter is connected to the input of the receive serial shift register internally."]
pub type LbmR = crate::BitReader;
#[doc = "Field `LBM` writer - SSI loop-back mode (R/W) Reset value: 0x0 0: Normal serial port operation is enabled. 1: The output of the transmit serial shifter is connected to the input of the receive serial shift register internally."]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE` reader - SSI synchronous serial port enable (R/W) Reset value: 0x0 0: SSI operation is disabled. 1: SSI operation is enabled."]
pub type SseR = crate::BitReader;
#[doc = "Field `SSE` writer - SSI synchronous serial port enable (R/W) Reset value: 0x0 0: SSI operation is disabled. 1: SSI operation is enabled."]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - SSI master and slave select (R/W) Reset value: 0x0 This bit can be modified only when the SSI is disabled (SSE = 0). 0: Device configured as a master (default) 1: Device configured as a slave"]
pub type MsR = crate::BitReader;
#[doc = "Field `MS` writer - SSI master and slave select (R/W) Reset value: 0x0 This bit can be modified only when the SSI is disabled (SSE = 0). 0: Device configured as a master (default) 1: Device configured as a slave"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOD` reader - SSI slave mode output disable (R/W) Reset value: 0x0 This bit is relevant only in the slave mode (MS = 1). In multiple-slave systems, it is possible for the SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto the serial output line. In such systems, the RXD lines from multiple slaves could be tied together. To operate in such a system, the SOD bit can be set if the SSI slave is not suppose to drive the SSITXD line. 0: SSI can drive SSITXD in slave output mode 1: SSI must not drive the SSITXD output in slave mode"]
pub type SodR = crate::BitReader;
#[doc = "Field `SOD` writer - SSI slave mode output disable (R/W) Reset value: 0x0 This bit is relevant only in the slave mode (MS = 1). In multiple-slave systems, it is possible for the SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto the serial output line. In such systems, the RXD lines from multiple slaves could be tied together. To operate in such a system, the SOD bit can be set if the SSI slave is not suppose to drive the SSITXD line. 0: SSI can drive SSITXD in slave output mode 1: SSI must not drive the SSITXD output in slave mode"]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SSI loop-back mode (R/W) Reset value: 0x0 0: Normal serial port operation is enabled. 1: The output of the transmit serial shifter is connected to the input of the receive serial shift register internally."]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI synchronous serial port enable (R/W) Reset value: 0x0 0: SSI operation is disabled. 1: SSI operation is enabled."]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI master and slave select (R/W) Reset value: 0x0 This bit can be modified only when the SSI is disabled (SSE = 0). 0: Device configured as a master (default) 1: Device configured as a slave"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI slave mode output disable (R/W) Reset value: 0x0 This bit is relevant only in the slave mode (MS = 1). In multiple-slave systems, it is possible for the SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto the serial output line. In such systems, the RXD lines from multiple slaves could be tied together. To operate in such a system, the SOD bit can be set if the SSI slave is not suppose to drive the SSITXD line. 0: SSI can drive SSITXD in slave output mode 1: SSI must not drive the SSITXD output in slave mode"]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI loop-back mode (R/W) Reset value: 0x0 0: Normal serial port operation is enabled. 1: The output of the transmit serial shifter is connected to the input of the receive serial shift register internally."]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<Cr1Spec> {
        LbmW::new(self, 0)
    }
    #[doc = "Bit 1 - SSI synchronous serial port enable (R/W) Reset value: 0x0 0: SSI operation is disabled. 1: SSI operation is enabled."]
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<Cr1Spec> {
        SseW::new(self, 1)
    }
    #[doc = "Bit 2 - SSI master and slave select (R/W) Reset value: 0x0 This bit can be modified only when the SSI is disabled (SSE = 0). 0: Device configured as a master (default) 1: Device configured as a slave"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<Cr1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - SSI slave mode output disable (R/W) Reset value: 0x0 This bit is relevant only in the slave mode (MS = 1). In multiple-slave systems, it is possible for the SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto the serial output line. In such systems, the RXD lines from multiple slaves could be tied together. To operate in such a system, the SOD bit can be set if the SSI slave is not suppose to drive the SSITXD line. 0: SSI can drive SSITXD in slave output mode 1: SSI must not drive the SSITXD output in slave mode"]
    #[inline(always)]
    pub fn sod(&mut self) -> SodW<Cr1Spec> {
        SodW::new(self, 3)
    }
}
#[doc = "The CR1 register contains bit fields that control various functions within the SSI module. Master and slave mode functionality is controlled by this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
