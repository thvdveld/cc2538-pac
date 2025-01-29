#[doc = "Register `COMPARE` reader"]
pub type R = crate::R<CompareSpec>;
#[doc = "Field `A_EQUALS_B` reader - Vector_A is equal to Vector_B"]
pub type AEqualsBR = crate::BitReader;
#[doc = "Field `A_LESS_THAN_B` reader - Vector_A is less than Vector_B"]
pub type ALessThanBR = crate::BitReader;
#[doc = "Field `A_GREATER_THAN_B` reader - Vector_A is greater than Vector_B"]
pub type AGreaterThanBR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Vector_A is equal to Vector_B"]
    #[inline(always)]
    pub fn a_equals_b(&self) -> AEqualsBR {
        AEqualsBR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Vector_A is less than Vector_B"]
    #[inline(always)]
    pub fn a_less_than_b(&self) -> ALessThanBR {
        ALessThanBR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vector_A is greater than Vector_B"]
    #[inline(always)]
    pub fn a_greater_than_b(&self) -> AGreaterThanBR {
        AGreaterThanBR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the run bit in the PKA_FUNCTION register is reset at the end of that operation. Status after a complex sequencer operation is unknown\n\nYou can [`read`](crate::Reg::read) this register and get [`compare::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompareSpec;
impl crate::RegisterSpec for CompareSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compare::R`](R) reader structure"]
impl crate::Readable for CompareSpec {}
#[doc = "`reset()` method sets COMPARE to value 0"]
impl crate::Resettable for CompareSpec {
    const RESET_VALUE: u32 = 0;
}
