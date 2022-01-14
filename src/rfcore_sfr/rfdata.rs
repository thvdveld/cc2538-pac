#[doc = "Register `RFDATA` reader"]
pub struct R(crate::R<RFDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFDATA` writer"]
pub struct W(crate::W<RFDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFDATA_SPEC>;
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
impl From<crate::W<RFDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFD` reader - Data written to the register is written to the TX FIFO. When reading this register, data from the RX FIFO is read."]
pub struct RFD_R(crate::FieldReader<u8, u8>);
impl RFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFD` writer - Data written to the register is written to the TX FIFO. When reading this register, data from the RX FIFO is read."]
pub struct RFD_W<'a> {
    w: &'a mut W,
}
impl<'a> RFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data written to the register is written to the TX FIFO. When reading this register, data from the RX FIFO is read."]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data written to the register is written to the TX FIFO. When reading this register, data from the RX FIFO is read."]
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W {
        RFD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfdata](index.html) module"]
pub struct RFDATA_SPEC;
impl crate::RegisterSpec for RFDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfdata::R](R) reader structure"]
impl crate::Readable for RFDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfdata::W](W) writer structure"]
impl crate::Writable for RFDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFDATA to value 0"]
impl crate::Resettable for RFDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
