#[doc = "Register `CSIH` reader"]
pub struct R(crate::R<CSIH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIH` writer"]
pub struct W(crate::W<CSIH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIH_SPEC>;
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
impl From<crate::W<CSIH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTISET` reader - If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
pub struct AUTISET_R(crate::FieldReader<bool, bool>);
impl AUTISET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTISET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTISET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTISET` writer - If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
pub struct AUTISET_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTISET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ISO` reader - Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
pub struct ISO_R(crate::FieldReader<bool, bool>);
impl ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISO` writer - Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
pub struct ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FORCEDATATOG` reader - Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
pub struct FORCEDATATOG_R(crate::FieldReader<bool, bool>);
impl FORCEDATATOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCEDATATOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCEDATATOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEDATATOG` writer - Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
pub struct FORCEDATATOG_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEDATATOG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `INDBLBUF` reader - IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
pub struct INDBLBUF_R(crate::FieldReader<bool, bool>);
impl INDBLBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INDBLBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDBLBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDBLBUF` writer - IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
pub struct INDBLBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> INDBLBUF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
    #[inline(always)]
    pub fn autiset(&self) -> AUTISET_R {
        AUTISET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
    #[inline(always)]
    pub fn forcedatatog(&self) -> FORCEDATATOG_R {
        FORCEDATATOG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    pub fn indblbuf(&self) -> INDBLBUF_R {
        INDBLBUF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
    #[inline(always)]
    pub fn autiset(&mut self) -> AUTISET_W {
        AUTISET_W { w: self }
    }
    #[doc = "Bit 6 - Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W {
        ISO_W { w: self }
    }
    #[doc = "Bit 3 - Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
    #[inline(always)]
    pub fn forcedatatog(&mut self) -> FORCEDATATOG_W {
        FORCEDATATOG_W { w: self }
    }
    #[doc = "Bit 0 - IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    pub fn indblbuf(&mut self) -> INDBLBUF_W {
        INDBLBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csih](index.html) module"]
pub struct CSIH_SPEC;
impl crate::RegisterSpec for CSIH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csih::R](R) reader structure"]
impl crate::Readable for CSIH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csih::W](W) writer structure"]
impl crate::Writable for CSIH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSIH to value 0"]
impl crate::Resettable for CSIH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
