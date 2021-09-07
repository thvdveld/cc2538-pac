#[doc = "Register `DMACTL` reader"]
pub struct R(crate::R<DMACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL` writer"]
pub struct W(crate::W<DMACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL_SPEC>;
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
impl From<crate::W<DMACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAERR` reader - DMA on error 1: uDMA receive requests are automatically disabled when a receive error occurs. 0: uDMA receive requests are unaffected when a receive error occurs."]
pub struct DMAERR_R(crate::FieldReader<bool, bool>);
impl DMAERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAERR` writer - DMA on error 1: uDMA receive requests are automatically disabled when a receive error occurs. 0: uDMA receive requests are unaffected when a receive error occurs."]
pub struct DMAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TXDMAE` reader - Transmit DMA enable 1: uDMA for the transmit FIFO is enabled. 0: uDMA for the transmit FIFO is disabled."]
pub struct TXDMAE_R(crate::FieldReader<bool, bool>);
impl TXDMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMAE` writer - Transmit DMA enable 1: uDMA for the transmit FIFO is enabled. 0: uDMA for the transmit FIFO is disabled."]
pub struct TXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXDMAE` reader - Receive DMA enable 1: uDMA for the receive FIFO is enabled. 0: uDMA for the receive FIFO is disabled."]
pub struct RXDMAE_R(crate::FieldReader<bool, bool>);
impl RXDMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMAE` writer - Receive DMA enable 1: uDMA for the receive FIFO is enabled. 0: uDMA for the receive FIFO is disabled."]
pub struct RXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - DMA on error 1: uDMA receive requests are automatically disabled when a receive error occurs. 0: uDMA receive requests are unaffected when a receive error occurs."]
    #[inline(always)]
    pub fn dmaerr(&self) -> DMAERR_R {
        DMAERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable 1: uDMA for the transmit FIFO is enabled. 0: uDMA for the transmit FIFO is disabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive DMA enable 1: uDMA for the receive FIFO is enabled. 0: uDMA for the receive FIFO is disabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DMA on error 1: uDMA receive requests are automatically disabled when a receive error occurs. 0: uDMA receive requests are unaffected when a receive error occurs."]
    #[inline(always)]
    pub fn dmaerr(&mut self) -> DMAERR_W {
        DMAERR_W { w: self }
    }
    #[doc = "Bit 1 - Transmit DMA enable 1: uDMA for the transmit FIFO is enabled. 0: uDMA for the transmit FIFO is disabled."]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TXDMAE_W {
        TXDMAE_W { w: self }
    }
    #[doc = "Bit 0 - Receive DMA enable 1: uDMA for the receive FIFO is enabled. 0: uDMA for the receive FIFO is disabled."]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RXDMAE_W {
        RXDMAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART DMA control The DMACTL register is the DMA control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](index.html) module"]
pub struct DMACTL_SPEC;
impl crate::RegisterSpec for DMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactl::R](R) reader structure"]
impl crate::Readable for DMACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl::W](W) writer structure"]
impl crate::Writable for DMACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DMACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
