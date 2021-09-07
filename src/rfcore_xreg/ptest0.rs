#[doc = "Register `PTEST0` reader"]
pub struct R(crate::R<PTEST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTEST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTEST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTEST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTEST0` writer"]
pub struct W(crate::W<PTEST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTEST0_SPEC>;
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
impl From<crate::W<PTEST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTEST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_PD` reader - Prescaler power-down signal When PD_OVERRIDE = 1"]
pub struct PRE_PD_R(crate::FieldReader<bool, bool>);
impl PRE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHP_PD` reader - Charge pump power-down signal When PD_OVERRIDE = 1"]
pub struct CHP_PD_R(crate::FieldReader<bool, bool>);
impl CHP_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHP_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHP_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHP_PD` writer - Charge pump power-down signal When PD_OVERRIDE = 1"]
pub struct CHP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ADC_PD` reader - ADC power-down signal When PD_OVERRIDE = 1"]
pub struct ADC_PD_R(crate::FieldReader<bool, bool>);
impl ADC_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_PD` writer - ADC power-down signal When PD_OVERRIDE = 1"]
pub struct ADC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DAC_PD` reader - DAC power-down signal When PD_OVERRIDE = 1"]
pub struct DAC_PD_R(crate::FieldReader<bool, bool>);
impl DAC_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_PD` writer - DAC power-down signal When PD_OVERRIDE = 1"]
pub struct DAC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `LNA_PD` reader - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
pub struct LNA_PD_R(crate::FieldReader<u8, u8>);
impl LNA_PD_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_PD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_PD` writer - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
pub struct LNA_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `TXMIX_PD` reader - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
pub struct TXMIX_PD_R(crate::FieldReader<bool, bool>);
impl TXMIX_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMIX_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIX_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIX_PD` writer - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
pub struct TXMIX_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIX_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `AAF_PD` reader - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
pub struct AAF_PD_R(crate::FieldReader<bool, bool>);
impl AAF_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        AAF_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAF_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAF_PD` writer - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
pub struct AAF_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AAF_PD_W<'a> {
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
    #[doc = "Bit 7 - Prescaler power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pre_pd(&self) -> PRE_PD_R {
        PRE_PD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Charge pump power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn chp_pd(&self) -> CHP_PD_R {
        CHP_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn adc_pd(&self) -> ADC_PD_R {
        ADC_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DAC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn dac_pd(&self) -> DAC_PD_R {
        DAC_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lna_pd(&self) -> LNA_PD_R {
        LNA_PD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn txmix_pd(&self) -> TXMIX_PD_R {
        TXMIX_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn aaf_pd(&self) -> AAF_PD_R {
        AAF_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Charge pump power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn chp_pd(&mut self) -> CHP_PD_W {
        CHP_PD_W { w: self }
    }
    #[doc = "Bit 5 - ADC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn adc_pd(&mut self) -> ADC_PD_W {
        ADC_PD_W { w: self }
    }
    #[doc = "Bit 4 - DAC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn dac_pd(&mut self) -> DAC_PD_W {
        DAC_PD_W { w: self }
    }
    #[doc = "Bits 2:3 - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lna_pd(&mut self) -> LNA_PD_W {
        LNA_PD_W { w: self }
    }
    #[doc = "Bit 1 - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn txmix_pd(&mut self) -> TXMIX_PD_W {
        TXMIX_PD_W { w: self }
    }
    #[doc = "Bit 0 - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn aaf_pd(&mut self) -> AAF_PD_W {
        AAF_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Override power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptest0](index.html) module"]
pub struct PTEST0_SPEC;
impl crate::RegisterSpec for PTEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptest0::R](R) reader structure"]
impl crate::Readable for PTEST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptest0::W](W) writer structure"]
impl crate::Writable for PTEST0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTEST0 to value 0"]
impl crate::Resettable for PTEST0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
