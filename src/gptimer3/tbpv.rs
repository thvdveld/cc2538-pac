#[doc = "Register `TBPV` reader"]
pub type R = crate::R<TBPV_SPEC>;
#[doc = "Field `PSV` reader - GPTM Timer B prescaler value"]
pub type PSV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B prescaler value"]
    #[inline(always)]
    pub fn psv(&self) -> PSV_R {
        PSV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBPV_SPEC;
impl crate::RegisterSpec for TBPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpv::R`](R) reader structure"]
impl crate::Readable for TBPV_SPEC {}
#[doc = "`reset()` method sets TBPV to value 0"]
impl crate::Resettable for TBPV_SPEC {
    const RESET_VALUE: u32 = 0;
}
