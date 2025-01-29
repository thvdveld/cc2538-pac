#[doc = "Register `POW` reader"]
pub type R = crate::R<PowSpec>;
#[doc = "Register `POW` writer"]
pub type W = crate::W<PowSpec>;
#[doc = "Field `SUSPENDEN` reader - Enables detection of and entry into suspend mode."]
pub type SuspendenR = crate::BitReader;
#[doc = "Field `SUSPENDEN` writer - Enables detection of and entry into suspend mode."]
pub type SuspendenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND` reader - Indicates entry into suspend mode Suspend mode must be enabled by setting USB_POW.SUSPENDEN Software clears this bit by reading the USB_CIF register or by asserting USB_POW.RESUME"]
pub type SuspendR = crate::BitReader;
#[doc = "Field `RESUME` reader - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Indicates that reset signaling is present on the bus"]
pub type RstR = crate::BitReader;
#[doc = "Field `ISOWAITSOF` reader - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
pub type IsowaitsofR = crate::BitReader;
#[doc = "Field `ISOWAITSOF` writer - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
pub type IsowaitsofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables detection of and entry into suspend mode."]
    #[inline(always)]
    pub fn suspenden(&self) -> SuspendenR {
        SuspendenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates entry into suspend mode Suspend mode must be enabled by setting USB_POW.SUSPENDEN Software clears this bit by reading the USB_CIF register or by asserting USB_POW.RESUME"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that reset signaling is present on the bus"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
    #[inline(always)]
    pub fn isowaitsof(&self) -> IsowaitsofR {
        IsowaitsofR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables detection of and entry into suspend mode."]
    #[inline(always)]
    pub fn suspenden(&mut self) -> SuspendenW<PowSpec> {
        SuspendenW::new(self, 0)
    }
    #[doc = "Bit 2 - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<PowSpec> {
        ResumeW::new(self, 2)
    }
    #[doc = "Bit 7 - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
    #[inline(always)]
    pub fn isowaitsof(&mut self) -> IsowaitsofW<PowSpec> {
        IsowaitsofW::new(self, 7)
    }
}
#[doc = "Power management and control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowSpec;
impl crate::RegisterSpec for PowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pow::R`](R) reader structure"]
impl crate::Readable for PowSpec {}
#[doc = "`write(|w| ..)` method takes [`pow::W`](W) writer structure"]
impl crate::Writable for PowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POW to value 0"]
impl crate::Resettable for PowSpec {
    const RESET_VALUE: u32 = 0;
}
