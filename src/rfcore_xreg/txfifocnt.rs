#[doc = "Register `TXFIFOCNT` reader"]
pub type R = crate::R<TxfifocntSpec>;
#[doc = "Field `TXFIFOCNT` reader - Number of bytes in the TX FIFO (unsigned integer)"]
pub type TxfifocntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of bytes in the TX FIFO (unsigned integer)"]
    #[inline(always)]
    pub fn txfifocnt(&self) -> TxfifocntR {
        TxfifocntR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes in TX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifocnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifocntSpec;
impl crate::RegisterSpec for TxfifocntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfifocnt::R`](R) reader structure"]
impl crate::Readable for TxfifocntSpec {}
#[doc = "`reset()` method sets TXFIFOCNT to value 0"]
impl crate::Resettable for TxfifocntSpec {
    const RESET_VALUE: u32 = 0;
}
