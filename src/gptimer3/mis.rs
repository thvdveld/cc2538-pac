#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `TATOMIS` reader - GPTM Timer A time-out raw interrupt"]
pub type TatomisR = crate::BitReader;
#[doc = "Field `CAMMIS` reader - GPTM Timer A capture match raw interrupt"]
pub type CammisR = crate::BitReader;
#[doc = "Field `CAEMIS` reader - GPTM Timer A capture event raw interrupt"]
pub type CaemisR = crate::BitReader;
#[doc = "Field `TAMRIS` reader - GPTM Timer A match raw interrupt"]
pub type TamrisR = crate::BitReader;
#[doc = "Field `TBTOMIS` reader - GPTM Timer B time-out masked interrupt"]
pub type TbtomisR = crate::BitReader;
#[doc = "Field `CBMMIS` reader - GPTM Timer B capture match masked interrupt"]
pub type CbmmisR = crate::BitReader;
#[doc = "Field `CBEMIS` reader - GPTM Timer B capture event masked interrupt"]
pub type CbemisR = crate::BitReader;
#[doc = "Field `TBMMIS` reader - GPTM Timer B match masked interrupt"]
pub type TbmmisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - GPTM Timer A time-out raw interrupt"]
    #[inline(always)]
    pub fn tatomis(&self) -> TatomisR {
        TatomisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match raw interrupt"]
    #[inline(always)]
    pub fn cammis(&self) -> CammisR {
        CammisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event raw interrupt"]
    #[inline(always)]
    pub fn caemis(&self) -> CaemisR {
        CaemisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match raw interrupt"]
    #[inline(always)]
    pub fn tamris(&self) -> TamrisR {
        TamrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out masked interrupt"]
    #[inline(always)]
    pub fn tbtomis(&self) -> TbtomisR {
        TbtomisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match masked interrupt"]
    #[inline(always)]
    pub fn cbmmis(&self) -> CbmmisR {
        CbmmisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event masked interrupt"]
    #[inline(always)]
    pub fn cbemis(&self) -> CbemisR {
        CbemisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B match masked interrupt"]
    #[inline(always)]
    pub fn tbmmis(&self) -> TbmmisR {
        TbmmisR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
