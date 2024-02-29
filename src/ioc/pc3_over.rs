#[doc = "Register `PC3_OVER` reader"]
pub type R = crate::R<Pc3OverSpec>;
#[doc = "Register `PC3_OVER` writer"]
pub type W = crate::W<Pc3OverSpec>;
#[doc = "Field `PC3_over` reader - 0: output disable 1: oe - output enable"]
pub type Pc3OverR = crate::BitReader;
#[doc = "Field `PC3_over` writer - 0: output disable 1: oe - output enable"]
pub type Pc3OverW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc3_over(&self) -> Pc3OverR {
        Pc3OverR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pc3_over(&mut self) -> Pc3OverW<Pc3OverSpec> {
        Pc3OverW::new(self, 3)
    }
}
#[doc = "This is the overide configuration register for each pad. PC3 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc3_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc3_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc3OverSpec;
impl crate::RegisterSpec for Pc3OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc3_over::R`](R) reader structure"]
impl crate::Readable for Pc3OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pc3_over::W`](W) writer structure"]
impl crate::Writable for Pc3OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC3_OVER to value 0x04"]
impl crate::Resettable for Pc3OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
