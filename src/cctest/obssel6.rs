#[doc = "Register `OBSSEL6` reader"]
pub struct R(crate::R<OBSSEL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBSSEL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBSSEL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBSSEL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OBSSEL6` writer"]
pub struct W(crate::W<OBSSEL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OBSSEL6_SPEC>;
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
impl From<crate::W<OBSSEL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OBSSEL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Observation output 6 enable control for PC6 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC6."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Observation output 6 enable control for PC6 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC6."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OBSSEL6_SPEC, bool, O>;
#[doc = "Field `SEL` reader - n - obs_sigs\\[n\\]
output on output 6: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL` writer - n - obs_sigs\\[n\\]
output on output 6: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OBSSEL6_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 7 - Observation output 6 enable control for PC6 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC6."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 6: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Observation output 6 enable control for PC6 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC6."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<7> {
        EN_W::new(self)
    }
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 6: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select output signal on observation output 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obssel6](index.html) module"]
pub struct OBSSEL6_SPEC;
impl crate::RegisterSpec for OBSSEL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obssel6::R](R) reader structure"]
impl crate::Readable for OBSSEL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [obssel6::W](W) writer structure"]
impl crate::Writable for OBSSEL6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OBSSEL6 to value 0"]
impl crate::Resettable for OBSSEL6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
