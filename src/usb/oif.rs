#[doc = "Register `OIF` reader"]
pub type R = crate::R<OifSpec>;
#[doc = "Field `OUTEP1IF` reader - Interrupt flag for OUT endpoint 1 Cleared by hardware when read"]
pub type Outep1ifR = crate::BitReader;
#[doc = "Field `OUTEP2IF` reader - Interrupt flag for OUT endpoint 2 Cleared by hardware when read"]
pub type Outep2ifR = crate::BitReader;
#[doc = "Field `OUTEP3IF` reader - Interrupt flag for OUT endpoint 3 Cleared by hardware when read"]
pub type Outep3ifR = crate::BitReader;
#[doc = "Field `OUTEP4IF` reader - Interrupt flag for OUT endpoint 4 Cleared by hardware when read"]
pub type Outep4ifR = crate::BitReader;
#[doc = "Field `OUTEP5IF` reader - Interrupt flag for OUT endpoint 5 Cleared by hardware when read"]
pub type Outep5ifR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Interrupt flag for OUT endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep1if(&self) -> Outep1ifR {
        Outep1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for OUT endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep2if(&self) -> Outep2ifR {
        Outep2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for OUT endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep3if(&self) -> Outep3ifR {
        Outep3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for OUT endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep4if(&self) -> Outep4ifR {
        Outep4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for OUT endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep5if(&self) -> Outep5ifR {
        Outep5ifR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt flags for OUT endpoints 1-5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oif::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OifSpec;
impl crate::RegisterSpec for OifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oif::R`](R) reader structure"]
impl crate::Readable for OifSpec {}
#[doc = "`reset()` method sets OIF to value 0"]
impl crate::Resettable for OifSpec {
    const RESET_VALUE: u32 = 0;
}
