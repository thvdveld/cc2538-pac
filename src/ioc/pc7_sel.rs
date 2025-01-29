#[doc = "Register `PC7_SEL` reader"]
pub type R = crate::R<Pc7SelSpec>;
#[doc = "Register `PC7_SEL` writer"]
pub type W = crate::W<Pc7SelSpec>;
#[doc = "Field `PC7_sel` reader - Select one peripheral signal output for PC7."]
pub type Pc7SelR = crate::FieldReader;
#[doc = "Field `PC7_sel` writer - Select one peripheral signal output for PC7."]
pub type Pc7SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC7."]
    #[inline(always)]
    pub fn pc7_sel(&self) -> Pc7SelR {
        Pc7SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC7."]
    #[inline(always)]
    pub fn pc7_sel(&mut self) -> Pc7SelW<Pc7SelSpec> {
        Pc7SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PC7\n\nYou can [`read`](crate::Reg::read) this register and get [`pc7_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc7_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc7SelSpec;
impl crate::RegisterSpec for Pc7SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc7_sel::R`](R) reader structure"]
impl crate::Readable for Pc7SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pc7_sel::W`](W) writer structure"]
impl crate::Writable for Pc7SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC7_SEL to value 0"]
impl crate::Resettable for Pc7SelSpec {
    const RESET_VALUE: u32 = 0;
}
