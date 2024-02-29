#[doc = "Register `IO` reader"]
pub type R = crate::R<IoSpec>;
#[doc = "Register `IO` writer"]
pub type W = crate::W<IoSpec>;
#[doc = "Field `SC` reader - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
pub type ScR = crate::BitReader;
#[doc = "Field `SC` writer - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
pub type ScW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> ScW<IoSpec> {
        ScW::new(self, 0)
    }
}
#[doc = "Output strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoSpec;
impl crate::RegisterSpec for IoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io::R`](R) reader structure"]
impl crate::Readable for IoSpec {}
#[doc = "`write(|w| ..)` method takes [`io::W`](W) writer structure"]
impl crate::Writable for IoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IO to value 0"]
impl crate::Resettable for IoSpec {
    const RESET_VALUE: u32 = 0;
}
