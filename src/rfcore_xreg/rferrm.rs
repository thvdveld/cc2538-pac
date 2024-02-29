#[doc = "Register `RFERRM` reader"]
pub type R = crate::R<RferrmSpec>;
#[doc = "Register `RFERRM` writer"]
pub type W = crate::W<RferrmSpec>;
#[doc = "Field `RFERRM` reader - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
pub type RferrmR = crate::FieldReader;
#[doc = "Field `RFERRM` writer - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
pub type RferrmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
    #[inline(always)]
    pub fn rferrm(&self) -> RferrmR {
        RferrmR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn rferrm(&mut self) -> RferrmW<RferrmSpec> {
        RferrmW::new(self, 0)
    }
}
#[doc = "RF error interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rferrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rferrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RferrmSpec;
impl crate::RegisterSpec for RferrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rferrm::R`](R) reader structure"]
impl crate::Readable for RferrmSpec {}
#[doc = "`write(|w| ..)` method takes [`rferrm::W`](W) writer structure"]
impl crate::Writable for RferrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFERRM to value 0"]
impl crate::Resettable for RferrmSpec {
    const RESET_VALUE: u32 = 0;
}
