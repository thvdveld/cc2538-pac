#[doc = "Register `IC` writer"]
pub type W = crate::W<IcSpec>;
#[doc = "Field `IC` writer - Bit written as 1: Clears edge detection logic Bit written as 0: Has no effect"]
pub type IcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Bit written as 1: Clears edge detection logic Bit written as 0: Has no effect"]
    #[inline(always)]
    pub fn ic(&mut self) -> IcW<IcSpec> {
        IcW::new(self, 0)
    }
}
#[doc = "The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSpec;
impl crate::RegisterSpec for IcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ic::W`](W) writer structure"]
impl crate::Writable for IcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IcSpec {
    const RESET_VALUE: u32 = 0;
}
