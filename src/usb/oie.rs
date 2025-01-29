#[doc = "Register `OIE` reader"]
pub type R = crate::R<OieSpec>;
#[doc = "Register `OIE` writer"]
pub type W = crate::W<OieSpec>;
#[doc = "Field `OUTEP1IE` reader - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep1ieR = crate::BitReader;
#[doc = "Field `OUTEP1IE` writer - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEP2IE` reader - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep2ieR = crate::BitReader;
#[doc = "Field `OUTEP2IE` writer - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEP3IE` reader - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep3ieR = crate::BitReader;
#[doc = "Field `OUTEP3IE` writer - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEP4IE` reader - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep4ieR = crate::BitReader;
#[doc = "Field `OUTEP4IE` writer - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEP5IE` reader - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep5ieR = crate::BitReader;
#[doc = "Field `OUTEP5IE` writer - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type Outep5ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&self) -> Outep1ieR {
        Outep1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&self) -> Outep2ieR {
        Outep2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&self) -> Outep3ieR {
        Outep3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&self) -> Outep4ieR {
        Outep4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&self) -> Outep5ieR {
        Outep5ieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&mut self) -> Outep1ieW<OieSpec> {
        Outep1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&mut self) -> Outep2ieW<OieSpec> {
        Outep2ieW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&mut self) -> Outep3ieW<OieSpec> {
        Outep3ieW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&mut self) -> Outep4ieW<OieSpec> {
        Outep4ieW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&mut self) -> Outep5ieW<OieSpec> {
        Outep5ieW::new(self, 5)
    }
}
#[doc = "Interrupt enable mask for OUT endpoints 1-5\n\nYou can [`read`](crate::Reg::read) this register and get [`oie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OieSpec;
impl crate::RegisterSpec for OieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oie::R`](R) reader structure"]
impl crate::Readable for OieSpec {}
#[doc = "`write(|w| ..)` method takes [`oie::W`](W) writer structure"]
impl crate::Writable for OieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OIE to value 0"]
impl crate::Resettable for OieSpec {
    const RESET_VALUE: u32 = 0;
}
