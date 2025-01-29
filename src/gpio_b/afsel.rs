#[doc = "Register `AFSEL` reader"]
pub type R = crate::R<AfselSpec>;
#[doc = "Register `AFSEL` writer"]
pub type W = crate::W<AfselSpec>;
#[doc = "Field `AFSEL` reader - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
pub type AfselR = crate::FieldReader;
#[doc = "Field `AFSEL` writer - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
pub type AfselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
    #[inline(always)]
    pub fn afsel(&self) -> AfselR {
        AfselR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
    #[inline(always)]
    pub fn afsel(&mut self) -> AfselW<AfselSpec> {
        AfselW::new(self, 0)
    }
}
#[doc = "The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default.\n\nYou can [`read`](crate::Reg::read) this register and get [`afsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfselSpec;
impl crate::RegisterSpec for AfselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel::R`](R) reader structure"]
impl crate::Readable for AfselSpec {}
#[doc = "`write(|w| ..)` method takes [`afsel::W`](W) writer structure"]
impl crate::Writable for AfselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFSEL to value 0"]
impl crate::Resettable for AfselSpec {
    const RESET_VALUE: u32 = 0;
}
