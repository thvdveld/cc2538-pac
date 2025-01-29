#[doc = "Register `IIF` reader"]
pub type R = crate::R<IifSpec>;
#[doc = "Field `EP0IF` reader - Interrupt flag for endpoint 0 Cleared by hardware when read"]
pub type Ep0ifR = crate::BitReader;
#[doc = "Field `INEP1IF` reader - Interrupt flag for IN endpoint 1 Cleared by hardware when read"]
pub type Inep1ifR = crate::BitReader;
#[doc = "Field `INEP2IF` reader - Interrupt flag for IN endpoint 2 Cleared by hardware when read"]
pub type Inep2ifR = crate::BitReader;
#[doc = "Field `INEP3IF` reader - Interrupt flag for IN endpoint 3 Cleared by hardware when read"]
pub type Inep3ifR = crate::BitReader;
#[doc = "Field `INEP4IF` reader - Interrupt flag for IN endpoint 4 Cleared by hardware when read"]
pub type Inep4ifR = crate::BitReader;
#[doc = "Field `INEP5IF` reader - Interrupt flag for IN endpoint 5 Cleared by hardware when read"]
pub type Inep5ifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt flag for endpoint 0 Cleared by hardware when read"]
    #[inline(always)]
    pub fn ep0if(&self) -> Ep0ifR {
        Ep0ifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for IN endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep1if(&self) -> Inep1ifR {
        Inep1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for IN endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep2if(&self) -> Inep2ifR {
        Inep2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for IN endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep3if(&self) -> Inep3ifR {
        Inep3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for IN endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep4if(&self) -> Inep4ifR {
        Inep4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for IN endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep5if(&self) -> Inep5ifR {
        Inep5ifR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt flags for endpoint 0 and IN endpoints 1-5\n\nYou can [`read`](crate::Reg::read) this register and get [`iif::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IifSpec;
impl crate::RegisterSpec for IifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iif::R`](R) reader structure"]
impl crate::Readable for IifSpec {}
#[doc = "`reset()` method sets IIF to value 0"]
impl crate::Resettable for IifSpec {
    const RESET_VALUE: u32 = 0;
}
