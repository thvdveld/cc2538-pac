#[doc = "Register `SCGCGPT` reader"]
pub struct R(crate::R<SCGCGPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCGPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCGPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCGPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCGPT` writer"]
pub struct W(crate::W<SCGCGPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCGPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCGCGPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCGPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPT0` reader - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
pub type GPT0_R = crate::BitReader<bool>;
#[doc = "Field `GPT0` writer - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
pub type GPT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGCGPT_SPEC, bool, O>;
#[doc = "Field `GPT1` reader - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
pub type GPT1_R = crate::BitReader<bool>;
#[doc = "Field `GPT1` writer - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
pub type GPT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGCGPT_SPEC, bool, O>;
#[doc = "Field `GPT2` reader - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
pub type GPT2_R = crate::BitReader<bool>;
#[doc = "Field `GPT2` writer - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
pub type GPT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGCGPT_SPEC, bool, O>;
#[doc = "Field `GPT3` reader - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
pub type GPT3_R = crate::BitReader<bool>;
#[doc = "Field `GPT3` writer - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
pub type GPT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGCGPT_SPEC, bool, O>;
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
    pub fn gpt0(&mut self) -> GPT0_W<0> {
        GPT0_W::new(self)
    }
    #[doc = "Bit 1 - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    pub fn gpt1(&mut self) -> GPT1_W<1> {
        GPT1_W::new(self)
    }
    #[doc = "Bit 2 - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    pub fn gpt2(&mut self) -> GPT2_W<2> {
        GPT2_W::new(self)
    }
    #[doc = "Bit 3 - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    pub fn gpt3(&mut self) -> GPT3_W<3> {
        GPT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcgpt](index.html) module"]
pub struct SCGCGPT_SPEC;
impl crate::RegisterSpec for SCGCGPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcgpt::R](R) reader structure"]
impl crate::Readable for SCGCGPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcgpt::W](W) writer structure"]
impl crate::Writable for SCGCGPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCGPT to value 0"]
impl crate::Resettable for SCGCGPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
