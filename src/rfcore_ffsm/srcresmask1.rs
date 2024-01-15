#[doc = "Register `SRCRESMASK1` reader"]
pub type R = crate::R<SRCRESMASK1_SPEC>;
#[doc = "Register `SRCRESMASK1` writer"]
pub type W = crate::W<SRCRESMASK1_SPEC>;
#[doc = "Field `SRCRESMASK1` reader - Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
pub type SRCRESMASK1_R = crate::FieldReader;
#[doc = "Field `SRCRESMASK1` writer - Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
pub type SRCRESMASK1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask1(&self) -> SRCRESMASK1_R {
        SRCRESMASK1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
    #[inline(always)]
    #[must_use]
    pub fn srcresmask1(&mut self) -> SRCRESMASK1_W<SRCRESMASK1_SPEC> {
        SRCRESMASK1_W::new(self, 0)
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
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCRESMASK1_SPEC;
impl crate::RegisterSpec for SRCRESMASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcresmask1::R`](R) reader structure"]
impl crate::Readable for SRCRESMASK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcresmask1::W`](W) writer structure"]
impl crate::Writable for SRCRESMASK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCRESMASK1 to value 0"]
impl crate::Resettable for SRCRESMASK1_SPEC {
    const RESET_VALUE: u32 = 0;
}
