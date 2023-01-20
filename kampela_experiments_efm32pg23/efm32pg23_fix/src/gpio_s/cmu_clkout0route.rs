#[doc = "Register `CMU_CLKOUT0ROUTE` reader"]
pub struct R(crate::R<CMU_CLKOUT0ROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMU_CLKOUT0ROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMU_CLKOUT0ROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMU_CLKOUT0ROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMU_CLKOUT0ROUTE` writer"]
pub struct W(crate::W<CMU_CLKOUT0ROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMU_CLKOUT0ROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CMU_CLKOUT0ROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMU_CLKOUT0ROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT` reader - CLKOUT0 port select register"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - CLKOUT0 port select register"]
pub type PORT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMU_CLKOUT0ROUTE_SPEC, u8, u8, 2, O>;
#[doc = "Field `PIN` reader - CLKOUT0 pin select register"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - CLKOUT0 pin select register"]
pub type PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMU_CLKOUT0ROUTE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - CLKOUT0 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CLKOUT0 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKOUT0 port select register"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PORT_W<0> {
        PORT_W::new(self)
    }
    #[doc = "Bits 16:19 - CLKOUT0 pin select register"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PIN_W<16> {
        PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKOUT0 port/pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmu_clkout0route](index.html) module"]
pub struct CMU_CLKOUT0ROUTE_SPEC;
impl crate::RegisterSpec for CMU_CLKOUT0ROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmu_clkout0route::R](R) reader structure"]
impl crate::Readable for CMU_CLKOUT0ROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmu_clkout0route::W](W) writer structure"]
impl crate::Writable for CMU_CLKOUT0ROUTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMU_CLKOUT0ROUTE to value 0"]
impl crate::Resettable for CMU_CLKOUT0ROUTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
