#[doc = "Register `PB2_SEL` reader"]
pub type R = crate::R<Pb2SelSpec>;
#[doc = "Register `PB2_SEL` writer"]
pub type W = crate::W<Pb2SelSpec>;
#[doc = "Field `PB2_sel` reader - Select one peripheral signal output for PB2."]
pub type Pb2SelR = crate::FieldReader;
#[doc = "Field `PB2_sel` writer - Select one peripheral signal output for PB2."]
pub type Pb2SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB2."]
    #[inline(always)]
    pub fn pb2_sel(&self) -> Pb2SelR {
        Pb2SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB2."]
    #[inline(always)]
    pub fn pb2_sel(&mut self) -> Pb2SelW<Pb2SelSpec> {
        Pb2SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PB2\n\nYou can [`read`](crate::Reg::read) this register and get [`pb2_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb2_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb2SelSpec;
impl crate::RegisterSpec for Pb2SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb2_sel::R`](R) reader structure"]
impl crate::Readable for Pb2SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pb2_sel::W`](W) writer structure"]
impl crate::Writable for Pb2SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB2_SEL to value 0"]
impl crate::Resettable for Pb2SelSpec {
    const RESET_VALUE: u32 = 0;
}
