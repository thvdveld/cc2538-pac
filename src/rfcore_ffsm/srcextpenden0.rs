#[doc = "Register `SRCEXTPENDEN0` reader"]
pub type R = crate::R<Srcextpenden0Spec>;
#[doc = "Register `SRCEXTPENDEN0` writer"]
pub type W = crate::W<Srcextpenden0Spec>;
#[doc = "Field `SRCEXTPENDEN0` reader - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type Srcextpenden0R = crate::FieldReader;
#[doc = "Field `SRCEXTPENDEN0` writer - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type Srcextpenden0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden0(&self) -> Srcextpenden0R {
        Srcextpenden0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden0(&mut self) -> Srcextpenden0W<Srcextpenden0Spec> {
        Srcextpenden0W::new(self, 0)
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcextpenden0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcextpenden0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcextpenden0Spec;
impl crate::RegisterSpec for Srcextpenden0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcextpenden0::R`](R) reader structure"]
impl crate::Readable for Srcextpenden0Spec {}
#[doc = "`write(|w| ..)` method takes [`srcextpenden0::W`](W) writer structure"]
impl crate::Writable for Srcextpenden0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCEXTPENDEN0 to value 0"]
impl crate::Resettable for Srcextpenden0Spec {
    const RESET_VALUE: u32 = 0;
}
