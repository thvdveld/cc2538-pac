#[doc = "Register `CIF` reader"]
pub type R = crate::R<CifSpec>;
#[doc = "Field `SUSPENDIF` reader - Suspend interrupt flag Cleared by hardware when read"]
pub type SuspendifR = crate::BitReader;
#[doc = "Field `RESUMEIF` reader - Resume interrupt flag Cleared by hardware when read"]
pub type ResumeifR = crate::BitReader;
#[doc = "Field `RSTIF` reader - Reset interrupt flag Cleared by hardware when read"]
pub type RstifR = crate::BitReader;
#[doc = "Field `SOFIF` reader - Start-of-frame interrupt flag Cleared by hardware when read"]
pub type SofifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Suspend interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn suspendif(&self) -> SuspendifR {
        SuspendifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn resumeif(&self) -> ResumeifR {
        ResumeifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn rstif(&self) -> RstifR {
        RstifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start-of-frame interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn sofif(&self) -> SofifR {
        SofifR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Common USB interrupt flags\n\nYou can [`read`](crate::Reg::read) this register and get [`cif::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CifSpec;
impl crate::RegisterSpec for CifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cif::R`](R) reader structure"]
impl crate::Readable for CifSpec {}
#[doc = "`reset()` method sets CIF to value 0"]
impl crate::Resettable for CifSpec {
    const RESET_VALUE: u32 = 0;
}
