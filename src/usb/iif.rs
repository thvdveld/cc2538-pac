#[doc = "Register `IIF` reader"]
pub struct R(crate::R<IIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EP0IF` reader - Interrupt flag for endpoint 0 Cleared by hardware when read"]
pub type EP0IF_R = crate::BitReader<bool>;
#[doc = "Field `INEP1IF` reader - Interrupt flag for IN endpoint 1 Cleared by hardware when read"]
pub type INEP1IF_R = crate::BitReader<bool>;
#[doc = "Field `INEP2IF` reader - Interrupt flag for IN endpoint 2 Cleared by hardware when read"]
pub type INEP2IF_R = crate::BitReader<bool>;
#[doc = "Field `INEP3IF` reader - Interrupt flag for IN endpoint 3 Cleared by hardware when read"]
pub type INEP3IF_R = crate::BitReader<bool>;
#[doc = "Field `INEP4IF` reader - Interrupt flag for IN endpoint 4 Cleared by hardware when read"]
pub type INEP4IF_R = crate::BitReader<bool>;
#[doc = "Field `INEP5IF` reader - Interrupt flag for IN endpoint 5 Cleared by hardware when read"]
pub type INEP5IF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Interrupt flag for endpoint 0 Cleared by hardware when read"]
    #[inline(always)]
    pub fn ep0if(&self) -> EP0IF_R {
        EP0IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for IN endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep1if(&self) -> INEP1IF_R {
        INEP1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for IN endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep2if(&self) -> INEP2IF_R {
        INEP2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for IN endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep3if(&self) -> INEP3IF_R {
        INEP3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for IN endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep4if(&self) -> INEP4IF_R {
        INEP4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for IN endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep5if(&self) -> INEP5IF_R {
        INEP5IF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt flags for endpoint 0 and IN endpoints 1-5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iif](index.html) module"]
pub struct IIF_SPEC;
impl crate::RegisterSpec for IIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iif::R](R) reader structure"]
impl crate::Readable for IIF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIF to value 0"]
impl crate::Resettable for IIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
