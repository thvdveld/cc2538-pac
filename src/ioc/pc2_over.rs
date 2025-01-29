#[doc = "Register `PC2_OVER` reader"]
pub type R = crate::R<Pc2OverSpec>;
#[doc = "Register `PC2_OVER` writer"]
pub type W = crate::W<Pc2OverSpec>;
#[doc = "Field `PC2_over` reader - 0: output disable 1: oe - output enable"]
pub type Pc2OverR = crate::BitReader;
#[doc = "Field `PC2_over` writer - 0: output disable 1: oe - output enable"]
pub type Pc2OverW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc2_over(&self) -> Pc2OverR {
        Pc2OverR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc2_over(&mut self) -> Pc2OverW<Pc2OverSpec> {
        Pc2OverW::new(self, 3)
    }
}
#[doc = "This is the overide configuration register for each pad. PC2 has high drive capability.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc2_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc2_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc2OverSpec;
impl crate::RegisterSpec for Pc2OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc2_over::R`](R) reader structure"]
impl crate::Readable for Pc2OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pc2_over::W`](W) writer structure"]
impl crate::Writable for Pc2OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC2_OVER to value 0x04"]
impl crate::Resettable for Pc2OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
