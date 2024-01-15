#[doc = "Register `PC2_OVER` reader"]
pub type R = crate::R<PC2_OVER_SPEC>;
#[doc = "Register `PC2_OVER` writer"]
pub type W = crate::W<PC2_OVER_SPEC>;
#[doc = "Field `PC2_over` reader - 0: output disable 1: oe - output enable"]
pub type PC2_OVER_R = crate::BitReader;
#[doc = "Field `PC2_over` writer - 0: output disable 1: oe - output enable"]
pub type PC2_OVER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc2_over(&self) -> PC2_OVER_R {
        PC2_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pc2_over(&mut self) -> PC2_OVER_W<PC2_OVER_SPEC> {
        PC2_OVER_W::new(self, 3)
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
#[doc = "This is the overide configuration register for each pad. PC2 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc2_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc2_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC2_OVER_SPEC;
impl crate::RegisterSpec for PC2_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc2_over::R`](R) reader structure"]
impl crate::Readable for PC2_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc2_over::W`](W) writer structure"]
impl crate::Writable for PC2_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC2_OVER to value 0x04"]
impl crate::Resettable for PC2_OVER_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
