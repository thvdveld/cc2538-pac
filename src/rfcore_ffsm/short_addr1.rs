#[doc = "Register `SHORT_ADDR1` reader"]
pub type R = crate::R<SHORT_ADDR1_SPEC>;
#[doc = "Register `SHORT_ADDR1` writer"]
pub type W = crate::W<SHORT_ADDR1_SPEC>;
#[doc = "Field `SHORT_ADDR1` reader - SHORT_ADDR\\[15:8\\]
The short address used during destination address filtering"]
pub type SHORT_ADDR1_R = crate::FieldReader;
#[doc = "Field `SHORT_ADDR1` writer - SHORT_ADDR\\[15:8\\]
The short address used during destination address filtering"]
pub type SHORT_ADDR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[15:8\\]
The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr1(&self) -> SHORT_ADDR1_R {
        SHORT_ADDR1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[15:8\\]
The short address used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr1(&mut self) -> SHORT_ADDR1_W<SHORT_ADDR1_SPEC, 0> {
        SHORT_ADDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHORT_ADDR1_SPEC;
impl crate::RegisterSpec for SHORT_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`short_addr1::R`](R) reader structure"]
impl crate::Readable for SHORT_ADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`short_addr1::W`](W) writer structure"]
impl crate::Writable for SHORT_ADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHORT_ADDR1 to value 0"]
impl crate::Resettable for SHORT_ADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
