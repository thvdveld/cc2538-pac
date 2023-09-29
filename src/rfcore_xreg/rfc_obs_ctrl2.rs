#[doc = "Register `RFC_OBS_CTRL2` reader"]
pub type R = crate::R<RFC_OBS_CTRL2_SPEC>;
#[doc = "Register `RFC_OBS_CTRL2` writer"]
pub type W = crate::W<RFC_OBS_CTRL2_SPEC>;
#[doc = "Field `RFC_OBS_MUX2` reader - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RFC_OBS_MUX2_R = crate::FieldReader;
#[doc = "Field `RFC_OBS_MUX2` writer - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RFC_OBS_MUX2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `RFC_OBS_POL2` reader - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
pub type RFC_OBS_POL2_R = crate::BitReader;
#[doc = "Field `RFC_OBS_POL2` writer - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
pub type RFC_OBS_POL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    pub fn rfc_obs_mux2(&self) -> RFC_OBS_MUX2_R {
        RFC_OBS_MUX2_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol2(&self) -> RFC_OBS_POL2_R {
        RFC_OBS_POL2_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_obs_mux2(&mut self) -> RFC_OBS_MUX2_W<RFC_OBS_CTRL2_SPEC, 0> {
        RFC_OBS_MUX2_W::new(self)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_obs_pol2(&mut self) -> RFC_OBS_POL2_W<RFC_OBS_CTRL2_SPEC, 6> {
        RFC_OBS_POL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RF observation mux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc_obs_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfc_obs_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFC_OBS_CTRL2_SPEC;
impl crate::RegisterSpec for RFC_OBS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfc_obs_ctrl2::R`](R) reader structure"]
impl crate::Readable for RFC_OBS_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfc_obs_ctrl2::W`](W) writer structure"]
impl crate::Writable for RFC_OBS_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFC_OBS_CTRL2 to value 0"]
impl crate::Resettable for RFC_OBS_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
