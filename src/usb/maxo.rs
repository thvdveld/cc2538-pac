#[doc = "Register `MAXO` reader"]
pub type R = crate::R<MAXO_SPEC>;
#[doc = "Register `MAXO` writer"]
pub type W = crate::W<MAXO_SPEC>;
#[doc = "Field `USBMAXO` reader - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub type USBMAXO_R = crate::FieldReader;
#[doc = "Field `USBMAXO` writer - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub type USBMAXO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    #[must_use]
    pub fn usbmaxo(&mut self) -> USBMAXO_W<MAXO_SPEC, 0> {
        USBMAXO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXO_SPEC;
impl crate::RegisterSpec for MAXO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxo::R`](R) reader structure"]
impl crate::Readable for MAXO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maxo::W`](W) writer structure"]
impl crate::Writable for MAXO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAXO to value 0"]
impl crate::Resettable for MAXO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
