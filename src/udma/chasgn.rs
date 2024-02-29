#[doc = "Register `CHASGN` reader"]
pub type R = crate::R<ChasgnSpec>;
#[doc = "Register `CHASGN` writer"]
pub type W = crate::W<ChasgnSpec>;
#[doc = "Field `CHASGN` reader - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
pub type ChasgnR = crate::FieldReader<u32>;
#[doc = "Field `CHASGN` writer - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
pub type ChasgnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
    #[inline(always)]
    pub fn chasgn(&self) -> ChasgnR {
        ChasgnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
    #[inline(always)]
    #[must_use]
    pub fn chasgn(&mut self) -> ChasgnW<ChasgnSpec> {
        ChasgnW::new(self, 0)
    }
}
#[doc = "DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chasgn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chasgn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChasgnSpec;
impl crate::RegisterSpec for ChasgnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chasgn::R`](R) reader structure"]
impl crate::Readable for ChasgnSpec {}
#[doc = "`write(|w| ..)` method takes [`chasgn::W`](W) writer structure"]
impl crate::Writable for ChasgnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHASGN to value 0"]
impl crate::Resettable for ChasgnSpec {
    const RESET_VALUE: u32 = 0;
}
