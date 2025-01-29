#[doc = "Register `OBSSEL2` reader"]
pub type R = crate::R<Obssel2Spec>;
#[doc = "Register `OBSSEL2` writer"]
pub type W = crate::W<Obssel2Spec>;
#[doc = "Field `SEL` reader - n - obs_sigs\\[n\\]
output on output 2: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - n - obs_sigs\\[n\\]
output on output 2: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - Observation output 2 enable control for PC2 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC2."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Observation output 2 enable control for PC2 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC2."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 2: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Observation output 2 enable control for PC2 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC2."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 2: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<Obssel2Spec> {
        SelW::new(self, 0)
    }
    #[doc = "Bit 7 - Observation output 2 enable control for PC2 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC2."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Obssel2Spec> {
        EnW::new(self, 7)
    }
}
#[doc = "Select output signal on observation output 2\n\nYou can [`read`](crate::Reg::read) this register and get [`obssel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obssel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obssel2Spec;
impl crate::RegisterSpec for Obssel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obssel2::R`](R) reader structure"]
impl crate::Readable for Obssel2Spec {}
#[doc = "`write(|w| ..)` method takes [`obssel2::W`](W) writer structure"]
impl crate::Writable for Obssel2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OBSSEL2 to value 0"]
impl crate::Resettable for Obssel2Spec {
    const RESET_VALUE: u32 = 0;
}
