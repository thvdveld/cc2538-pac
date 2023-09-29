#[doc = "Register `SRCEXTPENDEN0` reader"]
pub type R = crate::R<SRCEXTPENDEN0_SPEC>;
#[doc = "Register `SRCEXTPENDEN0` writer"]
pub type W = crate::W<SRCEXTPENDEN0_SPEC>;
#[doc = "Field `SRCEXTPENDEN0` reader - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type SRCEXTPENDEN0_R = crate::FieldReader;
#[doc = "Field `SRCEXTPENDEN0` writer - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type SRCEXTPENDEN0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden0(&self) -> SRCEXTPENDEN0_R {
        SRCEXTPENDEN0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    #[must_use]
    pub fn srcextpenden0(&mut self) -> SRCEXTPENDEN0_W<SRCEXTPENDEN0_SPEC, 0> {
        SRCEXTPENDEN0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcextpenden0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcextpenden0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCEXTPENDEN0_SPEC;
impl crate::RegisterSpec for SRCEXTPENDEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcextpenden0::R`](R) reader structure"]
impl crate::Readable for SRCEXTPENDEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcextpenden0::W`](W) writer structure"]
impl crate::Writable for SRCEXTPENDEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCEXTPENDEN0 to value 0"]
impl crate::Resettable for SRCEXTPENDEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
