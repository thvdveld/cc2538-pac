#[doc = "Register `TBPMR` reader"]
pub type R = crate::R<TbpmrSpec>;
#[doc = "Register `TBPMR` writer"]
pub type W = crate::W<TbpmrSpec>;
#[doc = "Field `TBPSR` reader - GPTM Timer B prescale match"]
pub type TbpsrR = crate::FieldReader;
#[doc = "Field `TBPSR` writer - GPTM Timer B prescale match"]
pub type TbpsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer B prescale match"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TbpsrR {
        TbpsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B prescale match"]
    #[inline(always)]
    pub fn tbpsr(&mut self) -> TbpsrW<TbpmrSpec> {
        TbpsrW::new(self, 0)
    }
}
#[doc = "GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbpmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbpmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbpmrSpec;
impl crate::RegisterSpec for TbpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpmr::R`](R) reader structure"]
impl crate::Readable for TbpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbpmr::W`](W) writer structure"]
impl crate::Writable for TbpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBPMR to value 0"]
impl crate::Resettable for TbpmrSpec {
    const RESET_VALUE: u32 = 0;
}
