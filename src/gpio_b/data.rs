#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `DATA` reader - Input and output data"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Input and output data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input and output data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input and output data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
