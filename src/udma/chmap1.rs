#[doc = "Register `CHMAP1` reader"]
pub type R = crate::R<Chmap1Spec>;
#[doc = "Register `CHMAP1` writer"]
pub type W = crate::W<Chmap1Spec>;
#[doc = "Field `CH8SEL` reader - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch8selR = crate::FieldReader;
#[doc = "Field `CH8SEL` writer - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch8selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH9SEL` reader - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch9selR = crate::FieldReader;
#[doc = "Field `CH9SEL` writer - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch9selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH10SEL` reader - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch10selR = crate::FieldReader;
#[doc = "Field `CH10SEL` writer - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch10selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH11SEL` reader - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch11selR = crate::FieldReader;
#[doc = "Field `CH11SEL` writer - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch11selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH12SEL` reader - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch12selR = crate::FieldReader;
#[doc = "Field `CH12SEL` writer - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch12selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH13SEL` reader - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch13selR = crate::FieldReader;
#[doc = "Field `CH13SEL` writer - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch13selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH14SEL` reader - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch14selR = crate::FieldReader;
#[doc = "Field `CH14SEL` writer - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch14selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH15SEL` reader - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch15selR = crate::FieldReader;
#[doc = "Field `CH15SEL` writer - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch15selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch8sel(&self) -> Ch8selR {
        Ch8selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch9sel(&self) -> Ch9selR {
        Ch9selR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch10sel(&self) -> Ch10selR {
        Ch10selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch11sel(&self) -> Ch11selR {
        Ch11selR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch12sel(&self) -> Ch12selR {
        Ch12selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch13sel(&self) -> Ch13selR {
        Ch13selR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch14sel(&self) -> Ch14selR {
        Ch14selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch15sel(&self) -> Ch15selR {
        Ch15selR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch8sel(&mut self) -> Ch8selW<Chmap1Spec> {
        Ch8selW::new(self, 0)
    }
    #[doc = "Bits 4:7 - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch9sel(&mut self) -> Ch9selW<Chmap1Spec> {
        Ch9selW::new(self, 4)
    }
    #[doc = "Bits 8:11 - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch10sel(&mut self) -> Ch10selW<Chmap1Spec> {
        Ch10selW::new(self, 8)
    }
    #[doc = "Bits 12:15 - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch11sel(&mut self) -> Ch11selW<Chmap1Spec> {
        Ch11selW::new(self, 12)
    }
    #[doc = "Bits 16:19 - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch12sel(&mut self) -> Ch12selW<Chmap1Spec> {
        Ch12selW::new(self, 16)
    }
    #[doc = "Bits 20:23 - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch13sel(&mut self) -> Ch13selW<Chmap1Spec> {
        Ch13selW::new(self, 20)
    }
    #[doc = "Bits 24:27 - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch14sel(&mut self) -> Ch14selW<Chmap1Spec> {
        Ch14selW::new(self, 24)
    }
    #[doc = "Bits 28:31 - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch15sel(&mut self) -> Ch15selW<Chmap1Spec> {
        Ch15selW::new(self, 28)
    }
}
#[doc = "DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmap1Spec;
impl crate::RegisterSpec for Chmap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap1::R`](R) reader structure"]
impl crate::Readable for Chmap1Spec {}
#[doc = "`write(|w| ..)` method takes [`chmap1::W`](W) writer structure"]
impl crate::Writable for Chmap1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHMAP1 to value 0"]
impl crate::Resettable for Chmap1Spec {
    const RESET_VALUE: u32 = 0;
}
