#[doc = "Register `AGCCTRL0` reader"]
pub struct R(crate::R<AGCCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGCCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGCCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGCCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGCCTRL0` writer"]
pub struct W(crate::W<AGCCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGCCTRL0_SPEC>;
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
impl From<crate::W<AGCCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGCCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGC_DR_XTND_EN` reader - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
pub struct AGC_DR_XTND_EN_R(crate::FieldReader<bool, bool>);
impl AGC_DR_XTND_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AGC_DR_XTND_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AGC_DR_XTND_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AGC_DR_XTND_EN` writer - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
pub struct AGC_DR_XTND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_DR_XTND_EN_W<'a> {
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
#[doc = "Field `AGC_DR_XTND_THR` reader - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
pub struct AGC_DR_XTND_THR_R(crate::FieldReader<u8, u8>);
impl AGC_DR_XTND_THR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AGC_DR_XTND_THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AGC_DR_XTND_THR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AGC_DR_XTND_THR` writer - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
pub struct AGC_DR_XTND_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_DR_XTND_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
    #[inline(always)]
    pub fn agc_dr_xtnd_en(&self) -> AGC_DR_XTND_EN_R {
        AGC_DR_XTND_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
    #[inline(always)]
    pub fn agc_dr_xtnd_thr(&self) -> AGC_DR_XTND_THR_R {
        AGC_DR_XTND_THR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
    #[inline(always)]
    pub fn agc_dr_xtnd_en(&mut self) -> AGC_DR_XTND_EN_W {
        AGC_DR_XTND_EN_W { w: self }
    }
    #[doc = "Bits 0:5 - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
    #[inline(always)]
    pub fn agc_dr_xtnd_thr(&mut self) -> AGC_DR_XTND_THR_W {
        AGC_DR_XTND_THR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGC dynamic range control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agcctrl0](index.html) module"]
pub struct AGCCTRL0_SPEC;
impl crate::RegisterSpec for AGCCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [agcctrl0::R](R) reader structure"]
impl crate::Readable for AGCCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agcctrl0::W](W) writer structure"]
impl crate::Writable for AGCCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AGCCTRL0 to value 0"]
impl crate::Resettable for AGCCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
