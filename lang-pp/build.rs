use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Generate interned strings
    string_cache_codegen::AtomType::new("exts::names::ExtNameAtom", "ext_name!")
        .atoms(&[
            "GL_ARB_shading_language_include",
            "GL_GOOGLE_cpp_style_line_directive",
            "GL_GOOGLE_include_directive",
        ])
        .write_to_file(&out_dir.join("ext_names.rs"))
        .expect("failed to generate atoms");

    string_cache_codegen::AtomType::new("last::type_names::TypeNameAtom", "type_name!")
        .atoms(&[
            "void",
            "int",
            "bool",
            "float",
            "double",
            "vec2",
            "vec3",
            "vec4",
            "ivec2",
            "ivec3",
            "ivec4",
            "bvec2",
            "bvec3",
            "bvec4",
            "uint",
            "atomic_uint",
            "uvec2",
            "uvec3",
            "uvec4",
            "dvec2",
            "dvec3",
            "dvec4",
            "mat2",
            "mat3",
            "mat4",
            "mat2x2",
            "mat2x3",
            "mat2x4",
            "mat3x2",
            "mat3x3",
            "mat3x4",
            "mat4x2",
            "mat4x3",
            "mat4x4",
            "dmat2",
            "dmat3",
            "dmat4",
            "dmat2x2",
            "dmat2x3",
            "dmat2x4",
            "dmat3x2",
            "dmat3x3",
            "dmat3x4",
            "dmat4x2",
            "dmat4x3",
            "dmat4x4",
            "sampler1D",
            "sampler1DShadow",
            "sampler1DArray",
            "sampler1DArrayShadow",
            "isampler1D",
            "isampler1DArray",
            "usampler1D",
            "usampler1DArray",
            "sampler2D",
            "sampler2DShadow",
            "sampler2DArray",
            "sampler2DArrayShadow",
            "isampler2D",
            "isampler2DArray",
            "usampler2D",
            "usampler2DArray",
            "sampler2DRect",
            "sampler2DRectShadow",
            "isampler2DRect",
            "usampler2DRect",
            "sampler2DMS",
            "isampler2DMS",
            "usampler2DMS",
            "sampler2DMSArray",
            "isampler2DMSArray",
            "usampler2DMSArray",
            "sampler3D",
            "isampler3D",
            "usampler3D",
            "samplerCube",
            "samplerCubeShadow",
            "isamplerCube",
            "usamplerCube",
            "samplerCubeArray",
            "samplerCubeArrayShadow",
            "isamplerCubeArray",
            "usamplerCubeArray",
            "samplerBuffer",
            "isamplerBuffer",
            "usamplerBuffer",
            "image1D",
            "iimage1D",
            "uimage1D",
            "image1DArray",
            "iimage1DArray",
            "uimage1DArray",
            "image2D",
            "iimage2D",
            "uimage2D",
            "image2DArray",
            "iimage2DArray",
            "uimage2DArray",
            "image2DRect",
            "iimage2DRect",
            "uimage2DRect",
            "image2DMS",
            "iimage2DMS",
            "uimage2DMS",
            "image2DMSArray",
            "iimage2DMSArray",
            "uimage2DMSArray",
            "image3D",
            "iimage3D",
            "uimage3D",
            "imageCube",
            "iimageCube",
            "uimageCube",
            "imageCubeArray",
            "iimageCubeArray",
            "uimageCubeArray",
            "imageBuffer",
            "iimageBuffer",
            "uimageBuffer",
            // Vulkan type names
            "texture1D",
            "texture1DArray",
            "itexture1D",
            "itexture1DArray",
            "utexture1D",
            "utexture1DArray",
            "texture2D",
            "texture2DArray",
            "itexture2D",
            "itexture2DArray",
            "utexture2D",
            "utexture2DArray",
            "texture2DRect",
            "itexture2DRect",
            "utexture2DRect",
            "texture2DMS",
            "itexture2DMS",
            "utexture2DMS",
            "texture2DMSArray",
            "itexture2DMSArray",
            "utexture2DMSArray",
            "texture3D",
            "itexture3D",
            "utexture3D",
            "textureCube",
            "itextureCube",
            "utextureCube",
            "textureCubeArray",
            "itextureCubeArray",
            "utextureCubeArray",
            "textureBuffer",
            "itextureBuffer",
            "utextureBuffer",
            "sampler",
            "samplerShadow",
            "subpassInput",
            "isubpassInput",
            "usubpassInput",
            "subpassInputMS",
            "isubpassInputMS",
            "usubpassInputMS",
            // Reserved for future use
            "hvec2",
            "hvec3",
            "hvec4",
            "fvec2",
            "fvec3",
            "fvec4",
            "sampler3DRect",
        ])
        .write_to_file(&out_dir.join("type_names.rs"))
        .expect("failed to generate atoms");

    string_cache_codegen::AtomType::new("last::keywords::KeywordAtom", "keyword!")
        .atoms(&[
            "const",
            "uniform",
            "buffer",
            "shared",
            "attribute",
            "varying",
            "coherent",
            "volatile",
            "restrict",
            "readonly",
            "writeonly",
            "layout",
            "centroid",
            "flat",
            "smooth",
            "noperspective",
            "patch",
            "sample",
            "invariant",
            "precise",
            "break",
            "continue",
            "do",
            "for",
            "while",
            "switch",
            "case",
            "default",
            "if",
            "else",
            "subroutine",
            "in",
            "out",
            "inout",
            "true",
            "false",
            "discard",
            "return",
            "lowp",
            "mediump",
            "highp",
            "precision",
            "struct",
            // Reserved for future use
            "common",
            "partition",
            "active",
            "asm",
            "class",
            "union",
            "enum",
            "typedef",
            "template",
            "this",
            "resource",
            "goto",
            "inline",
            "noinline",
            "public",
            "static",
            "extern",
            "external",
            "interface",
            "long",
            "short",
            "half",
            "fixed",
            "unsigned",
            "superp",
            "input",
            "output",
            "filter",
            "sizeof",
            "cast",
            "namespace",
            "using",
        ])
        .write_to_file(&out_dir.join("keywords.rs"))
        .expect("failed to generate atoms");
}
