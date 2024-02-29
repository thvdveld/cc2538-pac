#[doc = "Register `TXPOWER` reader"]
pub type R = crate::R<TxpowerSpec>;
#[doc = "Register `TXPOWER` writer"]
pub type W = crate::W<TxpowerSpec>;
#[doc = "Field `PA_BIAS` reader - PA bias control"]
pub type PaBiasR = crate::FieldReader;
#[doc = "Field `PA_BIAS` writer - PA bias control"]
pub type PaBiasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PA_POWER` reader - PA power control"]
pub type PaPowerR = crate::FieldReader;
#[doc = "Field `PA_POWER` writer - PA power control"]
pub type PaPowerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PA bias control"]
    #[inline(always)]
    pub fn pa_bias(&self) -> PaBiasR {
        PaBiasR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PA power control"]
    #[inline(always)]
    pub fn pa_power(&self) -> PaPowerR {
        PaPowerR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PA bias control"]
    #[inline(always)]
    #[must_use]
    pub fn pa_bias(&mut self) -> PaBiasW<TxpowerSpec> {
        PaBiasW::new(self, 0)
    }
    #[doc = "Bits 4:7 - PA power control"]
    #[inline(always)]
    #[must_use]
    pub fn pa_power(&mut self) -> PaPowerW<TxpowerSpec> {
        PaPowerW::new(self, 4)
    }
}
#[doc = "Controls the output power\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxpowerSpec;
impl crate::RegisterSpec for TxpowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpower::R`](R) reader structure"]
impl crate::Readable for TxpowerSpec {}
#[doc = "`write(|w| ..)` method takes [`txpower::W`](W) writer structure"]
impl crate::Writable for TxpowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXPOWER to value 0"]
impl crate::Resettable for TxpowerSpec {
    const RESET_VALUE: u32 = 0;
}
