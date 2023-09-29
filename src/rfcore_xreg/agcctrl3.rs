#[doc = "Register `AGCCTRL3` reader"]
pub type R = crate::R<AGCCTRL3_SPEC>;
#[doc = "Register `AGCCTRL3` writer"]
pub type W = crate::W<AGCCTRL3_SPEC>;
#[doc = "Field `AAF_RP_OE` reader - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
pub type AAF_RP_OE_R = crate::BitReader;
#[doc = "Field `AAF_RP_OE` writer - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
pub type AAF_RP_OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AAF_RP` reader - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
pub type AAF_RP_R = crate::FieldReader;
#[doc = "Field `AAF_RP` writer - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
pub type AAF_RP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `AGC_WIN_SIZE` reader - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
pub type AGC_WIN_SIZE_R = crate::FieldReader;
#[doc = "Field `AGC_WIN_SIZE` writer - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
pub type AGC_WIN_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `AGC_SETTLE_WAIT` reader - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
pub type AGC_SETTLE_WAIT_R = crate::FieldReader;
#[doc = "Field `AGC_SETTLE_WAIT` writer - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
pub type AGC_SETTLE_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
    #[must_use]
    pub fn aaf_rp_oe(&mut self) -> AAF_RP_OE_W<AGCCTRL3_SPEC, 0> {
        AAF_RP_OE_W::new(self)
    }
    #[doc = "Bits 1:2 - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
    #[inline(always)]
    #[must_use]
    pub fn aaf_rp(&mut self) -> AAF_RP_W<AGCCTRL3_SPEC, 1> {
        AAF_RP_W::new(self)
    }
    #[doc = "Bits 3:4 - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
    #[inline(always)]
    #[must_use]
    pub fn agc_win_size(&mut self) -> AGC_WIN_SIZE_W<AGCCTRL3_SPEC, 3> {
        AGC_WIN_SIZE_W::new(self)
    }
    #[doc = "Bits 5:6 - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
    #[inline(always)]
    #[must_use]
    pub fn agc_settle_wait(&mut self) -> AGC_SETTLE_WAIT_W<AGCCTRL3_SPEC, 5> {
        AGC_SETTLE_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AGC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGCCTRL3_SPEC;
impl crate::RegisterSpec for AGCCTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agcctrl3::R`](R) reader structure"]
impl crate::Readable for AGCCTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agcctrl3::W`](W) writer structure"]
impl crate::Writable for AGCCTRL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGCCTRL3 to value 0"]
impl crate::Resettable for AGCCTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
