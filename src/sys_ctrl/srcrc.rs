#[doc = "Register `SRCRC` reader"]
pub struct R(crate::R<SRCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCRC` writer"]
pub struct W(crate::W<SRCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCRC_SPEC>;
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
impl From<crate::W<SRCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_REN_RF` reader - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CRC_REN_RF_R = crate::BitReader<bool>;
#[doc = "Field `CRC_REN_RF` writer - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CRC_REN_RF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCRC_SPEC, bool, O>;
#[doc = "Field `CRC_REN_USB` reader - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CRC_REN_USB_R = crate::BitReader<bool>;
#[doc = "Field `CRC_REN_USB` writer - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub type CRC_REN_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCRC_SPEC, bool, O>;
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
    pub fn crc_ren_rf(&mut self) -> CRC_REN_RF_W<0> {
        CRC_REN_RF_W::new(self)
    }
    #[doc = "Bit 8 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    #[must_use]
    pub fn crc_ren_usb(&mut self) -> CRC_REN_USB_W<8> {
        CRC_REN_USB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls CRC on state retention.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcrc](index.html) module"]
pub struct SRCRC_SPEC;
impl crate::RegisterSpec for SRCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcrc::R](R) reader structure"]
impl crate::Readable for SRCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcrc::W](W) writer structure"]
impl crate::Writable for SRCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCRC to value 0"]
impl crate::Resettable for SRCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
