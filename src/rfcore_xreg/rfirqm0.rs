#[doc = "Register `RFIRQM0` reader"]
pub type R = crate::R<Rfirqm0Spec>;
#[doc = "Register `RFIRQM0` writer"]
pub type W = crate::W<Rfirqm0Spec>;
#[doc = "Field `RFIRQM` reader - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
pub type RfirqmR = crate::FieldReader;
#[doc = "Field `RFIRQM` writer - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
pub type RfirqmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
    #[inline(always)]
    pub fn rfirqm(&self) -> RfirqmR {
        RfirqmR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
    #[inline(always)]
    pub fn rfirqm(&mut self) -> RfirqmW<Rfirqm0Spec> {
        RfirqmW::new(self, 0)
    }
}
#[doc = "RF interrupt masks\n\nYou can [`read`](crate::Reg::read) this register and get [`rfirqm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfirqm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfirqm0Spec;
impl crate::RegisterSpec for Rfirqm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfirqm0::R`](R) reader structure"]
impl crate::Readable for Rfirqm0Spec {}
#[doc = "`write(|w| ..)` method takes [`rfirqm0::W`](W) writer structure"]
impl crate::Writable for Rfirqm0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFIRQM0 to value 0"]
impl crate::Resettable for Rfirqm0Spec {
    const RESET_VALUE: u32 = 0;
}
