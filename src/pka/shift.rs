#[doc = "Register `SHIFT` reader"]
pub type R = crate::R<SHIFT_SPEC>;
#[doc = "Register `SHIFT` writer"]
pub type W = crate::W<SHIFT_SPEC>;
#[doc = "Field `NUM_BITS_TO_SHIFT` reader - This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
pub type NUM_BITS_TO_SHIFT_R = crate::FieldReader;
#[doc = "Field `NUM_BITS_TO_SHIFT` writer - This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
pub type NUM_BITS_TO_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    pub fn num_bits_to_shift(&self) -> NUM_BITS_TO_SHIFT_R {
        NUM_BITS_TO_SHIFT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    #[must_use]
    pub fn num_bits_to_shift(&mut self) -> NUM_BITS_TO_SHIFT_W<SHIFT_SPEC> {
        NUM_BITS_TO_SHIFT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PKA bit shift value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shift::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shift::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHIFT_SPEC;
impl crate::RegisterSpec for SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shift::R`](R) reader structure"]
impl crate::Readable for SHIFT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shift::W`](W) writer structure"]
impl crate::Writable for SHIFT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFT to value 0"]
impl crate::Resettable for SHIFT_SPEC {
    const RESET_VALUE: u32 = 0;
}
