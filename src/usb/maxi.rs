#[doc = "Register `MAXI` reader"]
pub type R = crate::R<MaxiSpec>;
#[doc = "Register `MAXI` writer"]
pub type W = crate::W<MaxiSpec>;
#[doc = "Field `USBMAXI` reader - Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub type UsbmaxiR = crate::FieldReader;
#[doc = "Field `USBMAXI` writer - Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub type UsbmaxiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxi(&self) -> UsbmaxiR {
        UsbmaxiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    #[must_use]
    pub fn usbmaxi(&mut self) -> UsbmaxiW<MaxiSpec> {
        UsbmaxiW::new(self, 0)
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxiSpec;
impl crate::RegisterSpec for MaxiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxi::R`](R) reader structure"]
impl crate::Readable for MaxiSpec {}
#[doc = "`write(|w| ..)` method takes [`maxi::W`](W) writer structure"]
impl crate::Writable for MaxiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXI to value 0"]
impl crate::Resettable for MaxiSpec {
    const RESET_VALUE: u32 = 0;
}
