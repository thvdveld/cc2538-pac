#[doc = "Register `TXCTRL` reader"]
pub struct R(crate::R<TXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTRL` writer"]
pub struct W(crate::W<TXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTRL_SPEC>;
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
impl From<crate::W<TXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_CURR` reader - Change the current in the DAC."]
pub struct DAC_CURR_R(crate::FieldReader<u8, u8>);
impl DAC_CURR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_CURR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CURR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CURR` writer - Change the current in the DAC."]
pub struct DAC_CURR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CURR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DAC_DC` reader - Adjusts the DC level to the TX mixer."]
pub struct DAC_DC_R(crate::FieldReader<u8, u8>);
impl DAC_DC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_DC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_DC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_DC` writer - Adjusts the DC level to the TX mixer."]
pub struct DAC_DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `TXMIX_CURRENT` reader - Transmit mixers core current Current increases with increasing setting."]
pub struct TXMIX_CURRENT_R(crate::FieldReader<u8, u8>);
impl TXMIX_CURRENT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXMIX_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIX_CURRENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIX_CURRENT` writer - Transmit mixers core current Current increases with increasing setting."]
pub struct TXMIX_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIX_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&self) -> DAC_CURR_R {
        DAC_CURR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&self) -> DAC_DC_R {
        DAC_DC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&self) -> TXMIX_CURRENT_R {
        TXMIX_CURRENT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&mut self) -> DAC_CURR_W {
        DAC_CURR_W { w: self }
    }
    #[doc = "Bits 2:3 - Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&mut self) -> DAC_DC_W {
        DAC_DC_W { w: self }
    }
    #[doc = "Bits 0:1 - Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&mut self) -> TXMIX_CURRENT_W {
        TXMIX_CURRENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the TX settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctrl](index.html) module"]
pub struct TXCTRL_SPEC;
impl crate::RegisterSpec for TXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctrl::R](R) reader structure"]
impl crate::Readable for TXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctrl::W](W) writer structure"]
impl crate::Writable for TXCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TXCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
