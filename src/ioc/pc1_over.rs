#[doc = "Register `PC1_OVER` reader"]
pub type R = crate::R<Pc1OverSpec>;
#[doc = "Register `PC1_OVER` writer"]
pub type W = crate::W<Pc1OverSpec>;
#[doc = "Field `PC1_over` reader - 0: output disable 1: oe - output enable"]
pub type Pc1OverR = crate::BitReader;
#[doc = "Field `PC1_over` writer - 0: output disable 1: oe - output enable"]
pub type Pc1OverW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc1_over(&self) -> Pc1OverR {
        Pc1OverR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc1_over(&mut self) -> Pc1OverW<Pc1OverSpec> {
        Pc1OverW::new(self, 3)
    }
}
#[doc = "This is the overide configuration register for each pad. PC1 has high drive capability.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc1_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc1_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc1OverSpec;
impl crate::RegisterSpec for Pc1OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc1_over::R`](R) reader structure"]
impl crate::Readable for Pc1OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pc1_over::W`](W) writer structure"]
impl crate::Writable for Pc1OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC1_OVER to value 0x04"]
impl crate::Resettable for Pc1OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
