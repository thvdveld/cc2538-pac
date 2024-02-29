#[doc = "Register `SRCEXTPENDEN1` reader"]
pub type R = crate::R<Srcextpenden1Spec>;
#[doc = "Register `SRCEXTPENDEN1` writer"]
pub type W = crate::W<Srcextpenden1Spec>;
#[doc = "Field `SRCEXTPENDEN1` reader - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type Srcextpenden1R = crate::FieldReader;
#[doc = "Field `SRCEXTPENDEN1` writer - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type Srcextpenden1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden1(&self) -> Srcextpenden1R {
        Srcextpenden1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    #[must_use]
    pub fn srcextpenden1(&mut self) -> Srcextpenden1W<Srcextpenden1Spec> {
        Srcextpenden1W::new(self, 0)
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcextpenden1Spec;
impl crate::RegisterSpec for Srcextpenden1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcextpenden1::R`](R) reader structure"]
impl crate::Readable for Srcextpenden1Spec {}
#[doc = "`write(|w| ..)` method takes [`srcextpenden1::W`](W) writer structure"]
impl crate::Writable for Srcextpenden1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCEXTPENDEN1 to value 0"]
impl crate::Resettable for Srcextpenden1Spec {
    const RESET_VALUE: u32 = 0;
}
