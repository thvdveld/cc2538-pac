#[doc = "Register `AGCCTRL3` reader"]
pub type R = crate::R<Agcctrl3Spec>;
#[doc = "Register `AGCCTRL3` writer"]
pub type W = crate::W<Agcctrl3Spec>;
#[doc = "Field `AAF_RP_OE` reader - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
pub type AafRpOeR = crate::BitReader;
#[doc = "Field `AAF_RP_OE` writer - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
pub type AafRpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAF_RP` reader - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
pub type AafRpR = crate::FieldReader;
#[doc = "Field `AAF_RP` writer - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
pub type AafRpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AGC_WIN_SIZE` reader - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
pub type AgcWinSizeR = crate::FieldReader;
#[doc = "Field `AGC_WIN_SIZE` writer - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
pub type AgcWinSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AGC_SETTLE_WAIT` reader - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
pub type AgcSettleWaitR = crate::FieldReader;
#[doc = "Field `AGC_SETTLE_WAIT` writer - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
pub type AgcSettleWaitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
    #[inline(always)]
    pub fn aaf_rp_oe(&self) -> AafRpOeR {
        AafRpOeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
    #[inline(always)]
    pub fn aaf_rp(&self) -> AafRpR {
        AafRpR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
    #[inline(always)]
    pub fn agc_win_size(&self) -> AgcWinSizeR {
        AgcWinSizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
    #[inline(always)]
    pub fn agc_settle_wait(&self) -> AgcSettleWaitR {
        AgcSettleWaitR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Override the AAF control signals of the AGC with the values stored in AAF_RP."]
    #[inline(always)]
    pub fn aaf_rp_oe(&mut self) -> AafRpOeW<Agcctrl3Spec> {
        AafRpOeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
    #[inline(always)]
    pub fn aaf_rp(&mut self) -> AafRpW<Agcctrl3Spec> {
        AafRpW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
    #[inline(always)]
    pub fn agc_win_size(&mut self) -> AgcWinSizeW<Agcctrl3Spec> {
        AgcWinSizeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
    #[inline(always)]
    pub fn agc_settle_wait(&mut self) -> AgcSettleWaitW<Agcctrl3Spec> {
        AgcSettleWaitW::new(self, 5)
    }
}
#[doc = "AGC control\n\nYou can [`read`](crate::Reg::read) this register and get [`agcctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agcctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agcctrl3Spec;
impl crate::RegisterSpec for Agcctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agcctrl3::R`](R) reader structure"]
impl crate::Readable for Agcctrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`agcctrl3::W`](W) writer structure"]
impl crate::Writable for Agcctrl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGCCTRL3 to value 0"]
impl crate::Resettable for Agcctrl3Spec {
    const RESET_VALUE: u32 = 0;
}
