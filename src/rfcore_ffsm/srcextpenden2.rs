#[doc = "Register `SRCEXTPENDEN2` reader"]
pub type R = crate::R<SRCEXTPENDEN2_SPEC>;
#[doc = "Register `SRCEXTPENDEN2` writer"]
pub type W = crate::W<SRCEXTPENDEN2_SPEC>;
#[doc = "Field `SRCEXTPENDEN2` reader - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type SRCEXTPENDEN2_R = crate::FieldReader;
#[doc = "Field `SRCEXTPENDEN2` writer - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type SRCEXTPENDEN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden2(&self) -> SRCEXTPENDEN2_R {
        SRCEXTPENDEN2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    #[must_use]
    pub fn srcextpenden2(&mut self) -> SRCEXTPENDEN2_W<SRCEXTPENDEN2_SPEC, 0> {
        SRCEXTPENDEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCEXTPENDEN2_SPEC;
impl crate::RegisterSpec for SRCEXTPENDEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcextpenden2::R`](R) reader structure"]
impl crate::Readable for SRCEXTPENDEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcextpenden2::W`](W) writer structure"]
impl crate::Writable for SRCEXTPENDEN2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCEXTPENDEN2 to value 0"]
impl crate::Resettable for SRCEXTPENDEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
