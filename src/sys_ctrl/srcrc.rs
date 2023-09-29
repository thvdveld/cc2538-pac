#[doc = "Register `SRCRC` reader"]
pub type R = crate::R<SRCRC_SPEC>;
#[doc = "Register `SRCRC` writer"]
pub type W = crate::W<SRCRC_SPEC>;
#[doc = "Field `CRC_REN_RF` reader - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CRC_REN_RF_R = crate::BitReader;
#[doc = "Field `CRC_REN_RF` writer - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CRC_REN_RF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC_REN_USB` reader - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CRC_REN_USB_R = crate::BitReader;
#[doc = "Field `CRC_REN_USB` writer - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CRC_REN_USB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    pub fn crc_ren_rf(&self) -> CRC_REN_RF_R {
        CRC_REN_RF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    pub fn crc_ren_usb(&self) -> CRC_REN_USB_R {
        CRC_REN_USB_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    #[must_use]
    pub fn crc_ren_rf(&mut self) -> CRC_REN_RF_W<SRCRC_SPEC, 0> {
        CRC_REN_RF_W::new(self)
    }
    #[doc = "Bit 8 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    #[must_use]
    pub fn crc_ren_usb(&mut self) -> CRC_REN_USB_W<SRCRC_SPEC, 8> {
        CRC_REN_USB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls CRC on state retention.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCRC_SPEC;
impl crate::RegisterSpec for SRCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcrc::R`](R) reader structure"]
impl crate::Readable for SRCRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcrc::W`](W) writer structure"]
impl crate::Writable for SRCRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCRC to value 0"]
impl crate::Resettable for SRCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
