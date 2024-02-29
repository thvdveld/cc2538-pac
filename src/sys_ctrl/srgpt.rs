#[doc = "Register `SRGPT` reader"]
pub type R = crate::R<SrgptSpec>;
#[doc = "Register `SRGPT` writer"]
pub type W = crate::W<SrgptSpec>;
#[doc = "Field `GPT0` reader - 0: GPT0 module is not reset 1: GPT0 module is reset"]
pub type Gpt0R = crate::BitReader;
#[doc = "Field `GPT0` writer - 0: GPT0 module is not reset 1: GPT0 module is reset"]
pub type Gpt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPT1` reader - 0: GPT1 module is not reset 1: GPT1 module is reset"]
pub type Gpt1R = crate::BitReader;
#[doc = "Field `GPT1` writer - 0: GPT1 module is not reset 1: GPT1 module is reset"]
pub type Gpt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPT2` reader - 0: GPT2 module is not reset 1: GPT2 module is reset"]
pub type Gpt2R = crate::BitReader;
#[doc = "Field `GPT2` writer - 0: GPT2 module is not reset 1: GPT2 module is reset"]
pub type Gpt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPT3` reader - 0: GPT3 module is not reset 1: GPT3 module is reset"]
pub type Gpt3R = crate::BitReader;
#[doc = "Field `GPT3` writer - 0: GPT3 module is not reset 1: GPT3 module is reset"]
pub type Gpt3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: GPT0 module is not reset 1: GPT0 module is reset"]
    #[inline(always)]
    pub fn gpt0(&self) -> Gpt0R {
        Gpt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: GPT1 module is not reset 1: GPT1 module is reset"]
    #[inline(always)]
    pub fn gpt1(&self) -> Gpt1R {
        Gpt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: GPT2 module is not reset 1: GPT2 module is reset"]
    #[inline(always)]
    pub fn gpt2(&self) -> Gpt2R {
        Gpt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: GPT3 module is not reset 1: GPT3 module is reset"]
    #[inline(always)]
    pub fn gpt3(&self) -> Gpt3R {
        Gpt3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: GPT0 module is not reset 1: GPT0 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpt0(&mut self) -> Gpt0W<SrgptSpec> {
        Gpt0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: GPT1 module is not reset 1: GPT1 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpt1(&mut self) -> Gpt1W<SrgptSpec> {
        Gpt1W::new(self, 1)
    }
    #[doc = "Bit 2 - 0: GPT2 module is not reset 1: GPT2 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpt2(&mut self) -> Gpt2W<SrgptSpec> {
        Gpt2W::new(self, 2)
    }
    #[doc = "Bit 3 - 0: GPT3 module is not reset 1: GPT3 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpt3(&mut self) -> Gpt3W<SrgptSpec> {
        Gpt3W::new(self, 3)
    }
}
#[doc = "This register controls the reset for GPT\\[3:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srgpt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srgpt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrgptSpec;
impl crate::RegisterSpec for SrgptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srgpt::R`](R) reader structure"]
impl crate::Readable for SrgptSpec {}
#[doc = "`write(|w| ..)` method takes [`srgpt::W`](W) writer structure"]
impl crate::Writable for SrgptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRGPT to value 0"]
impl crate::Resettable for SrgptSpec {
    const RESET_VALUE: u32 = 0;
}
