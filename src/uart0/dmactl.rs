#[doc = "Register `DMACTL` reader"]
pub type R = crate::R<DMACTL_SPEC>;
#[doc = "Register `DMACTL` writer"]
pub type W = crate::W<DMACTL_SPEC>;
#[doc = "Field `RXDMAE` reader - Receive DMA enable 1: uDMA for the receive FIFO is enabled. 0: uDMA for the receive FIFO is disabled."]
pub type RXDMAE_R = crate::BitReader;
#[doc = "Field `RXDMAE` writer - Receive DMA enable 1: uDMA for the receive FIFO is enabled. 0: uDMA for the receive FIFO is disabled."]
pub type RXDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - Transmit DMA enable 1: uDMA for the transmit FIFO is enabled. 0: uDMA for the transmit FIFO is disabled."]
pub type TXDMAE_R = crate::BitReader;
#[doc = "Field `TXDMAE` writer - Transmit DMA enable 1: uDMA for the transmit FIFO is enabled. 0: uDMA for the transmit FIFO is disabled."]
pub type TXDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAERR` reader - DMA on error 1: uDMA receive requests are automatically disabled when a receive error occurs. 0: uDMA receive requests are unaffected when a receive error occurs."]
pub type DMAERR_R = crate::BitReader;
#[doc = "Field `DMAERR` writer - DMA on error 1: uDMA receive requests are automatically disabled when a receive error occurs. 0: uDMA receive requests are unaffected when a receive error occurs."]
pub type DMAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive DMA enable 1: uDMA for the receive FIFO is enabled. 0: uDMA for the receive FIFO is disabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable 1: uDMA for the transmit FIFO is enabled. 0: uDMA for the transmit FIFO is disabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA on error 1: uDMA receive requests are automatically disabled when a receive error occurs. 0: uDMA receive requests are unaffected when a receive error occurs."]
    #[inline(always)]
    pub fn dmaerr(&self) -> DMAERR_R {
        DMAERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable 1: uDMA for the receive FIFO is enabled. 0: uDMA for the receive FIFO is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<DMACTL_SPEC> {
        RXDMAE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable 1: uDMA for the transmit FIFO is enabled. 0: uDMA for the transmit FIFO is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<DMACTL_SPEC> {
        TXDMAE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA on error 1: uDMA receive requests are automatically disabled when a receive error occurs. 0: uDMA receive requests are unaffected when a receive error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn dmaerr(&mut self) -> DMAERR_W<DMACTL_SPEC> {
        DMAERR_W::new(self, 2)
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
#[doc = "UART DMA control The DMACTL register is the DMA control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTL_SPEC;
impl crate::RegisterSpec for DMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactl::R`](R) reader structure"]
impl crate::Readable for DMACTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactl::W`](W) writer structure"]
impl crate::Writable for DMACTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DMACTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
