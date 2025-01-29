#[doc = "Register `SRCSHORTPENDEN2` reader"]
pub type R = crate::R<Srcshortpenden2Spec>;
#[doc = "Register `SRCSHORTPENDEN2` writer"]
pub type W = crate::W<Srcshortpenden2Spec>;
#[doc = "Field `SRCSHORTPENDEN2` reader - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type Srcshortpenden2R = crate::FieldReader;
#[doc = "Field `SRCSHORTPENDEN2` writer - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type Srcshortpenden2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden2(&self) -> Srcshortpenden2R {
        Srcshortpenden2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden2(&mut self) -> Srcshortpenden2W<Srcshortpenden2Spec> {
        Srcshortpenden2W::new(self, 0)
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcshortpenden2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcshortpenden2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcshortpenden2Spec;
impl crate::RegisterSpec for Srcshortpenden2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshortpenden2::R`](R) reader structure"]
impl crate::Readable for Srcshortpenden2Spec {}
#[doc = "`write(|w| ..)` method takes [`srcshortpenden2::W`](W) writer structure"]
impl crate::Writable for Srcshortpenden2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCSHORTPENDEN2 to value 0"]
impl crate::Resettable for Srcshortpenden2Spec {
    const RESET_VALUE: u32 = 0;
}
