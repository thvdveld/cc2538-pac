#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<ADDR_SPEC>;
#[doc = "Field `USBADDR` reader - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
pub type USBADDR_R = crate::FieldReader;
#[doc = "Field `USBADDR` writer - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
pub type USBADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `UPDATE` reader - This bit is set by hardware when writing to this register, and is cleared by hardware when the new address becomes effective."]
pub type UPDATE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - This bit is set by hardware when writing to this register, and is cleared by hardware when the new address becomes effective."]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
    #[inline(always)]
    #[must_use]
    pub fn usbaddr(&mut self) -> USBADDR_W<ADDR_SPEC, 0> {
        USBADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Function address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
