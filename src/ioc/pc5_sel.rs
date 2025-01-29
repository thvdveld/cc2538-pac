#[doc = "Register `PC5_SEL` reader"]
pub type R = crate::R<Pc5SelSpec>;
#[doc = "Register `PC5_SEL` writer"]
pub type W = crate::W<Pc5SelSpec>;
#[doc = "Field `PC5_sel` reader - Select one peripheral signal output for PC5."]
pub type Pc5SelR = crate::FieldReader;
#[doc = "Field `PC5_sel` writer - Select one peripheral signal output for PC5."]
pub type Pc5SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC5."]
    #[inline(always)]
    pub fn pc5_sel(&self) -> Pc5SelR {
        Pc5SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC5."]
    #[inline(always)]
    pub fn pc5_sel(&mut self) -> Pc5SelW<Pc5SelSpec> {
        Pc5SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PC5\n\nYou can [`read`](crate::Reg::read) this register and get [`pc5_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc5_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc5SelSpec;
impl crate::RegisterSpec for Pc5SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc5_sel::R`](R) reader structure"]
impl crate::Readable for Pc5SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pc5_sel::W`](W) writer structure"]
impl crate::Writable for Pc5SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC5_SEL to value 0"]
impl crate::Resettable for Pc5SelSpec {
    const RESET_VALUE: u32 = 0;
}
