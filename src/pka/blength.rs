#[doc = "Register `BLENGTH` reader"]
pub type R = crate::R<BLENGTH_SPEC>;
#[doc = "Register `BLENGTH` writer"]
pub type W = crate::W<BLENGTH_SPEC>;
#[doc = "Field `BLENGTH` reader - This register specifies the length (in 32-bit words) of Vector B."]
pub type BLENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `BLENGTH` writer - This register specifies the length (in 32-bit words) of Vector B."]
pub type BLENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - This register specifies the length (in 32-bit words) of Vector B."]
    #[inline(always)]
    pub fn blength(&self) -> BLENGTH_R {
        BLENGTH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register specifies the length (in 32-bit words) of Vector B."]
    #[inline(always)]
    #[must_use]
    pub fn blength(&mut self) -> BLENGTH_W<BLENGTH_SPEC> {
        BLENGTH_W::new(self, 0)
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
#[doc = "PKA vector B length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blength::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blength::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLENGTH_SPEC;
impl crate::RegisterSpec for BLENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blength::R`](R) reader structure"]
impl crate::Readable for BLENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blength::W`](W) writer structure"]
impl crate::Writable for BLENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLENGTH to value 0"]
impl crate::Resettable for BLENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
