#[doc = "Register `TXPOWER` reader"]
pub type R = crate::R<TXPOWER_SPEC>;
#[doc = "Register `TXPOWER` writer"]
pub type W = crate::W<TXPOWER_SPEC>;
#[doc = "Field `PA_BIAS` reader - PA bias control"]
pub type PA_BIAS_R = crate::FieldReader;
#[doc = "Field `PA_BIAS` writer - PA bias control"]
pub type PA_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PA_POWER` reader - PA power control"]
pub type PA_POWER_R = crate::FieldReader;
#[doc = "Field `PA_POWER` writer - PA power control"]
pub type PA_POWER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PA bias control"]
    #[inline(always)]
    pub fn pa_bias(&self) -> PA_BIAS_R {
        PA_BIAS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PA power control"]
    #[inline(always)]
    pub fn pa_power(&self) -> PA_POWER_R {
        PA_POWER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PA bias control"]
    #[inline(always)]
    #[must_use]
    pub fn pa_bias(&mut self) -> PA_BIAS_W<TXPOWER_SPEC> {
        PA_BIAS_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PA power control"]
    #[inline(always)]
    #[must_use]
    pub fn pa_power(&mut self) -> PA_POWER_W<TXPOWER_SPEC> {
        PA_POWER_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Controls the output power\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPOWER_SPEC;
impl crate::RegisterSpec for TXPOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpower::R`](R) reader structure"]
impl crate::Readable for TXPOWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txpower::W`](W) writer structure"]
impl crate::Writable for TXPOWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXPOWER to value 0"]
impl crate::Resettable for TXPOWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
