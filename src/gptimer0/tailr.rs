#[doc = "Register `TAILR` reader"]
pub type R = crate::R<TailrSpec>;
#[doc = "Register `TAILR` writer"]
pub type W = crate::W<TailrSpec>;
#[doc = "Field `TAILR` reader - GPTM A interval load register"]
pub type TailrR = crate::FieldReader<u32>;
#[doc = "Field `TAILR` writer - GPTM A interval load register"]
pub type TailrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPTM A interval load register"]
    #[inline(always)]
    pub fn tailr(&self) -> TailrR {
        TailrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM A interval load register"]
    #[inline(always)]
    pub fn tailr(&mut self) -> TailrW<TailrSpec> {
        TailrW::new(self, 0)
    }
}
#[doc = "GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tailr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tailr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TailrSpec;
impl crate::RegisterSpec for TailrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tailr::R`](R) reader structure"]
impl crate::Readable for TailrSpec {}
#[doc = "`write(|w| ..)` method takes [`tailr::W`](W) writer structure"]
impl crate::Writable for TailrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAILR to value 0"]
impl crate::Resettable for TailrSpec {
    const RESET_VALUE: u32 = 0;
}
