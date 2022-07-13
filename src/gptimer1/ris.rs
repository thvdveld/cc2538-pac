#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TBMRIS` reader - GPTM Timer B match raw interrupt"]
pub type TBMRIS_R = crate::BitReader<bool>;
#[doc = "Field `CBERIS` reader - GPTM Timer B capture event raw interrupt"]
pub type CBERIS_R = crate::BitReader<bool>;
#[doc = "Field `CBMRIS` reader - GPTM Timer B capture match raw interrupt"]
pub type CBMRIS_R = crate::BitReader<bool>;
#[doc = "Field `TBTORIS` reader - GPTM Timer B time-out raw interrupt"]
pub type TBTORIS_R = crate::BitReader<bool>;
#[doc = "Field `TAMRIS` reader - GPTM Timer A match raw interrupt"]
pub type TAMRIS_R = crate::BitReader<bool>;
#[doc = "Field `CAERIS` reader - GPTM Timer A capture event raw interrupt"]
pub type CAERIS_R = crate::BitReader<bool>;
#[doc = "Field `CAMRIS` reader - GPTM Timer A capture match raw interrupt"]
pub type CAMRIS_R = crate::BitReader<bool>;
#[doc = "Field `TATORIS` reader - GPTM Timer A time-out raw interrupt"]
pub type TATORIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 11 - GPTM Timer B match raw interrupt"]
    #[inline(always)]
    pub fn tbmris(&self) -> TBMRIS_R {
        TBMRIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event raw interrupt"]
    #[inline(always)]
    pub fn cberis(&self) -> CBERIS_R {
        CBERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match raw interrupt"]
    #[inline(always)]
    pub fn cbmris(&self) -> CBMRIS_R {
        CBMRIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out raw interrupt"]
    #[inline(always)]
    pub fn tbtoris(&self) -> TBTORIS_R {
        TBTORIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match raw interrupt"]
    #[inline(always)]
    pub fn tamris(&self) -> TAMRIS_R {
        TAMRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event raw interrupt"]
    #[inline(always)]
    pub fn caeris(&self) -> CAERIS_R {
        CAERIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match raw interrupt"]
    #[inline(always)]
    pub fn camris(&self) -> CAMRIS_R {
        CAMRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - GPTM Timer A time-out raw interrupt"]
    #[inline(always)]
    pub fn tatoris(&self) -> TATORIS_R {
        TATORIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
