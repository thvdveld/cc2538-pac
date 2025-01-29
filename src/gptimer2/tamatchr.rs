#[doc = "Register `TAMATCHR` reader"]
pub type R = crate::R<TamatchrSpec>;
#[doc = "Register `TAMATCHR` writer"]
pub type W = crate::W<TamatchrSpec>;
#[doc = "Field `TAMR` reader - GPTM Timer A match register"]
pub type TamrR = crate::FieldReader<u32>;
#[doc = "Field `TAMR` writer - GPTM Timer A match register"]
pub type TamrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A match register"]
    #[inline(always)]
    pub fn tamr(&self) -> TamrR {
        TamrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM Timer A match register"]
    #[inline(always)]
    pub fn tamr(&mut self) -> TamrW<TamatchrSpec> {
        TamrW::new(self, 0)
    }
}
#[doc = "GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tamatchr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamatchr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TamatchrSpec;
impl crate::RegisterSpec for TamatchrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamatchr::R`](R) reader structure"]
impl crate::Readable for TamatchrSpec {}
#[doc = "`write(|w| ..)` method takes [`tamatchr::W`](W) writer structure"]
impl crate::Writable for TamatchrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMATCHR to value 0"]
impl crate::Resettable for TamatchrSpec {
    const RESET_VALUE: u32 = 0;
}
