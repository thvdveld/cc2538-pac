#[doc = "Register `CNTH` reader"]
pub type R = crate::R<CnthSpec>;
#[doc = "Field `FIFOCNTH` reader - Bits 10:8 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
pub type FifocnthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Bits 10:8 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
    #[inline(always)]
    pub fn fifocnth(&self) -> FifocnthR {
        FifocnthR::new((self.bits & 7) as u8)
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnthSpec;
impl crate::RegisterSpec for CnthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnth::R`](R) reader structure"]
impl crate::Readable for CnthSpec {}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CnthSpec {
    const RESET_VALUE: u32 = 0;
}
