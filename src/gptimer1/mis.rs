#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Field `TATOMIS` reader - GPTM Timer A time-out raw interrupt"]
pub type TATOMIS_R = crate::BitReader;
#[doc = "Field `CAMMIS` reader - GPTM Timer A capture match raw interrupt"]
pub type CAMMIS_R = crate::BitReader;
#[doc = "Field `CAEMIS` reader - GPTM Timer A capture event raw interrupt"]
pub type CAEMIS_R = crate::BitReader;
#[doc = "Field `TAMRIS` reader - GPTM Timer A match raw interrupt"]
pub type TAMRIS_R = crate::BitReader;
#[doc = "Field `TBTOMIS` reader - GPTM Timer B time-out masked interrupt"]
pub type TBTOMIS_R = crate::BitReader;
#[doc = "Field `CBMMIS` reader - GPTM Timer B capture match masked interrupt"]
pub type CBMMIS_R = crate::BitReader;
#[doc = "Field `CBEMIS` reader - GPTM Timer B capture event masked interrupt"]
pub type CBEMIS_R = crate::BitReader;
#[doc = "Field `TBMMIS` reader - GPTM Timer B match masked interrupt"]
pub type TBMMIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - GPTM Timer A time-out raw interrupt"]
    #[inline(always)]
    pub fn tatomis(&self) -> TATOMIS_R {
        TATOMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match raw interrupt"]
    #[inline(always)]
    pub fn cammis(&self) -> CAMMIS_R {
        CAMMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event raw interrupt"]
    #[inline(always)]
    pub fn caemis(&self) -> CAEMIS_R {
        CAEMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match raw interrupt"]
    #[inline(always)]
    pub fn tamris(&self) -> TAMRIS_R {
        TAMRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out masked interrupt"]
    #[inline(always)]
    pub fn tbtomis(&self) -> TBTOMIS_R {
        TBTOMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match masked interrupt"]
    #[inline(always)]
    pub fn cbmmis(&self) -> CBMMIS_R {
        CBMMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event masked interrupt"]
    #[inline(always)]
    pub fn cbemis(&self) -> CBEMIS_R {
        CBEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B match masked interrupt"]
    #[inline(always)]
    pub fn tbmmis(&self) -> TBMMIS_R {
        TBMMIS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
