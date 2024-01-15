#[doc = "Register `SHORT_ADDR0` reader"]
pub type R = crate::R<SHORT_ADDR0_SPEC>;
#[doc = "Register `SHORT_ADDR0` writer"]
pub type W = crate::W<SHORT_ADDR0_SPEC>;
#[doc = "Field `SHORT_ADDR0` reader - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
pub type SHORT_ADDR0_R = crate::FieldReader;
#[doc = "Field `SHORT_ADDR0` writer - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
pub type SHORT_ADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr0(&self) -> SHORT_ADDR0_R {
        SHORT_ADDR0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr0(&mut self) -> SHORT_ADDR0_W<SHORT_ADDR0_SPEC> {
        SHORT_ADDR0_W::new(self, 0)
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
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHORT_ADDR0_SPEC;
impl crate::RegisterSpec for SHORT_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`short_addr0::R`](R) reader structure"]
impl crate::Readable for SHORT_ADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`short_addr0::W`](W) writer structure"]
impl crate::Writable for SHORT_ADDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORT_ADDR0 to value 0"]
impl crate::Resettable for SHORT_ADDR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
