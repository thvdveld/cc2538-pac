#[doc = "Register `DIR` reader"]
pub type R = crate::R<DirSpec>;
#[doc = "Register `DIR` writer"]
pub type W = crate::W<DirSpec>;
#[doc = "Field `DIR` reader - Bits set: Corresponding pin is output Bits cleared: Corresponding pin is input"]
pub type DirR = crate::FieldReader;
#[doc = "Field `DIR` writer - Bits set: Corresponding pin is output Bits cleared: Corresponding pin is input"]
pub type DirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Corresponding pin is output Bits cleared: Corresponding pin is input"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Corresponding pin is output Bits cleared: Corresponding pin is input"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<DirSpec> {
        DirW::new(self, 0)
    }
}
#[doc = "The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default.\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirSpec;
impl crate::RegisterSpec for DirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DirSpec {}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DirSpec {
    const RESET_VALUE: u32 = 0;
}
