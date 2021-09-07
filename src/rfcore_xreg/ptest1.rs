#[doc = "Register `PTEST1` reader"]
pub struct R(crate::R<PTEST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTEST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTEST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTEST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTEST1` writer"]
pub struct W(crate::W<PTEST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTEST1_SPEC>;
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
impl From<crate::W<PTEST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTEST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD_OVERRIDE` reader - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
pub struct PD_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl PD_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_OVERRIDE` writer - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
pub struct PD_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_OVERRIDE_W<'a> {
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
#[doc = "Field `PA_PD` reader - Power amplifier power-down signal When PD_OVERRIDE = 1"]
pub struct PA_PD_R(crate::FieldReader<bool, bool>);
impl PA_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_PD` writer - Power amplifier power-down signal When PD_OVERRIDE = 1"]
pub struct PA_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PD_W<'a> {
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
#[doc = "Field `VCO_PD` reader - VCO power-down signal When PD_OVERRIDE = 1"]
pub struct VCO_PD_R(crate::FieldReader<bool, bool>);
impl VCO_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCO_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_PD` writer - VCO power-down signal When PD_OVERRIDE = 1"]
pub struct VCO_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_PD_W<'a> {
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
#[doc = "Field `LODIV_PD` reader - LO power-down signal When PD_OVERRIDE = 1"]
pub struct LODIV_PD_R(crate::FieldReader<bool, bool>);
impl LODIV_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LODIV_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIV_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LODIV_PD` writer - LO power-down signal When PD_OVERRIDE = 1"]
pub struct LODIV_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_PD_W<'a> {
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
    #[doc = "Bit 3 - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
    #[inline(always)]
    pub fn pd_override(&self) -> PD_OVERRIDE_R {
        PD_OVERRIDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pa_pd(&self) -> PA_PD_R {
        PA_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn vco_pd(&self) -> VCO_PD_R {
        VCO_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lodiv_pd(&self) -> LODIV_PD_R {
        LODIV_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
    #[inline(always)]
    pub fn pd_override(&mut self) -> PD_OVERRIDE_W {
        PD_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 2 - Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pa_pd(&mut self) -> PA_PD_W {
        PA_PD_W { w: self }
    }
    #[doc = "Bit 1 - VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn vco_pd(&mut self) -> VCO_PD_W {
        VCO_PD_W { w: self }
    }
    #[doc = "Bit 0 - LO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lodiv_pd(&mut self) -> LODIV_PD_W {
        LODIV_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Override power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptest1](index.html) module"]
pub struct PTEST1_SPEC;
impl crate::RegisterSpec for PTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptest1::R](R) reader structure"]
impl crate::Readable for PTEST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptest1::W](W) writer structure"]
impl crate::Writable for PTEST1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTEST1 to value 0"]
impl crate::Resettable for PTEST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
