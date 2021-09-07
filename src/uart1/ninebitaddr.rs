#[doc = "Register `NINEBITADDR` reader"]
pub struct R(crate::R<NINEBITADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NINEBITADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NINEBITADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NINEBITADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NINEBITADDR` writer"]
pub struct W(crate::W<NINEBITADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NINEBITADDR_SPEC>;
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
impl From<crate::W<NINEBITADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NINEBITADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NINEBITEN` reader - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
pub struct NINEBITEN_R(crate::FieldReader<bool, bool>);
impl NINEBITEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NINEBITEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NINEBITEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NINEBITEN` writer - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
pub struct NINEBITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NINEBITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ADDR` reader - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
    #[inline(always)]
    pub fn ninebiten(&self) -> NINEBITEN_R {
        NINEBITEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Enable 9-bit mode 1: 9-bit mode is enabled. 0: 9-bit mode is disabled."]
    #[inline(always)]
    pub fn ninebiten(&mut self) -> NINEBITEN_W {
        NINEBITEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Self address for 9-bit mode This field contains the address that should be matched when UART9BITAMASK is 0xFF."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 9-bit self address The NINEBITADDR register is used to write the specific address that should be matched with the receiving byte when the 9-bit address mask (NINEBITAMASK) is set to 0xFF. This register is used in conjunction with NINEBITAMASK to form a match for address-byte received.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ninebitaddr](index.html) module"]
pub struct NINEBITADDR_SPEC;
impl crate::RegisterSpec for NINEBITADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ninebitaddr::R](R) reader structure"]
impl crate::Readable for NINEBITADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ninebitaddr::W](W) writer structure"]
impl crate::Writable for NINEBITADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NINEBITADDR to value 0"]
impl crate::Resettable for NINEBITADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
