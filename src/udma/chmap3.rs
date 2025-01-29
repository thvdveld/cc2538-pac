#[doc = "Register `CHMAP3` reader"]
pub type R = crate::R<Chmap3Spec>;
#[doc = "Register `CHMAP3` writer"]
pub type W = crate::W<Chmap3Spec>;
#[doc = "Field `CH24SEL` reader - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch24selR = crate::FieldReader;
#[doc = "Field `CH24SEL` writer - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch24selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH25SEL` reader - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch25selR = crate::FieldReader;
#[doc = "Field `CH25SEL` writer - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch25selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH26SEL` reader - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch26selR = crate::FieldReader;
#[doc = "Field `CH26SEL` writer - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch26selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH27SEL` reader - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch27selR = crate::FieldReader;
#[doc = "Field `CH27SEL` writer - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch27selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH28SEL` reader - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch28selR = crate::FieldReader;
#[doc = "Field `CH28SEL` writer - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch28selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH29SEL` reader - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch29selR = crate::FieldReader;
#[doc = "Field `CH29SEL` writer - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch29selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH30SEL` reader - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch30selR = crate::FieldReader;
#[doc = "Field `CH30SEL` writer - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch30selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH31SEL` reader - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch31selR = crate::FieldReader;
#[doc = "Field `CH31SEL` writer - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch31selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch24sel(&self) -> Ch24selR {
        Ch24selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch25sel(&self) -> Ch25selR {
        Ch25selR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch26sel(&self) -> Ch26selR {
        Ch26selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch27sel(&self) -> Ch27selR {
        Ch27selR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch28sel(&self) -> Ch28selR {
        Ch28selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch29sel(&self) -> Ch29selR {
        Ch29selR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch30sel(&self) -> Ch30selR {
        Ch30selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch31sel(&self) -> Ch31selR {
        Ch31selR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch24sel(&mut self) -> Ch24selW<Chmap3Spec> {
        Ch24selW::new(self, 0)
    }
    #[doc = "Bits 4:7 - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch25sel(&mut self) -> Ch25selW<Chmap3Spec> {
        Ch25selW::new(self, 4)
    }
    #[doc = "Bits 8:11 - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch26sel(&mut self) -> Ch26selW<Chmap3Spec> {
        Ch26selW::new(self, 8)
    }
    #[doc = "Bits 12:15 - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch27sel(&mut self) -> Ch27selW<Chmap3Spec> {
        Ch27selW::new(self, 12)
    }
    #[doc = "Bits 16:19 - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch28sel(&mut self) -> Ch28selW<Chmap3Spec> {
        Ch28selW::new(self, 16)
    }
    #[doc = "Bits 20:23 - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch29sel(&mut self) -> Ch29selW<Chmap3Spec> {
        Ch29selW::new(self, 20)
    }
    #[doc = "Bits 24:27 - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch30sel(&mut self) -> Ch30selW<Chmap3Spec> {
        Ch30selW::new(self, 24)
    }
    #[doc = "Bits 28:31 - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch31sel(&mut self) -> Ch31selW<Chmap3Spec> {
        Ch31selW::new(self, 28)
    }
}
#[doc = "DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::Reg::read) this register and get [`chmap3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmap3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmap3Spec;
impl crate::RegisterSpec for Chmap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap3::R`](R) reader structure"]
impl crate::Readable for Chmap3Spec {}
#[doc = "`write(|w| ..)` method takes [`chmap3::W`](W) writer structure"]
impl crate::Writable for Chmap3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHMAP3 to value 0"]
impl crate::Resettable for Chmap3Spec {
    const RESET_VALUE: u32 = 0;
}
