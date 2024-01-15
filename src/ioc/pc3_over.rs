#[doc = "Register `PC3_OVER` reader"]
pub type R = crate::R<PC3_OVER_SPEC>;
#[doc = "Register `PC3_OVER` writer"]
pub type W = crate::W<PC3_OVER_SPEC>;
#[doc = "Field `PC3_over` reader - 0: output disable 1: oe - output enable"]
pub type PC3_OVER_R = crate::BitReader;
#[doc = "Field `PC3_over` writer - 0: output disable 1: oe - output enable"]
pub type PC3_OVER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc3_over(&self) -> PC3_OVER_R {
        PC3_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pc3_over(&mut self) -> PC3_OVER_W<PC3_OVER_SPEC> {
        PC3_OVER_W::new(self, 3)
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
#[doc = "This is the overide configuration register for each pad. PC3 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc3_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc3_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC3_OVER_SPEC;
impl crate::RegisterSpec for PC3_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc3_over::R`](R) reader structure"]
impl crate::Readable for PC3_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc3_over::W`](W) writer structure"]
impl crate::Writable for PC3_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC3_OVER to value 0x04"]
impl crate::Resettable for PC3_OVER_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
