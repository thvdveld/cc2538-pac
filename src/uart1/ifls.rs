#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "Field `TXIFLSEL` reader - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
pub type TxiflselR = crate::FieldReader;
#[doc = "Field `TXIFLSEL` writer - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXIFLSEL` reader - UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
pub type RxiflselR = crate::FieldReader;
#[doc = "Field `RXIFLSEL` writer - UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TxiflselW<IflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RxiflselW<IflsSpec> {
        RxiflselW::new(self, 3)
    }
}
#[doc = "UART interrupt FIFO level select The IFLS register is the interrupt FIFO level select register. This register can be used to define the FIFO level at which the TXRIS and RXRIS bits in the RIS register are triggered. The interrupts are generated based on a transition through a level rather than being based on the level. That is, the interrupts are generated when the fill level progresses through the trigger level. For example, if the receive trigger level is set to the half-way mark, the interrupt is triggered as the module is receiving the 9th character. Out of reset, the TXIFLSEL and RXIFLSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark.\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFLS to value 0"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0;
}
