#[doc = "Register `RCGCGPT` reader"]
pub type R = crate::R<RcgcgptSpec>;
#[doc = "Register `RCGCGPT` writer"]
pub type W = crate::W<RcgcgptSpec>;
#[doc = "Field `GPT0` reader - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
pub type Gpt0R = crate::BitReader;
#[doc = "Field `GPT0` writer - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
pub type Gpt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPT1` reader - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
pub type Gpt1R = crate::BitReader;
#[doc = "Field `GPT1` writer - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
pub type Gpt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPT2` reader - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
pub type Gpt2R = crate::BitReader;
#[doc = "Field `GPT2` writer - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
pub type Gpt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPT3` reader - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
pub type Gpt3R = crate::BitReader;
#[doc = "Field `GPT3` writer - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
pub type Gpt3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
    #[inline(always)]
    pub fn gpt0(&self) -> Gpt0R {
        Gpt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    pub fn gpt1(&self) -> Gpt1R {
        Gpt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    pub fn gpt2(&self) -> Gpt2R {
        Gpt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    pub fn gpt3(&self) -> Gpt3R {
        Gpt3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt0(&mut self) -> Gpt0W<RcgcgptSpec> {
        Gpt0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt1(&mut self) -> Gpt1W<RcgcgptSpec> {
        Gpt1W::new(self, 1)
    }
    #[doc = "Bit 2 - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt2(&mut self) -> Gpt2W<RcgcgptSpec> {
        Gpt2W::new(self, 2)
    }
    #[doc = "Bit 3 - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt3(&mut self) -> Gpt3W<RcgcgptSpec> {
        Gpt3W::new(self, 3)
    }
}
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcgpt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcgpt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcgcgptSpec;
impl crate::RegisterSpec for RcgcgptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgcgpt::R`](R) reader structure"]
impl crate::Readable for RcgcgptSpec {}
#[doc = "`write(|w| ..)` method takes [`rcgcgpt::W`](W) writer structure"]
impl crate::Writable for RcgcgptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCGCGPT to value 0"]
impl crate::Resettable for RcgcgptSpec {
    const RESET_VALUE: u32 = 0;
}
