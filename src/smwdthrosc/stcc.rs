#[doc = "Register `STCC` reader"]
pub type R = crate::R<StccSpec>;
#[doc = "Register `STCC` writer"]
pub type W = crate::W<StccSpec>;
#[doc = "Field `PIN` reader - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PORT` reader - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<StccSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<StccSpec> {
        PortW::new(self, 3)
    }
}
#[doc = "Sleep Timer Capture control\n\nYou can [`read`](crate::Reg::read) this register and get [`stcc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StccSpec;
impl crate::RegisterSpec for StccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcc::R`](R) reader structure"]
impl crate::Readable for StccSpec {}
#[doc = "`write(|w| ..)` method takes [`stcc::W`](W) writer structure"]
impl crate::Writable for StccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCC to value 0"]
impl crate::Resettable for StccSpec {
    const RESET_VALUE: u32 = 0;
}
