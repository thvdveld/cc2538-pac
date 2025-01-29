#[doc = "Register `IIE` reader"]
pub type R = crate::R<IieSpec>;
#[doc = "Register `IIE` writer"]
pub type W = crate::W<IieSpec>;
#[doc = "Field `EP0IE` reader - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
pub type Ep0ieR = crate::BitReader;
#[doc = "Field `EP0IE` writer - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
pub type Ep0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEP1IE` reader - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep1ieR = crate::BitReader;
#[doc = "Field `INEP1IE` writer - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEP2IE` reader - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep2ieR = crate::BitReader;
#[doc = "Field `INEP2IE` writer - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEP3IE` reader - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep3ieR = crate::BitReader;
#[doc = "Field `INEP3IE` writer - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEP4IE` reader - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep4ieR = crate::BitReader;
#[doc = "Field `INEP4IE` writer - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEP5IE` reader - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep5ieR = crate::BitReader;
#[doc = "Field `INEP5IE` writer - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type Inep5ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn ep0ie(&self) -> Ep0ieR {
        Ep0ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep1ie(&self) -> Inep1ieR {
        Inep1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep2ie(&self) -> Inep2ieR {
        Inep2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep3ie(&self) -> Inep3ieR {
        Inep3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep4ie(&self) -> Inep4ieR {
        Inep4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep5ie(&self) -> Inep5ieR {
        Inep5ieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn ep0ie(&mut self) -> Ep0ieW<IieSpec> {
        Ep0ieW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep1ie(&mut self) -> Inep1ieW<IieSpec> {
        Inep1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep2ie(&mut self) -> Inep2ieW<IieSpec> {
        Inep2ieW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep3ie(&mut self) -> Inep3ieW<IieSpec> {
        Inep3ieW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep4ie(&mut self) -> Inep4ieW<IieSpec> {
        Inep4ieW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep5ie(&mut self) -> Inep5ieW<IieSpec> {
        Inep5ieW::new(self, 5)
    }
}
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`iie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IieSpec;
impl crate::RegisterSpec for IieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iie::R`](R) reader structure"]
impl crate::Readable for IieSpec {}
#[doc = "`write(|w| ..)` method takes [`iie::W`](W) writer structure"]
impl crate::Writable for IieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IIE to value 0"]
impl crate::Resettable for IieSpec {
    const RESET_VALUE: u32 = 0;
}
