#[doc = "Register `POW` reader"]
pub type R = crate::R<POW_SPEC>;
#[doc = "Register `POW` writer"]
pub type W = crate::W<POW_SPEC>;
#[doc = "Field `SUSPENDEN` reader - Enables detection of and entry into suspend mode."]
pub type SUSPENDEN_R = crate::BitReader;
#[doc = "Field `SUSPENDEN` writer - Enables detection of and entry into suspend mode."]
pub type SUSPENDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUSPEND` reader - Indicates entry into suspend mode Suspend mode must be enabled by setting USB_POW.SUSPENDEN Software clears this bit by reading the USB_CIF register or by asserting USB_POW.RESUME"]
pub type SUSPEND_R = crate::BitReader;
#[doc = "Field `RESUME` reader - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
pub type RESUME_R = crate::BitReader;
#[doc = "Field `RESUME` writer - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
pub type RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RST` reader - Indicates that reset signaling is present on the bus"]
pub type RST_R = crate::BitReader;
#[doc = "Field `ISOWAITSOF` reader - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
pub type ISOWAITSOF_R = crate::BitReader;
#[doc = "Field `ISOWAITSOF` writer - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
pub type ISOWAITSOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enables detection of and entry into suspend mode."]
    #[inline(always)]
    pub fn suspenden(&self) -> SUSPENDEN_R {
        SUSPENDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates entry into suspend mode Suspend mode must be enabled by setting USB_POW.SUSPENDEN Software clears this bit by reading the USB_CIF register or by asserting USB_POW.RESUME"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that reset signaling is present on the bus"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
    #[inline(always)]
    pub fn isowaitsof(&self) -> ISOWAITSOF_R {
        ISOWAITSOF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables detection of and entry into suspend mode."]
    #[inline(always)]
    #[must_use]
    pub fn suspenden(&mut self) -> SUSPENDEN_W<POW_SPEC, 0> {
        SUSPENDEN_W::new(self)
    }
    #[doc = "Bit 2 - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<POW_SPEC, 2> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 7 - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
    #[inline(always)]
    #[must_use]
    pub fn isowaitsof(&mut self) -> ISOWAITSOF_W<POW_SPEC, 7> {
        ISOWAITSOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Power management and control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POW_SPEC;
impl crate::RegisterSpec for POW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pow::R`](R) reader structure"]
impl crate::Readable for POW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pow::W`](W) writer structure"]
impl crate::Writable for POW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POW to value 0"]
impl crate::Resettable for POW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
