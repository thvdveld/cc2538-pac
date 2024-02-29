#[doc = "Register `PC4_SEL` reader"]
pub type R = crate::R<Pc4SelSpec>;
#[doc = "Register `PC4_SEL` writer"]
pub type W = crate::W<Pc4SelSpec>;
#[doc = "Field `PC4_sel` reader - Select one peripheral signal output for PC4."]
pub type Pc4SelR = crate::FieldReader;
#[doc = "Field `PC4_sel` writer - Select one peripheral signal output for PC4."]
pub type Pc4SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC4."]
    #[inline(always)]
    pub fn pc4_sel(&self) -> Pc4SelR {
        Pc4SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC4."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_sel(&mut self) -> Pc4SelW<Pc4SelSpec> {
        Pc4SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PC4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc4_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc4_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc4SelSpec;
impl crate::RegisterSpec for Pc4SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc4_sel::R`](R) reader structure"]
impl crate::Readable for Pc4SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pc4_sel::W`](W) writer structure"]
impl crate::Writable for Pc4SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC4_SEL to value 0"]
impl crate::Resettable for Pc4SelSpec {
    const RESET_VALUE: u32 = 0;
}
