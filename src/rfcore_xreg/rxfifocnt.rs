#[doc = "Register `RXFIFOCNT` reader"]
pub struct R(crate::R<RXFIFOCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFOCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFOCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFOCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFOCNT` reader - Number of bytes in the RX FIFO (unsigned integer)"]
pub type RXFIFOCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of bytes in the RX FIFO (unsigned integer)"]
    #[inline(always)]
    pub fn rxfifocnt(&self) -> RXFIFOCNT_R {
        RXFIFOCNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes in RX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifocnt](index.html) module"]
pub struct RXFIFOCNT_SPEC;
impl crate::RegisterSpec for RXFIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifocnt::R](R) reader structure"]
impl crate::Readable for RXFIFOCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIFOCNT to value 0"]
impl crate::Resettable for RXFIFOCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
