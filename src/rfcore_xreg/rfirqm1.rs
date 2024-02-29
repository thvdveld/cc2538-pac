#[doc = "Register `RFIRQM1` reader"]
pub type R = crate::R<Rfirqm1Spec>;
#[doc = "Register `RFIRQM1` writer"]
pub type W = crate::W<Rfirqm1Spec>;
#[doc = "Field `RFIRQM` reader - Bit mask is masking out interrupt sources. Bit position: 5: CSP_WAIT 4: CSP_STOP 3: CSP_MANINT 2: RF_IDLE 1: TXDONE 0: TXACKDONE"]
pub type RfirqmR = crate::FieldReader;
#[doc = "Field `RFIRQM` writer - Bit mask is masking out interrupt sources. Bit position: 5: CSP_WAIT 4: CSP_STOP 3: CSP_MANINT 2: RF_IDLE 1: TXDONE 0: TXACKDONE"]
pub type RfirqmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Bit mask is masking out interrupt sources. Bit position: 5: CSP_WAIT 4: CSP_STOP 3: CSP_MANINT 2: RF_IDLE 1: TXDONE 0: TXACKDONE"]
    #[inline(always)]
    pub fn rfirqm(&self) -> RfirqmR {
        RfirqmR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Bit mask is masking out interrupt sources. Bit position: 5: CSP_WAIT 4: CSP_STOP 3: CSP_MANINT 2: RF_IDLE 1: TXDONE 0: TXACKDONE"]
    #[inline(always)]
    #[must_use]
    pub fn rfirqm(&mut self) -> RfirqmW<Rfirqm1Spec> {
        RfirqmW::new(self, 0)
    }
}
#[doc = "RF interrupt masks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfirqm1Spec;
impl crate::RegisterSpec for Rfirqm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfirqm1::R`](R) reader structure"]
impl crate::Readable for Rfirqm1Spec {}
#[doc = "`write(|w| ..)` method takes [`rfirqm1::W`](W) writer structure"]
impl crate::Writable for Rfirqm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFIRQM1 to value 0"]
impl crate::Resettable for Rfirqm1Spec {
    const RESET_VALUE: u32 = 0;
}
