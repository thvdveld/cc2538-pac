#[doc = "Register `MAXO` reader"]
pub struct R(crate::R<MAXO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXO` writer"]
pub struct W(crate::W<MAXO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXO_SPEC>;
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
impl From<crate::W<MAXO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBMAXO` reader - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub struct USBMAXO_R(crate::FieldReader<u8, u8>);
impl USBMAXO_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBMAXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBMAXO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBMAXO` writer - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub struct USBMAXO_W<'a> {
    w: &'a mut W,
}
impl<'a> USBMAXO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxo(&self) -> USBMAXO_R {
        USBMAXO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxo(&mut self) -> USBMAXO_W {
        USBMAXO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxo](index.html) module"]
pub struct MAXO_SPEC;
impl crate::RegisterSpec for MAXO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxo::R](R) reader structure"]
impl crate::Readable for MAXO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxo::W](W) writer structure"]
impl crate::Writable for MAXO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXO to value 0"]
impl crate::Resettable for MAXO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
