#[doc = "Register `STCS` reader"]
pub type R = crate::R<STCS_SPEC>;
#[doc = "Register `STCS` writer"]
pub type W = crate::W<STCS_SPEC>;
#[doc = "Field `VALID` reader - Capture valid flag Set to 1 when capture value in STCV has been updated Clear explicitly to allow new capture"]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID` writer - Capture valid flag Set to 1 when capture value in STCV has been updated Clear explicitly to allow new capture"]
pub type VALID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Capture valid flag Set to 1 when capture value in STCV has been updated Clear explicitly to allow new capture"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture valid flag Set to 1 when capture value in STCV has been updated Clear explicitly to allow new capture"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<STCS_SPEC, 0> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sleep Timer Capture status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCS_SPEC;
impl crate::RegisterSpec for STCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcs::R`](R) reader structure"]
impl crate::Readable for STCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stcs::W`](W) writer structure"]
impl crate::Writable for STCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCS to value 0"]
impl crate::Resettable for STCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
