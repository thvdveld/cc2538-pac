#[doc = "Register `SHORT_ADDR0` reader"]
pub type R = crate::R<ShortAddr0Spec>;
#[doc = "Register `SHORT_ADDR0` writer"]
pub type W = crate::W<ShortAddr0Spec>;
#[doc = "Field `SHORT_ADDR0` reader - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
pub type ShortAddr0R = crate::FieldReader;
#[doc = "Field `SHORT_ADDR0` writer - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
pub type ShortAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr0(&self) -> ShortAddr0R {
        ShortAddr0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr0(&mut self) -> ShortAddr0W<ShortAddr0Spec> {
        ShortAddr0W::new(self, 0)
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortAddr0Spec;
impl crate::RegisterSpec for ShortAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`short_addr0::R`](R) reader structure"]
impl crate::Readable for ShortAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`short_addr0::W`](W) writer structure"]
impl crate::Writable for ShortAddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORT_ADDR0 to value 0"]
impl crate::Resettable for ShortAddr0Spec {
    const RESET_VALUE: u32 = 0;
}
