#[doc = "Register `GPT` reader"]
pub type R = crate::R<GptSpec>;
#[doc = "Register `GPT` writer"]
pub type W = crate::W<GptSpec>;
#[doc = "Field `GPTIDOV` reader - GPTimer increment/decrement override value"]
pub type GptidovR = crate::FieldReader;
#[doc = "Field `GPTIDOV` writer - GPTimer increment/decrement override value"]
pub type GptidovW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GPTIDOE` reader - GPTimer increment/decrement override enable"]
pub type GptidoeR = crate::BitReader;
#[doc = "Field `GPTIDOE` writer - GPTimer increment/decrement override enable"]
pub type GptidoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - GPTimer increment/decrement override value"]
    #[inline(always)]
    pub fn gptidov(&self) -> GptidovR {
        GptidovR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - GPTimer increment/decrement override enable"]
    #[inline(always)]
    pub fn gptidoe(&self) -> GptidoeR {
        GptidoeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - GPTimer increment/decrement override value"]
    #[inline(always)]
    #[must_use]
    pub fn gptidov(&mut self) -> GptidovW<GptSpec> {
        GptidovW::new(self, 0)
    }
    #[doc = "Bit 8 - GPTimer increment/decrement override enable"]
    #[inline(always)]
    #[must_use]
    pub fn gptidoe(&mut self) -> GptidoeW<GptSpec> {
        GptidoeW::new(self, 8)
    }
}
#[doc = "GPTIMER override values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptSpec;
impl crate::RegisterSpec for GptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpt::R`](R) reader structure"]
impl crate::Readable for GptSpec {}
#[doc = "`write(|w| ..)` method takes [`gpt::W`](W) writer structure"]
impl crate::Writable for GptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPT to value 0"]
impl crate::Resettable for GptSpec {
    const RESET_VALUE: u32 = 0;
}
