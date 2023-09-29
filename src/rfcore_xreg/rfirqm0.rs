#[doc = "Register `RFIRQM0` reader"]
pub type R = crate::R<RFIRQM0_SPEC>;
#[doc = "Register `RFIRQM0` writer"]
pub type W = crate::W<RFIRQM0_SPEC>;
#[doc = "Field `RFIRQM` reader - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
pub type RFIRQM_R = crate::FieldReader;
#[doc = "Field `RFIRQM` writer - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
pub type RFIRQM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
    #[inline(always)]
    pub fn rfirqm(&self) -> RFIRQM_R {
        RFIRQM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
    #[inline(always)]
    #[must_use]
    pub fn rfirqm(&mut self) -> RFIRQM_W<RFIRQM0_SPEC, 0> {
        RFIRQM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RF interrupt masks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIRQM0_SPEC;
impl crate::RegisterSpec for RFIRQM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfirqm0::R`](R) reader structure"]
impl crate::Readable for RFIRQM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfirqm0::W`](W) writer structure"]
impl crate::Writable for RFIRQM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIRQM0 to value 0"]
impl crate::Resettable for RFIRQM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
