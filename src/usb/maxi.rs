#[doc = "Register `MAXI` reader"]
pub struct R(crate::R<MAXI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXI` writer"]
pub struct W(crate::W<MAXI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXI_SPEC>;
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
impl From<crate::W<MAXI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBMAXI` reader - Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub type USBMAXI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBMAXI` writer - Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub type USBMAXI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAXI_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxi(&self) -> USBMAXI_R {
        USBMAXI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxi(&mut self) -> USBMAXI_W<0> {
        USBMAXI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxi](index.html) module"]
pub struct MAXI_SPEC;
impl crate::RegisterSpec for MAXI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxi::R](R) reader structure"]
impl crate::Readable for MAXI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxi::W](W) writer structure"]
impl crate::Writable for MAXI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXI to value 0"]
impl crate::Resettable for MAXI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
