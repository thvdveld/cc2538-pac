#[doc = "Register `POW` reader"]
pub struct R(crate::R<POW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POW` writer"]
pub struct W(crate::W<POW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POW_SPEC>;
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
impl From<crate::W<POW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPENDEN` reader - Enables detection of and entry into suspend mode."]
pub type SUSPENDEN_R = crate::BitReader<bool>;
#[doc = "Field `SUSPENDEN` writer - Enables detection of and entry into suspend mode."]
pub type SUSPENDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POW_SPEC, bool, O>;
#[doc = "Field `SUSPEND` reader - Indicates entry into suspend mode Suspend mode must be enabled by setting USB_POW.SUSPENDEN Software clears this bit by reading the USB_CIF register or by asserting USB_POW.RESUME"]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` reader - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, POW_SPEC, bool, O>;
#[doc = "Field `RST` reader - Indicates that reset signaling is present on the bus"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `ISOWAITSOF` reader - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
pub type ISOWAITSOF_R = crate::BitReader<bool>;
#[doc = "Field `ISOWAITSOF` writer - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
pub type ISOWAITSOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POW_SPEC, bool, O>;
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
    pub fn suspenden(&mut self) -> SUSPENDEN_W<0> {
        SUSPENDEN_W::new(self)
    }
    #[doc = "Bit 2 - Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<2> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 7 - For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
    #[inline(always)]
    pub fn isowaitsof(&mut self) -> ISOWAITSOF_W<7> {
        ISOWAITSOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power management and control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pow](index.html) module"]
pub struct POW_SPEC;
impl crate::RegisterSpec for POW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pow::R](R) reader structure"]
impl crate::Readable for POW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pow::W](W) writer structure"]
impl crate::Writable for POW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POW to value 0"]
impl crate::Resettable for POW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
