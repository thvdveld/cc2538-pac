#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `USBADDR` reader - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
pub type UsbaddrR = crate::FieldReader;
#[doc = "Field `USBADDR` writer - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
pub type UsbaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `UPDATE` reader - This bit is set by hardware when writing to this register, and is cleared by hardware when the new address becomes effective."]
pub type UpdateR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
    #[inline(always)]
    pub fn usbaddr(&self) -> UsbaddrR {
        UsbaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - This bit is set by hardware when writing to this register, and is cleared by hardware when the new address becomes effective."]
    #[inline(always)]
    pub fn update(&self) -> UpdateR {
        UpdateR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
    #[inline(always)]
    #[must_use]
    pub fn usbaddr(&mut self) -> UsbaddrW<AddrSpec> {
        UsbaddrW::new(self, 0)
    }
}
#[doc = "Function address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0;
}
