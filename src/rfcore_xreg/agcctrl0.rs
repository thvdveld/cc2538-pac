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
#[doc = "Field `AGC_DR_XTND_THR` reader - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
pub type AGC_DR_XTND_THR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AGC_DR_XTND_THR` writer - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
pub type AGC_DR_XTND_THR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AGCCTRL0_SPEC, u8, u8, 6, O>;
#[doc = "Field `AGC_DR_XTND_EN` reader - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
pub type AGC_DR_XTND_EN_R = crate::BitReader<bool>;
#[doc = "Field `AGC_DR_XTND_EN` writer - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
pub type AGC_DR_XTND_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AGCCTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
    #[inline(always)]
    pub fn agc_dr_xtnd_thr(&self) -> AGC_DR_XTND_THR_R {
        AGC_DR_XTND_THR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
    #[inline(always)]
    pub fn agc_dr_xtnd_en(&self) -> AGC_DR_XTND_EN_R {
        AGC_DR_XTND_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
    #[inline(always)]
    #[must_use]
    pub fn agc_dr_xtnd_thr(&mut self) -> AGC_DR_XTND_THR_W<0> {
        AGC_DR_XTND_THR_W::new(self)
    }
    #[doc = "Bit 6 - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
    #[inline(always)]
    #[must_use]
    pub fn agc_dr_xtnd_en(&mut self) -> AGC_DR_XTND_EN_W<6> {
        AGC_DR_XTND_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGCCTRL0 to value 0"]
impl crate::Resettable for AGCCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
