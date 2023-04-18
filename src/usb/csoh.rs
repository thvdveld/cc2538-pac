#[doc = "Register `CSOH` reader"]
pub struct R(crate::R<CSOH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSOH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSOH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSOH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSOH` writer"]
pub struct W(crate::W<CSOH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSOH_SPEC>;
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
impl From<crate::W<CSOH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSOH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTDBLBUF` reader - OUT endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
pub type OUTDBLBUF_R = crate::BitReader<bool>;
#[doc = "Field `OUTDBLBUF` writer - OUT endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
pub type OUTDBLBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOH_SPEC, bool, O>;
#[doc = "Field `ISO` reader - Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
pub type ISO_R = crate::BitReader<bool>;
#[doc = "Field `ISO` writer - Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
pub type ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOH_SPEC, bool, O>;
#[doc = "Field `AUTOCLEAR` reader - If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
pub type AUTOCLEAR_R = crate::BitReader<bool>;
#[doc = "Field `AUTOCLEAR` writer - If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
pub type AUTOCLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OUT endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    pub fn outdblbuf(&self) -> OUTDBLBUF_R {
        OUTDBLBUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
    #[inline(always)]
    pub fn autoclear(&self) -> AUTOCLEAR_R {
        AUTOCLEAR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUT endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    #[must_use]
    pub fn outdblbuf(&mut self) -> OUTDBLBUF_W<0> {
        OUTDBLBUF_W::new(self)
    }
    #[doc = "Bit 6 - Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> ISO_W<6> {
        ISO_W::new(self)
    }
    #[doc = "Bit 7 - If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
    #[inline(always)]
    #[must_use]
    pub fn autoclear(&mut self) -> AUTOCLEAR_W<7> {
        AUTOCLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csoh](index.html) module"]
pub struct CSOH_SPEC;
impl crate::RegisterSpec for CSOH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csoh::R](R) reader structure"]
impl crate::Readable for CSOH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csoh::W](W) writer structure"]
impl crate::Writable for CSOH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSOH to value 0"]
impl crate::Resettable for CSOH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
