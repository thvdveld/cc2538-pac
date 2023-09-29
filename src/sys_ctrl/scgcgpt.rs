#[doc = "Register `SCGCGPT` reader"]
pub type R = crate::R<SCGCGPT_SPEC>;
#[doc = "Register `SCGCGPT` writer"]
pub type W = crate::W<SCGCGPT_SPEC>;
#[doc = "Field `GPT0` reader - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
pub type GPT0_R = crate::BitReader;
#[doc = "Field `GPT0` writer - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
pub type GPT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPT1` reader - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
pub type GPT1_R = crate::BitReader;
#[doc = "Field `GPT1` writer - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
pub type GPT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPT2` reader - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
pub type GPT2_R = crate::BitReader;
#[doc = "Field `GPT2` writer - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
pub type GPT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPT3` reader - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
pub type GPT3_R = crate::BitReader;
#[doc = "Field `GPT3` writer - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
pub type GPT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
    #[inline(always)]
    pub fn gpt0(&self) -> GPT0_R {
        GPT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    pub fn gpt1(&self) -> GPT1_R {
        GPT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    pub fn gpt2(&self) -> GPT2_R {
        GPT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    pub fn gpt3(&self) -> GPT3_R {
        GPT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt0(&mut self) -> GPT0_W<SCGCGPT_SPEC, 0> {
        GPT0_W::new(self)
    }
    #[doc = "Bit 1 - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt1(&mut self) -> GPT1_W<SCGCGPT_SPEC, 1> {
        GPT1_W::new(self)
    }
    #[doc = "Bit 2 - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt2(&mut self) -> GPT2_W<SCGCGPT_SPEC, 2> {
        GPT2_W::new(self)
    }
    #[doc = "Bit 3 - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt3(&mut self) -> GPT3_W<SCGCGPT_SPEC, 3> {
        GPT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcgpt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcgpt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCGPT_SPEC;
impl crate::RegisterSpec for SCGCGPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcgpt::R`](R) reader structure"]
impl crate::Readable for SCGCGPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcgpt::W`](W) writer structure"]
impl crate::Writable for SCGCGPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCGPT to value 0"]
impl crate::Resettable for SCGCGPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
