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
#[doc = "Field `OUTEP5IF` reader - Interrupt flag for OUT endpoint 5 Cleared by hardware when read"]
pub struct OUTEP5IF_R(crate::FieldReader<bool, bool>);
impl OUTEP5IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP5IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP5IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP4IF` reader - Interrupt flag for OUT endpoint 4 Cleared by hardware when read"]
pub struct OUTEP4IF_R(crate::FieldReader<bool, bool>);
impl OUTEP4IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP4IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP4IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP3IF` reader - Interrupt flag for OUT endpoint 3 Cleared by hardware when read"]
pub struct OUTEP3IF_R(crate::FieldReader<bool, bool>);
impl OUTEP3IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP3IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP3IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP2IF` reader - Interrupt flag for OUT endpoint 2 Cleared by hardware when read"]
pub struct OUTEP2IF_R(crate::FieldReader<bool, bool>);
impl OUTEP2IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEP1IF` reader - Interrupt flag for OUT endpoint 1 Cleared by hardware when read"]
pub struct OUTEP1IF_R(crate::FieldReader<bool, bool>);
impl OUTEP1IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTEP1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEP1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - Interrupt flag for OUT endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep5if(&self) -> OUTEP5IF_R {
        OUTEP5IF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for OUT endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep4if(&self) -> OUTEP4IF_R {
        OUTEP4IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for OUT endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep3if(&self) -> OUTEP3IF_R {
        OUTEP3IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for OUT endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep2if(&self) -> OUTEP2IF_R {
        OUTEP2IF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for OUT endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep1if(&self) -> OUTEP1IF_R {
        OUTEP1IF_R::new(((self.bits >> 1) & 0x01) != 0)
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
