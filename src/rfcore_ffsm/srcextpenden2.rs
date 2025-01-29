#[doc = "Register `SRCEXTPENDEN2` reader"]
pub type R = crate::R<Srcextpenden2Spec>;
#[doc = "Register `SRCEXTPENDEN2` writer"]
pub type W = crate::W<Srcextpenden2Spec>;
#[doc = "Field `SRCEXTPENDEN2` reader - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type Srcextpenden2R = crate::FieldReader;
#[doc = "Field `SRCEXTPENDEN2` writer - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type Srcextpenden2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden2(&self) -> Srcextpenden2R {
        Srcextpenden2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden2(&mut self) -> Srcextpenden2W<Srcextpenden2Spec> {
        Srcextpenden2W::new(self, 0)
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcextpenden2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcextpenden2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcextpenden2Spec;
impl crate::RegisterSpec for Srcextpenden2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcextpenden2::R`](R) reader structure"]
impl crate::Readable for Srcextpenden2Spec {}
#[doc = "`write(|w| ..)` method takes [`srcextpenden2::W`](W) writer structure"]
impl crate::Writable for Srcextpenden2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCEXTPENDEN2 to value 0"]
impl crate::Resettable for Srcextpenden2Spec {
    const RESET_VALUE: u32 = 0;
}
