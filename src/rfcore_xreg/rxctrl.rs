#[doc = "Register `RXCTRL` reader"]
pub type R = crate::R<RXCTRL_SPEC>;
#[doc = "Register `RXCTRL` writer"]
pub type W = crate::W<RXCTRL_SPEC>;
#[doc = "Field `MIX_CURRENT` reader - Control of the output current from the receiver mixers The current increases with increasing setting set."]
pub type MIX_CURRENT_R = crate::FieldReader;
#[doc = "Field `MIX_CURRENT` writer - Control of the output current from the receiver mixers The current increases with increasing setting set."]
pub type MIX_CURRENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `GBIAS_LNA_REF` reader - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub type GBIAS_LNA_REF_R = crate::FieldReader;
#[doc = "Field `GBIAS_LNA_REF` writer - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub type GBIAS_LNA_REF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `GBIAS_LNA2_REF` reader - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub type GBIAS_LNA2_REF_R = crate::FieldReader;
#[doc = "Field `GBIAS_LNA2_REF` writer - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub type GBIAS_LNA2_REF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Control of the output current from the receiver mixers The current increases with increasing setting set."]
    #[inline(always)]
    pub fn mix_current(&self) -> MIX_CURRENT_R {
        MIX_CURRENT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna_ref(&self) -> GBIAS_LNA_REF_R {
        GBIAS_LNA_REF_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna2_ref(&self) -> GBIAS_LNA2_REF_R {
        GBIAS_LNA2_REF_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control of the output current from the receiver mixers The current increases with increasing setting set."]
    #[inline(always)]
    #[must_use]
    pub fn mix_current(&mut self) -> MIX_CURRENT_W<RXCTRL_SPEC, 0> {
        MIX_CURRENT_W::new(self)
    }
    #[doc = "Bits 2:3 - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    #[must_use]
    pub fn gbias_lna_ref(&mut self) -> GBIAS_LNA_REF_W<RXCTRL_SPEC, 2> {
        GBIAS_LNA_REF_W::new(self)
    }
    #[doc = "Bits 4:5 - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    #[must_use]
    pub fn gbias_lna2_ref(&mut self) -> GBIAS_LNA2_REF_W<RXCTRL_SPEC, 4> {
        GBIAS_LNA2_REF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tune receive section\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRL_SPEC;
impl crate::RegisterSpec for RXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl::R`](R) reader structure"]
impl crate::Readable for RXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxctrl::W`](W) writer structure"]
impl crate::Writable for RXCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCTRL to value 0"]
impl crate::Resettable for RXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
