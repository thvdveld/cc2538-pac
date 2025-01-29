#[doc = "Register `CNT0_CNTL` reader"]
pub type R = crate::R<Cnt0CntlSpec>;
#[doc = "Field `FIFOCNT_or_FIFOCNTL` reader - USB_CS0.FIFOCNT (USBINDEX = 0) \\[RO\\]: Number of bytes received in the packet in the endpoint 0 FIFO Valid only when USB_CS0.OUTPKTRDY is set USB_CSIL.FIFOCNTL (USBINDEX = 1 to 5) \\[RW\\]: Bits 7:0 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
pub type FifocntOrFifocntlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - USB_CS0.FIFOCNT (USBINDEX = 0) \\[RO\\]: Number of bytes received in the packet in the endpoint 0 FIFO Valid only when USB_CS0.OUTPKTRDY is set USB_CSIL.FIFOCNTL (USBINDEX = 1 to 5) \\[RW\\]: Bits 7:0 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
    #[inline(always)]
    pub fn fifocnt_or_fifocntl(&self) -> FifocntOrFifocntlR {
        FifocntOrFifocntlR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt0_cntl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cnt0CntlSpec;
impl crate::RegisterSpec for Cnt0CntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt0_cntl::R`](R) reader structure"]
impl crate::Readable for Cnt0CntlSpec {}
#[doc = "`reset()` method sets CNT0_CNTL to value 0"]
impl crate::Resettable for Cnt0CntlSpec {
    const RESET_VALUE: u32 = 0;
}
