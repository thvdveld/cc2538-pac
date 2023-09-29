#[doc = "Register `IM` reader"]
pub type R = crate::R<IM_SPEC>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<IM_SPEC>;
#[doc = "Field `RORIM` reader - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
pub type RORIM_R = crate::BitReader;
#[doc = "Field `RORIM` writer - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
pub type RORIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTIM` reader - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
pub type RTIM_R = crate::BitReader;
#[doc = "Field `RTIM` writer - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
pub type RTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXIM` reader - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
pub type RXIM_R = crate::BitReader;
#[doc = "Field `RXIM` writer - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
pub type RXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXIM` reader - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
pub type TXIM_R = crate::BitReader;
#[doc = "Field `TXIM` writer - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
pub type TXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
    #[inline(always)]
    pub fn rorim(&self) -> RORIM_R {
        RORIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RORIM_W<IM_SPEC, 0> {
        RORIM_W::new(self)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<IM_SPEC, 1> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 2 - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<IM_SPEC, 2> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 3 - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<IM_SPEC, 3> {
        TXIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The IM register is the interrupt mask set or clear register. It is a read/write register and all bits are cleared on reset. On a read, this register gives the current value of the mask on the corresponding interrupt. Setting a bit sets the mask, preventing the interrupt from being signaled to the interrupt controller. Clearing a bit clears the corresponding mask, enabling the interrupt to be sent to the interrupt controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for IM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for IM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
