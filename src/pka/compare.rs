#[doc = "Register `COMPARE` reader"]
pub struct R(crate::R<COMPARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `A_GREATER_THAN_B` reader - Vector_A is greater than Vector_B"]
pub struct A_GREATER_THAN_B_R(crate::FieldReader<bool, bool>);
impl A_GREATER_THAN_B_R {
    pub(crate) fn new(bits: bool) -> Self {
        A_GREATER_THAN_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_GREATER_THAN_B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A_LESS_THAN_B` reader - Vector_A is less than Vector_B"]
pub struct A_LESS_THAN_B_R(crate::FieldReader<bool, bool>);
impl A_LESS_THAN_B_R {
    pub(crate) fn new(bits: bool) -> Self {
        A_LESS_THAN_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_LESS_THAN_B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A_EQUALS_B` reader - Vector_A is equal to Vector_B"]
pub struct A_EQUALS_B_R(crate::FieldReader<bool, bool>);
impl A_EQUALS_B_R {
    pub(crate) fn new(bits: bool) -> Self {
        A_EQUALS_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_EQUALS_B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - Vector_A is greater than Vector_B"]
    #[inline(always)]
    pub fn a_greater_than_b(&self) -> A_GREATER_THAN_B_R {
        A_GREATER_THAN_B_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Vector_A is less than Vector_B"]
    #[inline(always)]
    pub fn a_less_than_b(&self) -> A_LESS_THAN_B_R {
        A_LESS_THAN_B_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Vector_A is equal to Vector_B"]
    #[inline(always)]
    pub fn a_equals_b(&self) -> A_EQUALS_B_R {
        A_EQUALS_B_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the run bit in the PKA_FUNCTION register is reset at the end of that operation. Status after a complex sequencer operation is unknown\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compare](index.html) module"]
pub struct COMPARE_SPEC;
impl crate::RegisterSpec for COMPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compare::R](R) reader structure"]
impl crate::Readable for COMPARE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMPARE to value 0"]
impl crate::Resettable for COMPARE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
