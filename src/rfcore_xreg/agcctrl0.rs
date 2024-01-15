#[doc = "Register `AGCCTRL0` reader"]
pub type R = crate::R<AGCCTRL0_SPEC>;
#[doc = "Register `AGCCTRL0` writer"]
pub type W = crate::W<AGCCTRL0_SPEC>;
#[doc = "Field `AGC_DR_XTND_THR` reader - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
pub type AGC_DR_XTND_THR_R = crate::FieldReader;
#[doc = "Field `AGC_DR_XTND_THR` writer - If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
pub type AGC_DR_XTND_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AGC_DR_XTND_EN` reader - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
pub type AGC_DR_XTND_EN_R = crate::BitReader;
#[doc = "Field `AGC_DR_XTND_EN` writer - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
pub type AGC_DR_XTND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn agc_dr_xtnd_thr(&mut self) -> AGC_DR_XTND_THR_W<AGCCTRL0_SPEC> {
        AGC_DR_XTND_THR_W::new(self, 0)
    }
    #[doc = "Bit 6 - 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
    #[inline(always)]
    #[must_use]
    pub fn agc_dr_xtnd_en(&mut self) -> AGC_DR_XTND_EN_W<AGCCTRL0_SPEC> {
        AGC_DR_XTND_EN_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AGC dynamic range control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGCCTRL0_SPEC;
impl crate::RegisterSpec for AGCCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agcctrl0::R`](R) reader structure"]
impl crate::Readable for AGCCTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agcctrl0::W`](W) writer structure"]
impl crate::Writable for AGCCTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGCCTRL0 to value 0"]
impl crate::Resettable for AGCCTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
