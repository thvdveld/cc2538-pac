#[doc = "Register `CSOH` reader"]
pub type R = crate::R<CSOH_SPEC>;
#[doc = "Register `CSOH` writer"]
pub type W = crate::W<CSOH_SPEC>;
#[doc = "Field `OUTDBLBUF` reader - OUT endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
pub type OUTDBLBUF_R = crate::BitReader;
#[doc = "Field `OUTDBLBUF` writer - OUT endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
pub type OUTDBLBUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO` reader - Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
pub type ISO_R = crate::BitReader;
#[doc = "Field `ISO` writer - Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
pub type ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCLEAR` reader - If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
pub type AUTOCLEAR_R = crate::BitReader;
#[doc = "Field `AUTOCLEAR` writer - If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
pub type AUTOCLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn outdblbuf(&mut self) -> OUTDBLBUF_W<CSOH_SPEC> {
        OUTDBLBUF_W::new(self, 0)
    }
    #[doc = "Bit 6 - Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> ISO_W<CSOH_SPEC> {
        ISO_W::new(self, 6)
    }
    #[doc = "Bit 7 - If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
    #[inline(always)]
    #[must_use]
    pub fn autoclear(&mut self) -> AUTOCLEAR_W<CSOH_SPEC> {
        AUTOCLEAR_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csoh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csoh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSOH_SPEC;
impl crate::RegisterSpec for CSOH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csoh::R`](R) reader structure"]
impl crate::Readable for CSOH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csoh::W`](W) writer structure"]
impl crate::Writable for CSOH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSOH to value 0"]
impl crate::Resettable for CSOH_SPEC {
    const RESET_VALUE: u32 = 0;
}
