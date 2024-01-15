#[doc = "Register `GPT` reader"]
pub type R = crate::R<GPT_SPEC>;
#[doc = "Register `GPT` writer"]
pub type W = crate::W<GPT_SPEC>;
#[doc = "Field `GPTIDOV` reader - GPTimer increment/decrement override value"]
pub type GPTIDOV_R = crate::FieldReader;
#[doc = "Field `GPTIDOV` writer - GPTimer increment/decrement override value"]
pub type GPTIDOV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GPTIDOE` reader - GPTimer increment/decrement override enable"]
pub type GPTIDOE_R = crate::BitReader;
#[doc = "Field `GPTIDOE` writer - GPTimer increment/decrement override enable"]
pub type GPTIDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - GPTimer increment/decrement override value"]
    #[inline(always)]
    pub fn gptidov(&self) -> GPTIDOV_R {
        GPTIDOV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - GPTimer increment/decrement override enable"]
    #[inline(always)]
    pub fn gptidoe(&self) -> GPTIDOE_R {
        GPTIDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - GPTimer increment/decrement override value"]
    #[inline(always)]
    #[must_use]
    pub fn gptidov(&mut self) -> GPTIDOV_W<GPT_SPEC> {
        GPTIDOV_W::new(self, 0)
    }
    #[doc = "Bit 8 - GPTimer increment/decrement override enable"]
    #[inline(always)]
    #[must_use]
    pub fn gptidoe(&mut self) -> GPTIDOE_W<GPT_SPEC> {
        GPTIDOE_W::new(self, 8)
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
#[doc = "GPTIMER override values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPT_SPEC;
impl crate::RegisterSpec for GPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpt::R`](R) reader structure"]
impl crate::Readable for GPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpt::W`](W) writer structure"]
impl crate::Writable for GPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPT to value 0"]
impl crate::Resettable for GPT_SPEC {
    const RESET_VALUE: u32 = 0;
}
