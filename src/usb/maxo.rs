#[doc = "Register `MAXO` reader"]
pub type R = crate::R<MaxoSpec>;
#[doc = "Register `MAXO` writer"]
pub type W = crate::W<MaxoSpec>;
#[doc = "Field `USBMAXO` reader - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub type UsbmaxoR = crate::FieldReader;
#[doc = "Field `USBMAXO` writer - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
pub type UsbmaxoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxo(&self) -> UsbmaxoR {
        UsbmaxoR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum packet size, in units of 8 bytes, for the selected OUT endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxo(&mut self) -> UsbmaxoW<MaxoSpec> {
        UsbmaxoW::new(self, 0)
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}\n\nYou can [`read`](crate::Reg::read) this register and get [`maxo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxoSpec;
impl crate::RegisterSpec for MaxoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxo::R`](R) reader structure"]
impl crate::Readable for MaxoSpec {}
#[doc = "`write(|w| ..)` method takes [`maxo::W`](W) writer structure"]
impl crate::Writable for MaxoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXO to value 0"]
impl crate::Resettable for MaxoSpec {
    const RESET_VALUE: u32 = 0;
}
