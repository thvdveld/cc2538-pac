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
#[doc = "Field `CRC_REN_USB` reader - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub struct CRC_REN_USB_R(crate::FieldReader<bool, bool>);
impl CRC_REN_USB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_REN_USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_REN_USB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_REN_USB` writer - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub struct CRC_REN_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_REN_USB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CRC_REN_RF` reader - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub struct CRC_REN_RF_R(crate::FieldReader<bool, bool>);
impl CRC_REN_RF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_REN_RF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_REN_RF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_REN_RF` writer - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
pub struct CRC_REN_RF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_REN_RF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    pub fn crc_ren_usb(&self) -> CRC_REN_USB_R {
        CRC_REN_USB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    pub fn crc_ren_rf(&self) -> CRC_REN_RF_R {
        CRC_REN_RF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    pub fn crc_ren_usb(&mut self) -> CRC_REN_USB_W {
        CRC_REN_USB_W { w: self }
    }
    #[doc = "Bit 0 - 1: Enable reset of chip if CRC fails. 0: Disable reset feature of chip due to CRC."]
    #[inline(always)]
    pub fn crc_ren_rf(&mut self) -> CRC_REN_RF_W {
        CRC_REN_RF_W { w: self }
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
}
#[doc = "`reset()` method sets SRCRC to value 0"]
impl crate::Resettable for SRCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
