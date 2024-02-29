#[doc = "Register `PC6_SEL` reader"]
pub type R = crate::R<Pc6SelSpec>;
#[doc = "Register `PC6_SEL` writer"]
pub type W = crate::W<Pc6SelSpec>;
#[doc = "Field `PC6_sel` reader - Select one peripheral signal output for PC6."]
pub type Pc6SelR = crate::FieldReader;
#[doc = "Field `PC6_sel` writer - Select one peripheral signal output for PC6."]
pub type Pc6SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC6."]
    #[inline(always)]
    pub fn pc6_sel(&self) -> Pc6SelR {
        Pc6SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC6."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_sel(&mut self) -> Pc6SelW<Pc6SelSpec> {
        Pc6SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PC6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc6_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc6_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc6SelSpec;
impl crate::RegisterSpec for Pc6SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc6_sel::R`](R) reader structure"]
impl crate::Readable for Pc6SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pc6_sel::W`](W) writer structure"]
impl crate::Writable for Pc6SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC6_SEL to value 0"]
impl crate::Resettable for Pc6SelSpec {
    const RESET_VALUE: u32 = 0;
}
