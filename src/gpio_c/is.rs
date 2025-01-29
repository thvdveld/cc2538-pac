#[doc = "Register `IS` reader"]
pub type R = crate::R<IsSpec>;
#[doc = "Register `IS` writer"]
pub type W = crate::W<IsSpec>;
#[doc = "Field `IS` reader - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
pub type IsR = crate::FieldReader;
#[doc = "Field `IS` writer - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
pub type IsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
    #[inline(always)]
    pub fn is(&self) -> IsR {
        IsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
    #[inline(always)]
    pub fn is(&mut self) -> IsW<IsSpec> {
        IsW::new(self, 0)
    }
}
#[doc = "The IS register is the interrupt sense register.\n\nYou can [`read`](crate::Reg::read) this register and get [`is::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`is::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsSpec;
impl crate::RegisterSpec for IsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is::R`](R) reader structure"]
impl crate::Readable for IsSpec {}
#[doc = "`write(|w| ..)` method takes [`is::W`](W) writer structure"]
impl crate::Writable for IsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for IsSpec {
    const RESET_VALUE: u32 = 0;
}
