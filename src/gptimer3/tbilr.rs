#[doc = "Register `TBILR` reader"]
pub type R = crate::R<TbilrSpec>;
#[doc = "Register `TBILR` writer"]
pub type W = crate::W<TbilrSpec>;
#[doc = "Field `TBILR` reader - GPTM B interval load register"]
pub type TbilrR = crate::FieldReader<u16>;
#[doc = "Field `TBILR` writer - GPTM B interval load register"]
pub type TbilrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM B interval load register"]
    #[inline(always)]
    pub fn tbilr(&self) -> TbilrR {
        TbilrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM B interval load register"]
    #[inline(always)]
    pub fn tbilr(&mut self) -> TbilrW<TbilrSpec> {
        TbilrW::new(self, 0)
    }
}
#[doc = "GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the load value. Bits \\[31:16\\]
are reserved in both cases.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbilr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbilr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbilrSpec;
impl crate::RegisterSpec for TbilrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbilr::R`](R) reader structure"]
impl crate::Readable for TbilrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbilr::W`](W) writer structure"]
impl crate::Writable for TbilrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBILR to value 0"]
impl crate::Resettable for TbilrSpec {
    const RESET_VALUE: u32 = 0;
}
