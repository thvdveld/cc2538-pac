#[doc = "Register `IM` reader"]
pub type R = crate::R<ImSpec>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<ImSpec>;
#[doc = "Field `RORIM` reader - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
pub type RorimR = crate::BitReader;
#[doc = "Field `RORIM` writer - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
pub type RorimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
    #[inline(always)]
    pub fn rorim(&self) -> RorimR {
        RorimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RorimW<ImSpec> {
        RorimW::new(self, 0)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<ImSpec> {
        RtimW::new(self, 1)
    }
    #[doc = "Bit 2 - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<ImSpec> {
        RximW::new(self, 2)
    }
    #[doc = "Bit 3 - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<ImSpec> {
        TximW::new(self, 3)
    }
}
#[doc = "The IM register is the interrupt mask set or clear register. It is a read/write register and all bits are cleared on reset. On a read, this register gives the current value of the mask on the corresponding interrupt. Setting a bit sets the mask, preventing the interrupt from being signaled to the interrupt controller. Clearing a bit clears the corresponding mask, enabling the interrupt to be sent to the interrupt controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImSpec;
impl crate::RegisterSpec for ImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for ImSpec {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for ImSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for ImSpec {
    const RESET_VALUE: u32 = 0;
}
