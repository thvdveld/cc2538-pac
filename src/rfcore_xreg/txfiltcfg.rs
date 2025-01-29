#[doc = "Register `TXFILTCFG` reader"]
pub type R = crate::R<TxfiltcfgSpec>;
#[doc = "Register `TXFILTCFG` writer"]
pub type W = crate::W<TxfiltcfgSpec>;
#[doc = "Field `FC` reader - Drives signal rfr_txfilt_fc"]
pub type FcR = crate::FieldReader;
#[doc = "Field `FC` writer - Drives signal rfr_txfilt_fc"]
pub type FcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Drives signal rfr_txfilt_fc"]
    #[inline(always)]
    pub fn fc(&self) -> FcR {
        FcR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Drives signal rfr_txfilt_fc"]
    #[inline(always)]
    pub fn fc(&mut self) -> FcW<TxfiltcfgSpec> {
        FcW::new(self, 0)
    }
}
#[doc = "TX filter configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txfiltcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfiltcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfiltcfgSpec;
impl crate::RegisterSpec for TxfiltcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfiltcfg::R`](R) reader structure"]
impl crate::Readable for TxfiltcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`txfiltcfg::W`](W) writer structure"]
impl crate::Writable for TxfiltcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFILTCFG to value 0"]
impl crate::Resettable for TxfiltcfgSpec {
    const RESET_VALUE: u32 = 0;
}
