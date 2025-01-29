#[doc = "Register `CHMAP2` reader"]
pub type R = crate::R<Chmap2Spec>;
#[doc = "Register `CHMAP2` writer"]
pub type W = crate::W<Chmap2Spec>;
#[doc = "Field `CH16SEL` reader - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch16selR = crate::FieldReader;
#[doc = "Field `CH16SEL` writer - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch16selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH17SEL` reader - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch17selR = crate::FieldReader;
#[doc = "Field `CH17SEL` writer - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch17selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH18SEL` reader - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch18selR = crate::FieldReader;
#[doc = "Field `CH18SEL` writer - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch18selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH19SEL` reader - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch19selR = crate::FieldReader;
#[doc = "Field `CH19SEL` writer - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch19selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH20SEL` reader - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch20selR = crate::FieldReader;
#[doc = "Field `CH20SEL` writer - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch20selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH21SEL` reader - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch21selR = crate::FieldReader;
#[doc = "Field `CH21SEL` writer - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch21selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH22SEL` reader - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch22selR = crate::FieldReader;
#[doc = "Field `CH22SEL` writer - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch22selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH23SEL` reader - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch23selR = crate::FieldReader;
#[doc = "Field `CH23SEL` writer - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch23selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch16sel(&self) -> Ch16selR {
        Ch16selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch17sel(&self) -> Ch17selR {
        Ch17selR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch18sel(&self) -> Ch18selR {
        Ch18selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch19sel(&self) -> Ch19selR {
        Ch19selR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch20sel(&self) -> Ch20selR {
        Ch20selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch21sel(&self) -> Ch21selR {
        Ch21selR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch22sel(&self) -> Ch22selR {
        Ch22selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch23sel(&self) -> Ch23selR {
        Ch23selR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch16sel(&mut self) -> Ch16selW<Chmap2Spec> {
        Ch16selW::new(self, 0)
    }
    #[doc = "Bits 4:7 - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch17sel(&mut self) -> Ch17selW<Chmap2Spec> {
        Ch17selW::new(self, 4)
    }
    #[doc = "Bits 8:11 - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch18sel(&mut self) -> Ch18selW<Chmap2Spec> {
        Ch18selW::new(self, 8)
    }
    #[doc = "Bits 12:15 - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch19sel(&mut self) -> Ch19selW<Chmap2Spec> {
        Ch19selW::new(self, 12)
    }
    #[doc = "Bits 16:19 - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch20sel(&mut self) -> Ch20selW<Chmap2Spec> {
        Ch20selW::new(self, 16)
    }
    #[doc = "Bits 20:23 - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch21sel(&mut self) -> Ch21selW<Chmap2Spec> {
        Ch21selW::new(self, 20)
    }
    #[doc = "Bits 24:27 - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch22sel(&mut self) -> Ch22selW<Chmap2Spec> {
        Ch22selW::new(self, 24)
    }
    #[doc = "Bits 28:31 - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch23sel(&mut self) -> Ch23selW<Chmap2Spec> {
        Ch23selW::new(self, 28)
    }
}
#[doc = "DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::Reg::read) this register and get [`chmap2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmap2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmap2Spec;
impl crate::RegisterSpec for Chmap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap2::R`](R) reader structure"]
impl crate::Readable for Chmap2Spec {}
#[doc = "`write(|w| ..)` method takes [`chmap2::W`](W) writer structure"]
impl crate::Writable for Chmap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHMAP2 to value 0"]
impl crate::Resettable for Chmap2Spec {
    const RESET_VALUE: u32 = 0;
}
