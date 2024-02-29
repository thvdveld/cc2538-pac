#[doc = "Register `OBSSEL1` reader"]
pub type R = crate::R<Obssel1Spec>;
#[doc = "Register `OBSSEL1` writer"]
pub type W = crate::W<Obssel1Spec>;
#[doc = "Field `SEL` reader - n - obs_sigs\\[n\\]
output on output 1: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - n - obs_sigs\\[n\\]
output on output 1: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - Observation output 1 enable control for PC1 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC1."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Observation output 1 enable control for PC1 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC1."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 1: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Observation output 1 enable control for PC1 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC1."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 1: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Obssel1Spec> {
        SelW::new(self, 0)
    }
    #[doc = "Bit 7 - Observation output 1 enable control for PC1 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC1."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Obssel1Spec> {
        EnW::new(self, 7)
    }
}
#[doc = "Select output signal on observation output 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obssel1Spec;
impl crate::RegisterSpec for Obssel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obssel1::R`](R) reader structure"]
impl crate::Readable for Obssel1Spec {}
#[doc = "`write(|w| ..)` method takes [`obssel1::W`](W) writer structure"]
impl crate::Writable for Obssel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OBSSEL1 to value 0"]
impl crate::Resettable for Obssel1Spec {
    const RESET_VALUE: u32 = 0;
}
