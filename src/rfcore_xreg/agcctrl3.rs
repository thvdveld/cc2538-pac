#[doc = "Register `AGCCTRL3` reader"]
pub struct R(crate::R<AGCCTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGCCTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGCCTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGCCTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGCCTRL3` writer"]
pub struct W(crate::W<AGCCTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGCCTRL3_SPEC>;
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
impl From<crate::W<AGCCTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGCCTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AAF_RP_OE` reader - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
pub type AAF_RP_OE_R = crate::BitReader<bool>;
#[doc = "Field `AAF_RP_OE` writer - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
pub type AAF_RP_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AGCCTRL3_SPEC, bool, O>;
#[doc = "Field `AAF_RP` reader - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
pub type AAF_RP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AAF_RP` writer - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
pub type AAF_RP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AGCCTRL3_SPEC, u8, u8, 2, O>;
#[doc = "Field `AGC_WIN_SIZE` reader - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
pub type AGC_WIN_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AGC_WIN_SIZE` writer - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
pub type AGC_WIN_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AGCCTRL3_SPEC, u8, u8, 2, O>;
#[doc = "Field `AGC_SETTLE_WAIT` reader - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
pub type AGC_SETTLE_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AGC_SETTLE_WAIT` writer - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
pub type AGC_SETTLE_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AGCCTRL3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
    #[inline(always)]
    pub fn aaf_rp_oe(&self) -> AAF_RP_OE_R {
        AAF_RP_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
    #[inline(always)]
    pub fn aaf_rp(&self) -> AAF_RP_R {
        AAF_RP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
    #[inline(always)]
    pub fn agc_win_size(&self) -> AGC_WIN_SIZE_R {
        AGC_WIN_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
    #[inline(always)]
    pub fn agc_settle_wait(&self) -> AGC_SETTLE_WAIT_R {
        AGC_SETTLE_WAIT_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
    #[inline(always)]
    pub fn aaf_rp_oe(&mut self) -> AAF_RP_OE_W<0> {
        AAF_RP_OE_W::new(self)
    }
    #[doc = "Bits 1:2 - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
    #[inline(always)]
    pub fn aaf_rp(&mut self) -> AAF_RP_W<1> {
        AAF_RP_W::new(self)
    }
    #[doc = "Bits 3:4 - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
    #[inline(always)]
    pub fn agc_win_size(&mut self) -> AGC_WIN_SIZE_W<3> {
        AGC_WIN_SIZE_W::new(self)
    }
    #[doc = "Bits 5:6 - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
    #[inline(always)]
    pub fn agc_settle_wait(&mut self) -> AGC_SETTLE_WAIT_W<5> {
        AGC_SETTLE_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agcctrl3](index.html) module"]
pub struct AGCCTRL3_SPEC;
impl crate::RegisterSpec for AGCCTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [agcctrl3::R](R) reader structure"]
impl crate::Readable for AGCCTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agcctrl3::W](W) writer structure"]
impl crate::Writable for AGCCTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AGCCTRL3 to value 0"]
impl crate::Resettable for AGCCTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
