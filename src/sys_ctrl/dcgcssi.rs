#[doc = "Register `DCGCSSI` reader"]
pub type R = crate::R<DcgcssiSpec>;
#[doc = "Register `DCGCSSI` writer"]
pub type W = crate::W<DcgcssiSpec>;
#[doc = "Field `SSI0` reader - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
pub type Ssi0R = crate::BitReader;
#[doc = "Field `SSI0` writer - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
pub type Ssi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI1` reader - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
pub type Ssi1R = crate::BitReader;
#[doc = "Field `SSI1` writer - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
pub type Ssi1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
    #[inline(always)]
    pub fn ssi0(&self) -> Ssi0R {
        Ssi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
    #[inline(always)]
    pub fn ssi1(&self) -> Ssi1R {
        Ssi1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ssi0(&mut self) -> Ssi0W<DcgcssiSpec> {
        Ssi0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ssi1(&mut self) -> Ssi1W<DcgcssiSpec> {
        Ssi1W::new(self, 1)
    }
}
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcgcssiSpec;
impl crate::RegisterSpec for DcgcssiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcssi::R`](R) reader structure"]
impl crate::Readable for DcgcssiSpec {}
#[doc = "`write(|w| ..)` method takes [`dcgcssi::W`](W) writer structure"]
impl crate::Writable for DcgcssiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCGCSSI to value 0"]
impl crate::Resettable for DcgcssiSpec {
    const RESET_VALUE: u32 = 0;
}
