#[doc = "Register `RFC_OBS_CTRL1` reader"]
pub type R = crate::R<RfcObsCtrl1Spec>;
#[doc = "Register `RFC_OBS_CTRL1` writer"]
pub type W = crate::W<RfcObsCtrl1Spec>;
#[doc = "Field `RFC_OBS_MUX1` reader - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[1\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RfcObsMux1R = crate::FieldReader;
#[doc = "Field `RFC_OBS_MUX1` writer - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[1\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RfcObsMux1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RFC_OBS_POL1` reader - The signal chosen by RFC_OBS_MUX1 is XORed with this bit."]
pub type RfcObsPol1R = crate::BitReader;
#[doc = "Field `RFC_OBS_POL1` writer - The signal chosen by RFC_OBS_MUX1 is XORed with this bit."]
pub type RfcObsPol1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[1\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    pub fn rfc_obs_mux1(&self) -> RfcObsMux1R {
        RfcObsMux1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX1 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol1(&self) -> RfcObsPol1R {
        RfcObsPol1R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[1\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_obs_mux1(&mut self) -> RfcObsMux1W<RfcObsCtrl1Spec> {
        RfcObsMux1W::new(self, 0)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX1 is XORed with this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_obs_pol1(&mut self) -> RfcObsPol1W<RfcObsCtrl1Spec> {
        RfcObsPol1W::new(self, 6)
    }
}
#[doc = "RF observation mux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc_obs_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfc_obs_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcObsCtrl1Spec;
impl crate::RegisterSpec for RfcObsCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfc_obs_ctrl1::R`](R) reader structure"]
impl crate::Readable for RfcObsCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rfc_obs_ctrl1::W`](W) writer structure"]
impl crate::Writable for RfcObsCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFC_OBS_CTRL1 to value 0"]
impl crate::Resettable for RfcObsCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
