#[doc = "Register `IS` reader"]
pub type R = crate::R<IS_SPEC>;
#[doc = "Register `IS` writer"]
pub type W = crate::W<IS_SPEC>;
#[doc = "Field `IS` reader - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
pub type IS_R = crate::FieldReader;
#[doc = "Field `IS` writer - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
pub type IS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
    #[inline(always)]
    #[must_use]
    pub fn is(&mut self) -> IS_W<IS_SPEC> {
        IS_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The IS register is the interrupt sense register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IS_SPEC;
impl crate::RegisterSpec for IS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is::R`](R) reader structure"]
impl crate::Readable for IS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`is::W`](W) writer structure"]
impl crate::Writable for IS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for IS_SPEC {
    const RESET_VALUE: u32 = 0;
}
