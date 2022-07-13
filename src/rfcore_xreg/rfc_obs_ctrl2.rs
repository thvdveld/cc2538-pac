#[doc = "Register `RFC_OBS_CTRL2` reader"]
pub struct R(crate::R<RFC_OBS_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFC_OBS_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFC_OBS_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFC_OBS_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFC_OBS_CTRL2` writer"]
pub struct W(crate::W<RFC_OBS_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFC_OBS_CTRL2_SPEC>;
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
impl From<crate::W<RFC_OBS_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFC_OBS_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC_OBS_POL2` reader - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
pub type RFC_OBS_POL2_R = crate::BitReader<bool>;
#[doc = "Field `RFC_OBS_POL2` writer - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
pub type RFC_OBS_POL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFC_OBS_CTRL2_SPEC, bool, O>;
#[doc = "Field `RFC_OBS_MUX2` reader - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RFC_OBS_MUX2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFC_OBS_MUX2` writer - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
pub type RFC_OBS_MUX2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFC_OBS_CTRL2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol2(&self) -> RFC_OBS_POL2_R {
        RFC_OBS_POL2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    pub fn rfc_obs_mux2(&self) -> RFC_OBS_MUX2_R {
        RFC_OBS_MUX2_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX2 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol2(&mut self) -> RFC_OBS_POL2_W<6> {
        RFC_OBS_POL2_W::new(self)
    }
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[2\\]. See description of RFC_OBS_CTRL0 for details."]
    #[inline(always)]
    pub fn rfc_obs_mux2(&mut self) -> RFC_OBS_MUX2_W<0> {
        RFC_OBS_MUX2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF observation mux control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfc_obs_ctrl2](index.html) module"]
pub struct RFC_OBS_CTRL2_SPEC;
impl crate::RegisterSpec for RFC_OBS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfc_obs_ctrl2::R](R) reader structure"]
impl crate::Readable for RFC_OBS_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfc_obs_ctrl2::W](W) writer structure"]
impl crate::Writable for RFC_OBS_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFC_OBS_CTRL2 to value 0"]
impl crate::Resettable for RFC_OBS_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
