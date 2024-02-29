#[doc = "Register `PC0_OVER` reader"]
pub type R = crate::R<Pc0OverSpec>;
#[doc = "Register `PC0_OVER` writer"]
pub type W = crate::W<Pc0OverSpec>;
#[doc = "Field `PC0_over` reader - 0: output disable 1: oe - output enable"]
pub type Pc0OverR = crate::BitReader;
#[doc = "Field `PC0_over` writer - 0: output disable 1: oe - output enable"]
pub type Pc0OverW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc0_over(&self) -> Pc0OverR {
        Pc0OverR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pc0_over(&mut self) -> Pc0OverW<Pc0OverSpec> {
        Pc0OverW::new(self, 3)
    }
}
#[doc = "This is the overide configuration register for each pad. PC0 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc0_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc0_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc0OverSpec;
impl crate::RegisterSpec for Pc0OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc0_over::R`](R) reader structure"]
impl crate::Readable for Pc0OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pc0_over::W`](W) writer structure"]
impl crate::Writable for Pc0OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC0_OVER to value 0x04"]
impl crate::Resettable for Pc0OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
