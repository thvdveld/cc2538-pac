#[doc = "Register `SCGCGPT` reader"]
pub type R = crate::R<ScgcgptSpec>;
#[doc = "Register `SCGCGPT` writer"]
pub type W = crate::W<ScgcgptSpec>;
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
    pub fn gpt0(&mut self) -> Gpt0W<ScgcgptSpec> {
        Gpt0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    pub fn gpt1(&mut self) -> Gpt1W<ScgcgptSpec> {
        Gpt1W::new(self, 1)
    }
    #[doc = "Bit 2 - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    pub fn gpt2(&mut self) -> Gpt2W<ScgcgptSpec> {
        Gpt2W::new(self, 2)
    }
    #[doc = "Bit 3 - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    pub fn gpt3(&mut self) -> Gpt3W<ScgcgptSpec> {
        Gpt3W::new(self, 3)
    }
}
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`scgcgpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgcgpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScgcgptSpec;
impl crate::RegisterSpec for ScgcgptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcgpt::R`](R) reader structure"]
impl crate::Readable for ScgcgptSpec {}
#[doc = "`write(|w| ..)` method takes [`scgcgpt::W`](W) writer structure"]
impl crate::Writable for ScgcgptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGCGPT to value 0"]
impl crate::Resettable for ScgcgptSpec {
    const RESET_VALUE: u32 = 0;
}
