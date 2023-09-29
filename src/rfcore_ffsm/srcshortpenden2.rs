#[doc = "Register `SRCSHORTPENDEN2` reader"]
pub type R = crate::R<SRCSHORTPENDEN2_SPEC>;
#[doc = "Register `SRCSHORTPENDEN2` writer"]
pub type W = crate::W<SRCSHORTPENDEN2_SPEC>;
#[doc = "Field `SRCSHORTPENDEN2` reader - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type SRCSHORTPENDEN2_R = crate::FieldReader;
#[doc = "Field `SRCSHORTPENDEN2` writer - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type SRCSHORTPENDEN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden2(&self) -> SRCSHORTPENDEN2_R {
        SRCSHORTPENDEN2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    #[must_use]
    pub fn srcshortpenden2(&mut self) -> SRCSHORTPENDEN2_W<SRCSHORTPENDEN2_SPEC, 0> {
        SRCSHORTPENDEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshortpenden2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshortpenden2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCSHORTPENDEN2_SPEC;
impl crate::RegisterSpec for SRCSHORTPENDEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshortpenden2::R`](R) reader structure"]
impl crate::Readable for SRCSHORTPENDEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcshortpenden2::W`](W) writer structure"]
impl crate::Writable for SRCSHORTPENDEN2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCSHORTPENDEN2 to value 0"]
impl crate::Resettable for SRCSHORTPENDEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
