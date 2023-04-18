#[doc = "Register `RXLAST_PTR` reader"]
pub struct R(crate::R<RXLAST_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXLAST_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXLAST_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXLAST_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXLAST_PTR` reader - RAM address offset of the last byte + 1 byte in the RX FIFO"]
pub type RXLAST_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the last byte + 1 byte in the RX FIFO"]
    #[inline(always)]
    pub fn rxlast_ptr(&self) -> RXLAST_PTR_R {
        RXLAST_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxlast_ptr](index.html) module"]
pub struct RXLAST_PTR_SPEC;
impl crate::RegisterSpec for RXLAST_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxlast_ptr::R](R) reader structure"]
impl crate::Readable for RXLAST_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXLAST_PTR to value 0"]
impl crate::Resettable for RXLAST_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
