#[doc = "Register `ALENGTH` reader"]
pub type R = crate::R<ALENGTH_SPEC>;
#[doc = "Register `ALENGTH` writer"]
pub type W = crate::W<ALENGTH_SPEC>;
#[doc = "Field `ALENGTH` reader - This register specifies the length (in 32-bit words) of Vector A."]
pub type ALENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `ALENGTH` writer - This register specifies the length (in 32-bit words) of Vector A."]
pub type ALENGTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - This register specifies the length (in 32-bit words) of Vector A."]
    #[inline(always)]
    pub fn alength(&self) -> ALENGTH_R {
        ALENGTH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register specifies the length (in 32-bit words) of Vector A."]
    #[inline(always)]
    #[must_use]
    pub fn alength(&mut self) -> ALENGTH_W<ALENGTH_SPEC, 0> {
        ALENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PKA vector A length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alength::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alength::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALENGTH_SPEC;
impl crate::RegisterSpec for ALENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alength::R`](R) reader structure"]
impl crate::Readable for ALENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alength::W`](W) writer structure"]
impl crate::Writable for ALENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALENGTH to value 0"]
impl crate::Resettable for ALENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
