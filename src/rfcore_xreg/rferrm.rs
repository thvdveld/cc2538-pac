#[doc = "Register `RFERRM` reader"]
pub type R = crate::R<RFERRM_SPEC>;
#[doc = "Register `RFERRM` writer"]
pub type W = crate::W<RFERRM_SPEC>;
#[doc = "Field `RFERRM` reader - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
pub type RFERRM_R = crate::FieldReader;
#[doc = "Field `RFERRM` writer - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
pub type RFERRM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
    #[inline(always)]
    pub fn rferrm(&self) -> RFERRM_R {
        RFERRM_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn rferrm(&mut self) -> RFERRM_W<RFERRM_SPEC> {
        RFERRM_W::new(self, 0)
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
#[doc = "RF error interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rferrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rferrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFERRM_SPEC;
impl crate::RegisterSpec for RFERRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rferrm::R`](R) reader structure"]
impl crate::Readable for RFERRM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rferrm::W`](W) writer structure"]
impl crate::Writable for RFERRM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFERRM to value 0"]
impl crate::Resettable for RFERRM_SPEC {
    const RESET_VALUE: u32 = 0;
}
