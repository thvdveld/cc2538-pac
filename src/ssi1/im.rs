#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXIM` reader - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
pub type TXIM_R = crate::BitReader<bool>;
#[doc = "Field `TXIM` writer - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
pub type TXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `RXIM` reader - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
pub type RXIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIM` writer - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
pub type RXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `RTIM` reader - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
pub type RTIM_R = crate::BitReader<bool>;
#[doc = "Field `RTIM` writer - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
pub type RTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `RORIM` reader - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
pub type RORIM_R = crate::BitReader<bool>;
#[doc = "Field `RORIM` writer - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
pub type RORIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
    #[inline(always)]
    pub fn rorim(&self) -> RORIM_R {
        RORIM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - SSI transmit FIFO interrupt mask (R/W) Reset value: 0x0 0: TX FIFO half empty or condition interrupt is masked. 1: TX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W<3> {
        TXIM_W::new(self)
    }
    #[doc = "Bit 2 - SSI receive FIFO interrupt mask (R/W) Reset value: 0x0 0: RX FIFO half empty or condition interrupt is masked. 1: RX FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W<2> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt mask (R/W) Reset value: 0x0 0: RX FIFO time-out interrupt is masked. 1: RX FIFO time-out interrupt is not masked"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W<1> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 0 - SSI receive overrun interrupt mask (R/W) Reset value: 0x0 0: RX FIFO Overrun interrupt is masked. 1: RX FIFO Overrun interrupt is not masked"]
    #[inline(always)]
    pub fn rorim(&mut self) -> RORIM_W<0> {
        RORIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The IM register is the interrupt mask set or clear register. It is a read/write register and all bits are cleared on reset. On a read, this register gives the current value of the mask on the corresponding interrupt. Setting a bit sets the mask, preventing the interrupt from being signaled to the interrupt controller. Clearing a bit clears the corresponding mask, enabling the interrupt to be sent to the interrupt controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
