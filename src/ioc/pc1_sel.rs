#[doc = "Register `PC1_SEL` reader"]
pub type R = crate::R<Pc1SelSpec>;
#[doc = "Register `PC1_SEL` writer"]
pub type W = crate::W<Pc1SelSpec>;
#[doc = "Field `PC1_sel` reader - Select one peripheral signal output for PC1."]
pub type Pc1SelR = crate::FieldReader;
#[doc = "Field `PC1_sel` writer - Select one peripheral signal output for PC1."]
pub type Pc1SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC1."]
    #[inline(always)]
    pub fn pc1_sel(&self) -> Pc1SelR {
        Pc1SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC1."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_sel(&mut self) -> Pc1SelW<Pc1SelSpec> {
        Pc1SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc1_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc1_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc1SelSpec;
impl crate::RegisterSpec for Pc1SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc1_sel::R`](R) reader structure"]
impl crate::Readable for Pc1SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pc1_sel::W`](W) writer structure"]
impl crate::Writable for Pc1SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC1_SEL to value 0"]
impl crate::Resettable for Pc1SelSpec {
    const RESET_VALUE: u32 = 0;
}
