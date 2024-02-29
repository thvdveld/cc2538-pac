#[doc = "Register `SRCRC` reader"]
pub type R = crate::R<SrcrcSpec>;
#[doc = "Register `SRCRC` writer"]
pub type W = crate::W<SrcrcSpec>;
#[doc = "Field `CRC_REN_RF` reader - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CrcRenRfR = crate::BitReader;
#[doc = "Field `CRC_REN_RF` writer - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CrcRenRfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_REN_USB` reader - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CrcRenUsbR = crate::BitReader;
#[doc = "Field `CRC_REN_USB` writer - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CrcRenUsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    pub fn crc_ren_rf(&self) -> CrcRenRfR {
        CrcRenRfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    pub fn crc_ren_usb(&self) -> CrcRenUsbR {
        CrcRenUsbR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    #[must_use]
    pub fn crc_ren_rf(&mut self) -> CrcRenRfW<SrcrcSpec> {
        CrcRenRfW::new(self, 0)
    }
    #[doc = "Bit 8 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    #[must_use]
    pub fn crc_ren_usb(&mut self) -> CrcRenUsbW<SrcrcSpec> {
        CrcRenUsbW::new(self, 8)
    }
}
#[doc = "This register controls CRC on state retention.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcrcSpec;
impl crate::RegisterSpec for SrcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcrc::R`](R) reader structure"]
impl crate::Readable for SrcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`srcrc::W`](W) writer structure"]
impl crate::Writable for SrcrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCRC to value 0"]
impl crate::Resettable for SrcrcSpec {
    const RESET_VALUE: u32 = 0;
}
