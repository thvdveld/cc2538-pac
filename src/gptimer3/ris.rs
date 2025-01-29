#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `TATORIS` reader - GPTM Timer A time-out raw interrupt"]
pub type TatorisR = crate::BitReader;
#[doc = "Field `CAMRIS` reader - GPTM Timer A capture match raw interrupt"]
pub type CamrisR = crate::BitReader;
#[doc = "Field `CAERIS` reader - GPTM Timer A capture event raw interrupt"]
pub type CaerisR = crate::BitReader;
#[doc = "Field `TAMRIS` reader - GPTM Timer A match raw interrupt"]
pub type TamrisR = crate::BitReader;
#[doc = "Field `TBTORIS` reader - GPTM Timer B time-out raw interrupt"]
pub type TbtorisR = crate::BitReader;
#[doc = "Field `CBMRIS` reader - GPTM Timer B capture match raw interrupt"]
pub type CbmrisR = crate::BitReader;
#[doc = "Field `CBERIS` reader - GPTM Timer B capture event raw interrupt"]
pub type CberisR = crate::BitReader;
#[doc = "Field `TBMRIS` reader - GPTM Timer B match raw interrupt"]
pub type TbmrisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - GPTM Timer A time-out raw interrupt"]
    #[inline(always)]
    pub fn tatoris(&self) -> TatorisR {
        TatorisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match raw interrupt"]
    #[inline(always)]
    pub fn camris(&self) -> CamrisR {
        CamrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event raw interrupt"]
    #[inline(always)]
    pub fn caeris(&self) -> CaerisR {
        CaerisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match raw interrupt"]
    #[inline(always)]
    pub fn tamris(&self) -> TamrisR {
        TamrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out raw interrupt"]
    #[inline(always)]
    pub fn tbtoris(&self) -> TbtorisR {
        TbtorisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match raw interrupt"]
    #[inline(always)]
    pub fn cbmris(&self) -> CbmrisR {
        CbmrisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event raw interrupt"]
    #[inline(always)]
    pub fn cberis(&self) -> CberisR {
        CberisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B match raw interrupt"]
    #[inline(always)]
    pub fn tbmris(&self) -> TbmrisR {
        TbmrisR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR.\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
