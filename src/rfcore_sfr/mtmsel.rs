#[doc = "Register `MTMSEL` reader"]
pub type R = crate::R<MtmselSpec>;
#[doc = "Register `MTMSEL` writer"]
pub type W = crate::W<MtmselSpec>;
#[doc = "Field `MTMSEL` reader - The value of this register selects the internal registers that are modified or read when accessing MTM0 and MTM1. 000: MTtim (timer count value) 001: MT_cap (timer capture) 010: MT_per (timer period) 011: MT_cmp1 (timer compare 1) 100: MT_cmp2 (timer compare 2) 101 to 111: Reserved MTM0"]
pub type MtmselR = crate::FieldReader;
#[doc = "Field `MTMSEL` writer - The value of this register selects the internal registers that are modified or read when accessing MTM0 and MTM1. 000: MTtim (timer count value) 001: MT_cap (timer capture) 010: MT_per (timer period) 011: MT_cmp1 (timer compare 1) 100: MT_cmp2 (timer compare 2) 101 to 111: Reserved MTM0"]
pub type MtmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MTMOVFSEL` reader - The value of this register selects the internal registers that are modified or read when accessing MTMOVF0, MTMOVF1, and MTMOVF2. 000: MTovf (overflow counter) 001: MTovf_cap (overflow capture) 010: MTovf_per (overflow period) 011: MTovf_cmp1 (overflow compare 1) 100: MTovf_cmp2 (overflow compare 2) 101 to 111: Reserved"]
pub type MtmovfselR = crate::FieldReader;
#[doc = "Field `MTMOVFSEL` writer - The value of this register selects the internal registers that are modified or read when accessing MTMOVF0, MTMOVF1, and MTMOVF2. 000: MTovf (overflow counter) 001: MTovf_cap (overflow capture) 010: MTovf_per (overflow period) 011: MTovf_cmp1 (overflow compare 1) 100: MTovf_cmp2 (overflow compare 2) 101 to 111: Reserved"]
pub type MtmovfselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The value of this register selects the internal registers that are modified or read when accessing MTM0 and MTM1. 000: MTtim (timer count value) 001: MT_cap (timer capture) 010: MT_per (timer period) 011: MT_cmp1 (timer compare 1) 100: MT_cmp2 (timer compare 2) 101 to 111: Reserved MTM0"]
    #[inline(always)]
    pub fn mtmsel(&self) -> MtmselR {
        MtmselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - The value of this register selects the internal registers that are modified or read when accessing MTMOVF0, MTMOVF1, and MTMOVF2. 000: MTovf (overflow counter) 001: MTovf_cap (overflow capture) 010: MTovf_per (overflow period) 011: MTovf_cmp1 (overflow compare 1) 100: MTovf_cmp2 (overflow compare 2) 101 to 111: Reserved"]
    #[inline(always)]
    pub fn mtmovfsel(&self) -> MtmovfselR {
        MtmovfselR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The value of this register selects the internal registers that are modified or read when accessing MTM0 and MTM1. 000: MTtim (timer count value) 001: MT_cap (timer capture) 010: MT_per (timer period) 011: MT_cmp1 (timer compare 1) 100: MT_cmp2 (timer compare 2) 101 to 111: Reserved MTM0"]
    #[inline(always)]
    #[must_use]
    pub fn mtmsel(&mut self) -> MtmselW<MtmselSpec> {
        MtmselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - The value of this register selects the internal registers that are modified or read when accessing MTMOVF0, MTMOVF1, and MTMOVF2. 000: MTovf (overflow counter) 001: MTovf_cap (overflow capture) 010: MTovf_per (overflow period) 011: MTovf_cmp1 (overflow compare 1) 100: MTovf_cmp2 (overflow compare 2) 101 to 111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mtmovfsel(&mut self) -> MtmovfselW<MtmselSpec> {
        MtmovfselW::new(self, 4)
    }
}
#[doc = "MAC Timer multiplex select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtmselSpec;
impl crate::RegisterSpec for MtmselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtmsel::R`](R) reader structure"]
impl crate::Readable for MtmselSpec {}
#[doc = "`write(|w| ..)` method takes [`mtmsel::W`](W) writer structure"]
impl crate::Writable for MtmselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTMSEL to value 0"]
impl crate::Resettable for MtmselSpec {
    const RESET_VALUE: u32 = 0;
}
