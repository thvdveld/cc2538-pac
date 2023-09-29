#[doc = "Register `COMPARE` reader"]
pub type R = crate::R<COMPARE_SPEC>;
#[doc = "Field `A_EQUALS_B` reader - Vector_A is equal to Vector_B"]
pub type A_EQUALS_B_R = crate::BitReader;
#[doc = "Field `A_LESS_THAN_B` reader - Vector_A is less than Vector_B"]
pub type A_LESS_THAN_B_R = crate::BitReader;
#[doc = "Field `A_GREATER_THAN_B` reader - Vector_A is greater than Vector_B"]
pub type A_GREATER_THAN_B_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Vector_A is equal to Vector_B"]
    #[inline(always)]
    pub fn a_equals_b(&self) -> A_EQUALS_B_R {
        A_EQUALS_B_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Vector_A is less than Vector_B"]
    #[inline(always)]
    pub fn a_less_than_b(&self) -> A_LESS_THAN_B_R {
        A_LESS_THAN_B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vector_A is greater than Vector_B"]
    #[inline(always)]
    pub fn a_greater_than_b(&self) -> A_GREATER_THAN_B_R {
        A_GREATER_THAN_B_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the run bit in the PKA_FUNCTION register is reset at the end of that operation. Status after a complex sequencer operation is unknown\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compare::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPARE_SPEC;
impl crate::RegisterSpec for COMPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compare::R`](R) reader structure"]
impl crate::Readable for COMPARE_SPEC {}
#[doc = "`reset()` method sets COMPARE to value 0"]
impl crate::Resettable for COMPARE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
