#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `RXMIS` reader - UART receive masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified receive FIFO level. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - UART transmit masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified transmit FIFO level (if the EOT bit is clear) or due to the transmission of the last data bit (if the EOT bit is set). 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
pub type TxmisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - UART receive time-out masked interrupt status 1: An unmasked interrupt was signaled due to a receive time out. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
pub type RtmisR = crate::BitReader;
#[doc = "Field `FEMIS` reader - UART framing error masked interrupt status 1: An unmasked interrupt was signaled due to a framing error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
pub type FemisR = crate::BitReader;
#[doc = "Field `PEMIS` reader - UART parity error masked interrupt status 1: An unmasked interrupt was signaled due to a parity error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
pub type PemisR = crate::BitReader;
#[doc = "Field `BEMIS` reader - UART break error masked interrupt status 1: An unmasked interrupt was signaled due to a break error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
pub type BemisR = crate::BitReader;
#[doc = "Field `OEMIS` reader - UART overrun error masked interrupt status 1: An unmasked interrupt was signaled due to an overrun error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
pub type OemisR = crate::BitReader;
#[doc = "Field `NINEBITMIS` reader - 9-bit mode masked interrupt status 1: An unmasked interrupt was signaled due to a receive address match. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
pub type NinebitmisR = crate::BitReader;
#[doc = "Field `LMSBMIS` reader - LIN mode sync break masked interrupt status 1: An unmasked interrupt was signaled due to the receipt of a LIN sync break. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
pub type LmsbmisR = crate::BitReader;
#[doc = "Field `LME1MIS` reader - LIN mode edge 1 masked interrupt status 1: An unmasked interrupt was signaled due to the 1st falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
pub type Lme1misR = crate::BitReader;
#[doc = "Field `LME5MIS` reader - LIN mode edge 5 masked interrupt status 1: An unmasked interrupt was signaled due to the 5th falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
pub type Lme5misR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - UART receive masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified receive FIFO level. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmit masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified transmit FIFO level (if the EOT bit is clear) or due to the transmission of the last data bit (if the EOT bit is set). 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART receive time-out masked interrupt status 1: An unmasked interrupt was signaled due to a receive time out. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART framing error masked interrupt status 1: An unmasked interrupt was signaled due to a framing error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn femis(&self) -> FemisR {
        FemisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART parity error masked interrupt status 1: An unmasked interrupt was signaled due to a parity error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn pemis(&self) -> PemisR {
        PemisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART break error masked interrupt status 1: An unmasked interrupt was signaled due to a break error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn bemis(&self) -> BemisR {
        BemisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART overrun error masked interrupt status 1: An unmasked interrupt was signaled due to an overrun error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oemis(&self) -> OemisR {
        OemisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-bit mode masked interrupt status 1: An unmasked interrupt was signaled due to a receive address match. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitmis(&self) -> NinebitmisR {
        NinebitmisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LIN mode sync break masked interrupt status 1: An unmasked interrupt was signaled due to the receipt of a LIN sync break. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbmis(&self) -> LmsbmisR {
        LmsbmisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN mode edge 1 masked interrupt status 1: An unmasked interrupt was signaled due to the 1st falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1mis(&self) -> Lme1misR {
        Lme1misR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN mode edge 5 masked interrupt status 1: An unmasked interrupt was signaled due to the 5th falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5mis(&self) -> Lme5misR {
        Lme5misR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "UART masked interrupt status The MIS register is the masked interrupt status register. On a read, this register gives the current masked status value of the corresponding interrupt. A write has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
