#[doc = "Register `OIF` reader"]
pub struct R(crate::R<OIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTEP1IF` reader - Interrupt flag for OUT endpoint 1 Cleared by hardware when read"]
pub type OUTEP1IF_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP2IF` reader - Interrupt flag for OUT endpoint 2 Cleared by hardware when read"]
pub type OUTEP2IF_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP3IF` reader - Interrupt flag for OUT endpoint 3 Cleared by hardware when read"]
pub type OUTEP3IF_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP4IF` reader - Interrupt flag for OUT endpoint 4 Cleared by hardware when read"]
pub type OUTEP4IF_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP5IF` reader - Interrupt flag for OUT endpoint 5 Cleared by hardware when read"]
pub type OUTEP5IF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Interrupt flag for OUT endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep1if(&self) -> OUTEP1IF_R {
        OUTEP1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for OUT endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep2if(&self) -> OUTEP2IF_R {
        OUTEP2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for OUT endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep3if(&self) -> OUTEP3IF_R {
        OUTEP3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for OUT endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep4if(&self) -> OUTEP4IF_R {
        OUTEP4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for OUT endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep5if(&self) -> OUTEP5IF_R {
        OUTEP5IF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt flags for OUT endpoints 1-5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oif](index.html) module"]
pub struct OIF_SPEC;
impl crate::RegisterSpec for OIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oif::R](R) reader structure"]
impl crate::Readable for OIF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OIF to value 0"]
impl crate::Resettable for OIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
