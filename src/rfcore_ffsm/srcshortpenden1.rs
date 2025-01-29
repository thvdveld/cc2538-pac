#[doc = "Register `SRCSHORTPENDEN1` reader"]
pub type R = crate::R<Srcshortpenden1Spec>;
#[doc = "Register `SRCSHORTPENDEN1` writer"]
pub type W = crate::W<Srcshortpenden1Spec>;
#[doc = "Field `SRCSHORTPENDEN1` reader - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type Srcshortpenden1R = crate::FieldReader;
#[doc = "Field `SRCSHORTPENDEN1` writer - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type Srcshortpenden1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden1(&self) -> Srcshortpenden1R {
        Srcshortpenden1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden1(&mut self) -> Srcshortpenden1W<Srcshortpenden1Spec> {
        Srcshortpenden1W::new(self, 0)
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcshortpenden1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcshortpenden1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcshortpenden1Spec;
impl crate::RegisterSpec for Srcshortpenden1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshortpenden1::R`](R) reader structure"]
impl crate::Readable for Srcshortpenden1Spec {}
#[doc = "`write(|w| ..)` method takes [`srcshortpenden1::W`](W) writer structure"]
impl crate::Writable for Srcshortpenden1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCSHORTPENDEN1 to value 0"]
impl crate::Resettable for Srcshortpenden1Spec {
    const RESET_VALUE: u32 = 0;
}
