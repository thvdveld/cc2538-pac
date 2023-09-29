#[doc = "Register `CSIH` reader"]
pub type R = crate::R<CSIH_SPEC>;
#[doc = "Register `CSIH` writer"]
pub type W = crate::W<CSIH_SPEC>;
#[doc = "Field `INDBLBUF` reader - IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
pub type INDBLBUF_R = crate::BitReader;
#[doc = "Field `INDBLBUF` writer - IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
pub type INDBLBUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCEDATATOG` reader - Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
pub type FORCEDATATOG_R = crate::BitReader;
#[doc = "Field `FORCEDATATOG` writer - Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
pub type FORCEDATATOG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISO` reader - Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
pub type ISO_R = crate::BitReader;
#[doc = "Field `ISO` writer - Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
pub type ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTISET` reader - If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
pub type AUTISET_R = crate::BitReader;
#[doc = "Field `AUTISET` writer - If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
pub type AUTISET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    pub fn indblbuf(&self) -> INDBLBUF_R {
        INDBLBUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
    #[inline(always)]
    pub fn forcedatatog(&self) -> FORCEDATATOG_R {
        FORCEDATATOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
    #[inline(always)]
    pub fn autiset(&self) -> AUTISET_R {
        AUTISET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    #[must_use]
    pub fn indblbuf(&mut self) -> INDBLBUF_W<CSIH_SPEC, 0> {
        INDBLBUF_W::new(self)
    }
    #[doc = "Bit 3 - Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn forcedatatog(&mut self) -> FORCEDATATOG_W<CSIH_SPEC, 3> {
        FORCEDATATOG_W::new(self)
    }
    #[doc = "Bit 6 - Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> ISO_W<CSIH_SPEC, 6> {
        ISO_W::new(self)
    }
    #[doc = "Bit 7 - If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
    #[inline(always)]
    #[must_use]
    pub fn autiset(&mut self) -> AUTISET_W<CSIH_SPEC, 7> {
        AUTISET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csih::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csih::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIH_SPEC;
impl crate::RegisterSpec for CSIH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csih::R`](R) reader structure"]
impl crate::Readable for CSIH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csih::W`](W) writer structure"]
impl crate::Writable for CSIH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIH to value 0"]
impl crate::Resettable for CSIH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
