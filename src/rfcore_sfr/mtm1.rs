#[doc = "Register `MTM1` reader"]
pub type R = crate::R<MTM1_SPEC>;
#[doc = "Register `MTM1` writer"]
pub type W = crate::W<MTM1_SPEC>;
#[doc = "Field `MTM1` reader - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
pub type MTM1_R = crate::FieldReader;
#[doc = "Field `MTM1` writer - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
pub type MTM1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
    #[inline(always)]
    pub fn mtm1(&self) -> MTM1_R {
        MTM1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mtm1(&mut self) -> MTM1_W<MTM1_SPEC> {
        MTM1_W::new(self, 0)
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
#[doc = "MAC Timer multiplexed register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTM1_SPEC;
impl crate::RegisterSpec for MTM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtm1::R`](R) reader structure"]
impl crate::Readable for MTM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtm1::W`](W) writer structure"]
impl crate::Writable for MTM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTM1 to value 0"]
impl crate::Resettable for MTM1_SPEC {
    const RESET_VALUE: u32 = 0;
}
