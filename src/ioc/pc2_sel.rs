#[doc = "Register `PC2_SEL` reader"]
pub type R = crate::R<Pc2SelSpec>;
#[doc = "Register `PC2_SEL` writer"]
pub type W = crate::W<Pc2SelSpec>;
#[doc = "Field `PC2_sel` reader - Select one peripheral signal output for PC2."]
pub type Pc2SelR = crate::FieldReader;
#[doc = "Field `PC2_sel` writer - Select one peripheral signal output for PC2."]
pub type Pc2SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC2."]
    #[inline(always)]
    pub fn pc2_sel(&self) -> Pc2SelR {
        Pc2SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC2."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_sel(&mut self) -> Pc2SelW<Pc2SelSpec> {
        Pc2SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PC2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc2_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc2_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc2SelSpec;
impl crate::RegisterSpec for Pc2SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc2_sel::R`](R) reader structure"]
impl crate::Readable for Pc2SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pc2_sel::W`](W) writer structure"]
impl crate::Writable for Pc2SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC2_SEL to value 0"]
impl crate::Resettable for Pc2SelSpec {
    const RESET_VALUE: u32 = 0;
}
