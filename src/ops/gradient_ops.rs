use ndarray_ext::NdArray;
use ops;
use tensor::Tensor;


pub struct StopGradient;

impl ops::Op for StopGradient {
    fn name(&self) -> &str
    {
        "StopGradient"
    }

    fn compute(&self, _: ::runtime::OpComputeContext) -> Result<NdArray, ::OpComputeErrorStatus>
    {
        Err(::OpComputeErrorStatus::Delegate { to: 0 })
    }

    fn grad(&self, _: &Tensor, _: &[&Tensor], _: &Tensor) -> Vec<Option<Tensor>>
    {
        vec![None]
    }
}
