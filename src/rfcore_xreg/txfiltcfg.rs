#[doc = "Register `TXFILTCFG` reader"]
pub type R = crate::R<TXFILTCFG_SPEC>;
#[doc = "Register `TXFILTCFG` writer"]
pub type W = crate::W<TXFILTCFG_SPEC>;
#[doc = "Field `FC` reader - Drives signal rfr_txfilt_fc"]
pub type FC_R = crate::FieldReader;
#[doc = "Field `FC` writer - Drives signal rfr_txfilt_fc"]
pub type FC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Drives signal rfr_txfilt_fc"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Drives signal rfr_txfilt_fc"]
    #[inline(always)]
    #[must_use]
    pub fn fc(&mut self) -> FC_W<TXFILTCFG_SPEC, 0> {
        FC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TX filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfiltcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfiltcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFILTCFG_SPEC;
impl crate::RegisterSpec for TXFILTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfiltcfg::R`](R) reader structure"]
impl crate::Readable for TXFILTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txfiltcfg::W`](W) writer structure"]
impl crate::Writable for TXFILTCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFILTCFG to value 0"]
impl crate::Resettable for TXFILTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
