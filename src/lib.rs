pub mod x {
    use std::any::TypeId;

    use windows::Win32::Graphics::{Direct3D12::*, Dxgi::Common::*};

    macro_rules! add_pair {
        ($o:expr,$val:expr,$typ:ty) => {
            $o.push(($val, TypeId::of::<$typ>()));
        };
    }

    pub fn check_pair(ty: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE, val: TypeId) {
        let mut vec: Vec<(D3D12_PIPELINE_STATE_SUBOBJECT_TYPE, TypeId)> = Vec::new();

        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE, ID3D12RootSignature);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS, D3D12_SHADER_BYTECODE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS, D3D12_SHADER_BYTECODE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS, D3D12_SHADER_BYTECODE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS, D3D12_SHADER_BYTECODE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS, D3D12_SHADER_BYTECODE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CS, D3D12_SHADER_BYTECODE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT, D3D12_STREAM_OUTPUT_DESC);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND, D3D12_BLEND_DESC);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK, u32);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER, D3D12_RASTERIZER_DESC);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL, D3D12_DEPTH_STENCIL_DESC);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT, D3D12_INPUT_LAYOUT_DESC);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE, D3D12_INDEX_BUFFER_STRIP_CUT_VALUE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY, D3D12_PRIMITIVE_TOPOLOGY_TYPE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS, D3D12_RT_FORMAT_ARRAY);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT, DXGI_FORMAT);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC, DXGI_SAMPLE_DESC);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK, D3D12_NODE_MASK);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK, u32);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO, D3D12_CACHED_PIPELINE_STATE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS, D3D12_PIPELINE_STATE_FLAGS);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1, D3D12_DEPTH_STENCIL_DESC1);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VIEW_INSTANCING, D3D12_VIEW_INSTANCING_DESC);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_AS, D3D12_SHADER_BYTECODE);
        add_pair!(vec, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MS, D3D12_SHADER_BYTECODE);

        if let None = vec.into_iter().find(|(a, b)| *a == ty && *b == val) {
            panic!("The pair is not valid.")
        }
    }
}

pub mod y {
    pub use std::any::TypeId;
    pub use std::assert_eq;
    pub use std::mem::align_of;
    pub use windows::Win32::Graphics::Direct3D12::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE;
}

#[macro_export]
macro_rules! def {
    ($name:ident,$val:expr,$typ:ty,$test:ident) => {
        #[repr(C, align(8))]
        pub struct $name {
            ty: $crate::y::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE,
            v: $typ,
        }
        impl $name {
            pub fn new(v: $typ) -> Self {
                Self { ty: $val, v }
            }
        }
        #[test]
        fn $test() {
            $crate::x::check_pair($val, $crate::y::TypeId::of::<$typ>());
            $crate::y::assert_eq!($crate::y::align_of::<$name>(), 8);
        }
    };
}
