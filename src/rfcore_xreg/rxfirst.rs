#[doc = "Register `RXFIRST` reader"]
pub struct R(crate::R<RXFIRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - First byte of the RX FIFO Note: Reading this register does not modify the contents of the FIFO."]
pub type DATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - First byte of the RX FIFO Note: Reading this register does not modify the contents of the FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "First byte in RX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfirst](index.html) module"]
pub struct RXFIRST_SPEC;
impl crate::RegisterSpec for RXFIRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfirst::R](R) reader structure"]
impl crate::Readable for RXFIRST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIRST to value 0"]
impl crate::Resettable for RXFIRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
