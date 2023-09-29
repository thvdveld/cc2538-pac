#[doc = "Register `CIF` reader"]
pub type R = crate::R<CIF_SPEC>;
#[doc = "Field `SUSPENDIF` reader - Suspend interrupt flag Cleared by hardware when read"]
pub type SUSPENDIF_R = crate::BitReader;
#[doc = "Field `RESUMEIF` reader - Resume interrupt flag Cleared by hardware when read"]
pub type RESUMEIF_R = crate::BitReader;
#[doc = "Field `RSTIF` reader - Reset interrupt flag Cleared by hardware when read"]
pub type RSTIF_R = crate::BitReader;
#[doc = "Field `SOFIF` reader - Start-of-frame interrupt flag Cleared by hardware when read"]
pub type SOFIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Suspend interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn suspendif(&self) -> SUSPENDIF_R {
        SUSPENDIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn resumeif(&self) -> RESUMEIF_R {
        RESUMEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn rstif(&self) -> RSTIF_R {
        RSTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start-of-frame interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Common USB interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cif::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIF_SPEC;
impl crate::RegisterSpec for CIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cif::R`](R) reader structure"]
impl crate::Readable for CIF_SPEC {}
#[doc = "`reset()` method sets CIF to value 0"]
impl crate::Resettable for CIF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
