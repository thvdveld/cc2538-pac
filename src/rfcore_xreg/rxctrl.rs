#[doc = "Register `RXCTRL` reader"]
pub type R = crate::R<RxctrlSpec>;
#[doc = "Register `RXCTRL` writer"]
pub type W = crate::W<RxctrlSpec>;
#[doc = "Field `MIX_CURRENT` reader - Control of the output current from the receiver mixers The current increases with increasing setting set."]
pub type MixCurrentR = crate::FieldReader;
#[doc = "Field `MIX_CURRENT` writer - Control of the output current from the receiver mixers The current increases with increasing setting set."]
pub type MixCurrentW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GBIAS_LNA_REF` reader - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub type GbiasLnaRefR = crate::FieldReader;
#[doc = "Field `GBIAS_LNA_REF` writer - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub type GbiasLnaRefW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GBIAS_LNA2_REF` reader - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub type GbiasLna2RefR = crate::FieldReader;
#[doc = "Field `GBIAS_LNA2_REF` writer - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
pub type GbiasLna2RefW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Control of the output current from the receiver mixers The current increases with increasing setting set."]
    #[inline(always)]
    pub fn mix_current(&self) -> MixCurrentR {
        MixCurrentR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna_ref(&self) -> GbiasLnaRefR {
        GbiasLnaRefR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna2_ref(&self) -> GbiasLna2RefR {
        GbiasLna2RefR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control of the output current from the receiver mixers The current increases with increasing setting set."]
    #[inline(always)]
    pub fn mix_current(&mut self) -> MixCurrentW<RxctrlSpec> {
        MixCurrentW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Adjusts front-end LNA PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna_ref(&mut self) -> GbiasLnaRefW<RxctrlSpec> {
        GbiasLnaRefW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Adjusts front-end LNA2/mixer PTAT current output (from M = 3 to M = 6), default: M = 5"]
    #[inline(always)]
    pub fn gbias_lna2_ref(&mut self) -> GbiasLna2RefW<RxctrlSpec> {
        GbiasLna2RefW::new(self, 4)
    }
}
#[doc = "Tune receive section\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxctrlSpec;
impl crate::RegisterSpec for RxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl::R`](R) reader structure"]
impl crate::Readable for RxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rxctrl::W`](W) writer structure"]
impl crate::Writable for RxctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCTRL to value 0"]
impl crate::Resettable for RxctrlSpec {
    const RESET_VALUE: u32 = 0;
}
