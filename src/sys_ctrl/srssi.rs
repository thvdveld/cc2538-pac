#[doc = "Register `SRSSI` reader"]
pub type R = crate::R<SrssiSpec>;
#[doc = "Register `SRSSI` writer"]
pub type W = crate::W<SrssiSpec>;
#[doc = "Field `SSI0` reader - 0: SSI0 module is not reset 1: SSI0 module is reset"]
pub type Ssi0R = crate::BitReader;
#[doc = "Field `SSI0` writer - 0: SSI0 module is not reset 1: SSI0 module is reset"]
pub type Ssi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI1` reader - 0: SSI1 module is not reset 1: SSI1 module is reset"]
pub type Ssi1R = crate::BitReader;
#[doc = "Field `SSI1` writer - 0: SSI1 module is not reset 1: SSI1 module is reset"]
pub type Ssi1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: SSI0 module is not reset 1: SSI0 module is reset"]
    #[inline(always)]
    pub fn ssi0(&self) -> Ssi0R {
        Ssi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: SSI1 module is not reset 1: SSI1 module is reset"]
    #[inline(always)]
    pub fn ssi1(&self) -> Ssi1R {
        Ssi1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: SSI0 module is not reset 1: SSI0 module is reset"]
    #[inline(always)]
    pub fn ssi0(&mut self) -> Ssi0W<SrssiSpec> {
        Ssi0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: SSI1 module is not reset 1: SSI1 module is reset"]
    #[inline(always)]
    pub fn ssi1(&mut self) -> Ssi1W<SrssiSpec> {
        Ssi1W::new(self, 1)
    }
}
#[doc = "This register controls the reset for SSI\\[1:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`srssi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srssi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrssiSpec;
impl crate::RegisterSpec for SrssiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srssi::R`](R) reader structure"]
impl crate::Readable for SrssiSpec {}
#[doc = "`write(|w| ..)` method takes [`srssi::W`](W) writer structure"]
impl crate::Writable for SrssiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSSI to value 0"]
impl crate::Resettable for SrssiSpec {
    const RESET_VALUE: u32 = 0;
}
