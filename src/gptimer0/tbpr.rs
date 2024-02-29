#[doc = "Register `TBPR` reader"]
pub type R = crate::R<TbprSpec>;
#[doc = "Register `TBPR` writer"]
pub type W = crate::W<TbprSpec>;
#[doc = "Field `TBPSR` reader - GPTM Timer B prescale"]
pub type TbpsrR = crate::FieldReader;
#[doc = "Field `TBPSR` writer - GPTM Timer B prescale"]
pub type TbpsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer B prescale"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TbpsrR {
        TbpsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B prescale"]
    #[inline(always)]
    #[must_use]
    pub fn tbpsr(&mut self) -> TbpsrW<TbprSpec> {
        TbpsrW::new(self, 0)
    }
}
#[doc = "GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbprSpec;
impl crate::RegisterSpec for TbprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpr::R`](R) reader structure"]
impl crate::Readable for TbprSpec {}
#[doc = "`write(|w| ..)` method takes [`tbpr::W`](W) writer structure"]
impl crate::Writable for TbprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBPR to value 0"]
impl crate::Resettable for TbprSpec {
    const RESET_VALUE: u32 = 0;
}
