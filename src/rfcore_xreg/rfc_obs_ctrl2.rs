#[doc = "Register `RFC_OBS_CTRL2` reader"]
pub type R = crate::R<RfcObsCtrl2Spec>;
#[doc = "Register `RFC_OBS_CTRL2` writer"]
pub type W = crate::W<RfcObsCtrl2Spec>;
#[doc = "Field `RFC_OBS_MUX2` reader - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RfcObsMux2R = crate::FieldReader;
#[doc = "Field `RFC_OBS_MUX2` writer - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RfcObsMux2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RFC_OBS_POL2` reader - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
pub type RfcObsPol2R = crate::BitReader;
#[doc = "Field `RFC_OBS_POL2` writer - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
pub type RfcObsPol2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    pub fn rfc_obs_mux2(&self) -> RfcObsMux2R {
        RfcObsMux2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol2(&self) -> RfcObsPol2R {
        RfcObsPol2R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    pub fn rfc_obs_mux2(&mut self) -> RfcObsMux2W<RfcObsCtrl2Spec> {
        RfcObsMux2W::new(self, 0)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol2(&mut self) -> RfcObsPol2W<RfcObsCtrl2Spec> {
        RfcObsPol2W::new(self, 6)
    }
}
#[doc = "RF observation mux control\n\nYou can [`read`](crate::Reg::read) this register and get [`rfc_obs_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfc_obs_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcObsCtrl2Spec;
impl crate::RegisterSpec for RfcObsCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfc_obs_ctrl2::R`](R) reader structure"]
impl crate::Readable for RfcObsCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`rfc_obs_ctrl2::W`](W) writer structure"]
impl crate::Writable for RfcObsCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFC_OBS_CTRL2 to value 0"]
impl crate::Resettable for RfcObsCtrl2Spec {
    const RESET_VALUE: u32 = 0;
}
