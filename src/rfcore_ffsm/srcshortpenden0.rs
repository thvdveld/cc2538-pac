#[doc = "Register `SRCSHORTPENDEN0` reader"]
pub type R = crate::R<Srcshortpenden0Spec>;
#[doc = "Register `SRCSHORTPENDEN0` writer"]
pub type W = crate::W<Srcshortpenden0Spec>;
#[doc = "Field `SRCSHORTPENDEN0` reader - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type Srcshortpenden0R = crate::FieldReader;
#[doc = "Field `SRCSHORTPENDEN0` writer - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type Srcshortpenden0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden0(&self) -> Srcshortpenden0R {
        Srcshortpenden0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden0(&mut self) -> Srcshortpenden0W<Srcshortpenden0Spec> {
        Srcshortpenden0W::new(self, 0)
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcshortpenden0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcshortpenden0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcshortpenden0Spec;
impl crate::RegisterSpec for Srcshortpenden0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshortpenden0::R`](R) reader structure"]
impl crate::Readable for Srcshortpenden0Spec {}
#[doc = "`write(|w| ..)` method takes [`srcshortpenden0::W`](W) writer structure"]
impl crate::Writable for Srcshortpenden0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCSHORTPENDEN0 to value 0"]
impl crate::Resettable for Srcshortpenden0Spec {
    const RESET_VALUE: u32 = 0;
}
