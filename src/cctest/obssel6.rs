#[doc = "Register `OBSSEL6` reader"]
pub type R = crate::R<OBSSEL6_SPEC>;
#[doc = "Register `OBSSEL6` writer"]
pub type W = crate::W<OBSSEL6_SPEC>;
#[doc = "Field `SEL` reader - n - obs_sigs\\[n\\]
output on output 6: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - n - obs_sigs\\[n\\]
output on output 6: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `EN` reader - Observation output 6 enable control for PC6 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC6."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Observation output 6 enable control for PC6 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC6."]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 6: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Observation output 6 enable control for PC6 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC6."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 6: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<OBSSEL6_SPEC, 0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 7 - Observation output 6 enable control for PC6 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC6."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<OBSSEL6_SPEC, 7> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Select output signal on observation output 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBSSEL6_SPEC;
impl crate::RegisterSpec for OBSSEL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obssel6::R`](R) reader structure"]
impl crate::Readable for OBSSEL6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`obssel6::W`](W) writer structure"]
impl crate::Writable for OBSSEL6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OBSSEL6 to value 0"]
impl crate::Resettable for OBSSEL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
