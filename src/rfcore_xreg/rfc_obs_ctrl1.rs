#[doc = "Register `RFC_OBS_CTRL1` reader"]
pub struct R(crate::R<RFC_OBS_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFC_OBS_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFC_OBS_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFC_OBS_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFC_OBS_CTRL1` writer"]
pub struct W(crate::W<RFC_OBS_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFC_OBS_CTRL1_SPEC>;
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
impl From<crate::W<RFC_OBS_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFC_OBS_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC_OBS_MUX1` reader - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[1\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RFC_OBS_MUX1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFC_OBS_MUX1` writer - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[1\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RFC_OBS_MUX1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFC_OBS_CTRL1_SPEC, u8, u8, 6, O>;
#[doc = "Field `RFC_OBS_POL1` reader - The signal chosen by RFC_OBS_MUX1 is XORed with this bit."]
pub type RFC_OBS_POL1_R = crate::BitReader<bool>;
#[doc = "Field `RFC_OBS_POL1` writer - The signal chosen by RFC_OBS_MUX1 is XORed with this bit."]
pub type RFC_OBS_POL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFC_OBS_CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[1\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    pub fn rfc_obs_mux1(&self) -> RFC_OBS_MUX1_R {
        RFC_OBS_MUX1_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX1 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol1(&self) -> RFC_OBS_POL1_R {
        RFC_OBS_POL1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[1\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_obs_mux1(&mut self) -> RFC_OBS_MUX1_W<0> {
        RFC_OBS_MUX1_W::new(self)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX1 is XORed with this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_obs_pol1(&mut self) -> RFC_OBS_POL1_W<6> {
        RFC_OBS_POL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF observation mux control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfc_obs_ctrl1](index.html) module"]
pub struct RFC_OBS_CTRL1_SPEC;
impl crate::RegisterSpec for RFC_OBS_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfc_obs_ctrl1::R](R) reader structure"]
impl crate::Readable for RFC_OBS_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfc_obs_ctrl1::W](W) writer structure"]
impl crate::Writable for RFC_OBS_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFC_OBS_CTRL1 to value 0"]
impl crate::Resettable for RFC_OBS_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
