#[doc = "Register `IVCTRL` reader"]
pub struct R(crate::R<IVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IVCTRL` writer"]
pub struct W(crate::W<IVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVCTRL_SPEC>;
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
impl From<crate::W<IVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_CURR_CTRL` reader - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
pub struct DAC_CURR_CTRL_R(crate::FieldReader<u8, u8>);
impl DAC_CURR_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_CURR_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CURR_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CURR_CTRL` writer - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
pub struct DAC_CURR_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CURR_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `LODIV_BIAS_CTRL` reader - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
pub struct LODIV_BIAS_CTRL_R(crate::FieldReader<bool, bool>);
impl LODIV_BIAS_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LODIV_BIAS_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIV_BIAS_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LODIV_BIAS_CTRL` writer - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
pub struct LODIV_BIAS_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_BIAS_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TXMIX_DC_CTRL` reader - Controls DC bias in TXMIX"]
pub struct TXMIX_DC_CTRL_R(crate::FieldReader<bool, bool>);
impl TXMIX_DC_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMIX_DC_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIX_DC_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIX_DC_CTRL` writer - Controls DC bias in TXMIX"]
pub struct TXMIX_DC_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIX_DC_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PA_BIAS_CTRL` reader - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
pub struct PA_BIAS_CTRL_R(crate::FieldReader<u8, u8>);
impl PA_BIAS_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_BIAS_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_BIAS_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_BIAS_CTRL` writer - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
pub struct PA_BIAS_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_BIAS_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&self) -> DAC_CURR_CTRL_R {
        DAC_CURR_CTRL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&self) -> LODIV_BIAS_CTRL_R {
        LODIV_BIAS_CTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&self) -> TXMIX_DC_CTRL_R {
        TXMIX_DC_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&self) -> PA_BIAS_CTRL_R {
        PA_BIAS_CTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&mut self) -> DAC_CURR_CTRL_W {
        DAC_CURR_CTRL_W { w: self }
    }
    #[doc = "Bit 3 - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&mut self) -> LODIV_BIAS_CTRL_W {
        LODIV_BIAS_CTRL_W { w: self }
    }
    #[doc = "Bit 2 - Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&mut self) -> TXMIX_DC_CTRL_W {
        TXMIX_DC_CTRL_W { w: self }
    }
    #[doc = "Bits 0:1 - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&mut self) -> PA_BIAS_CTRL_W {
        PA_BIAS_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivctrl](index.html) module"]
pub struct IVCTRL_SPEC;
impl crate::RegisterSpec for IVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivctrl::R](R) reader structure"]
impl crate::Readable for IVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ivctrl::W](W) writer structure"]
impl crate::Writable for IVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVCTRL to value 0"]
impl crate::Resettable for IVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
