#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RXRIS` reader - UART receive raw interrupt status 1: The receive FIFO level has passed through the condition defined in the UARTIFLS register. 0: No interrupt This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - UART transmit raw interrupt status 1: If the EOT bit in the UARTCTL register is clear, the transmit FIFO level has passed through the condition defined in the UARTIFLS register. If the EOT bit is set, the last bit of all transmitted data and flags has left the serializer. 0: No interrupt This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
pub type TxrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - UART receive time-out raw interrupt status 1: A receive time out has occurred. 0: No interrupt This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
pub type RtrisR = crate::BitReader;
#[doc = "Field `FERIS` reader - UART framing error raw interrupt status 1: A framing error has occurred. 0: No interrupt This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
pub type FerisR = crate::BitReader;
#[doc = "Field `PERIS` reader - UART parity error raw interrupt status 1: A parity error has occurred. 0: No interrupt This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
pub type PerisR = crate::BitReader;
#[doc = "Field `BERIS` reader - UART break error raw interrupt status 1: A break error has occurred. 0: No interrupt This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
pub type BerisR = crate::BitReader;
#[doc = "Field `OERIS` reader - UART overrun error raw interrupt status 1: An overrun error has occurred. 0: No interrupt This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
pub type OerisR = crate::BitReader;
#[doc = "Field `NINEBITRIS` reader - 9-mit mode raw interrupt status 1: A receive address match has occurred. 0: No interrupt This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
pub type NinebitrisR = crate::BitReader;
#[doc = "Field `LMSBRIS` reader - LIN mode sync break raw interrupt status 1: A LIN sync break has been detected. 0: No interrupt This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
pub type LmsbrisR = crate::BitReader;
#[doc = "Field `LME1RIS` reader - LIN mode edge 1 raw interrupt status 1: The timer value at the 1st falling edge of the LIN Sync Field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
pub type Lme1risR = crate::BitReader;
#[doc = "Field `LME5RIS` reader - LIN mode edge 5 raw interrupt status 1: The timer value at the 5th falling edge of the LIN sync field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
pub type Lme5risR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - UART receive raw interrupt status 1: The receive FIFO level has passed through the condition defined in the UARTIFLS register. 0: No interrupt This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmit raw interrupt status 1: If the EOT bit in the UARTCTL register is clear, the transmit FIFO level has passed through the condition defined in the UARTIFLS register. If the EOT bit is set, the last bit of all transmitted data and flags has left the serializer. 0: No interrupt This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART receive time-out raw interrupt status 1: A receive time out has occurred. 0: No interrupt This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART framing error raw interrupt status 1: A framing error has occurred. 0: No interrupt This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn feris(&self) -> FerisR {
        FerisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART parity error raw interrupt status 1: A parity error has occurred. 0: No interrupt This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn peris(&self) -> PerisR {
        PerisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART break error raw interrupt status 1: A break error has occurred. 0: No interrupt This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn beris(&self) -> BerisR {
        BerisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART overrun error raw interrupt status 1: An overrun error has occurred. 0: No interrupt This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oeris(&self) -> OerisR {
        OerisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-mit mode raw interrupt status 1: A receive address match has occurred. 0: No interrupt This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitris(&self) -> NinebitrisR {
        NinebitrisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LIN mode sync break raw interrupt status 1: A LIN sync break has been detected. 0: No interrupt This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbris(&self) -> LmsbrisR {
        LmsbrisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN mode edge 1 raw interrupt status 1: The timer value at the 1st falling edge of the LIN Sync Field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1ris(&self) -> Lme1risR {
        Lme1risR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN mode edge 5 raw interrupt status 1: The timer value at the 5th falling edge of the LIN sync field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5ris(&self) -> Lme5risR {
        Lme5risR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "UART raw interrupt status The RIS register is the raw interrupt status register. On a read, this register gives the current raw status value of the corresponding interrupt. A write has no effect. Note that the HW modem flow control bits are only implemented on UART1 and are tied inactive on UART0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
