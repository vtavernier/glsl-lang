#version 310 es

layout(local_size_x = 2) in;
layout(local_size_x = 16) in;     // ERROR, changing
layout(local_size_z = 4096) in;   // ERROR, too large
layout(local_size_x = 2) in;
layout(local_size_y = 0) in;      // ERROR, 0 not allowed
const int total = gl_MaxComputeWorkGroupCount.y 
                + gl_MaxComputeUniformComponents
                + gl_MaxComputeTextureImageUnits
                + gl_MaxComputeImageUniforms
                + gl_MaxComputeAtomicCounters
                + gl_MaxComputeAtomicCounterBuffers;

buffer ShaderStorageBlock
{
    int value;
    float values[];
};

buffer InvalidShaderStorageBlock
{
    float values[];  // ERROR
    int value;
} invalid;

void main()
{
    barrier();
    memoryBarrier();
    memoryBarrierAtomicCounter();
    memoryBarrierBuffer();
    memoryBarrierShared();
    memoryBarrierImage();
    groupMemoryBarrier();
    value = int(values[gl_LocalInvocationIndex]);
}

layout(location = 2) in vec3 v3;      // ERROR
in float f;                           // ERROR
out float fo;                         // ERROR

shared vec4 s;
layout(location = 2) shared vec4 sl;  // ERROR
shared float fs = 4.2;                // ERROR

layout(local_size_x = 2, local_size_y = 3, local_size_z = 4) out;  // ERROR

int arrX[gl_WorkGroupSize.x];
int arrY[gl_WorkGroupSize.y];
int arrZ[gl_WorkGroupSize.z];

readonly buffer roblock
{
    int value;
    float values[];
} ro;

void foo()
{
    ro.values[2] = 4.7;        // ERROR, readonly
    ro.values.length();
    ++s;
}

buffer vec4 v;  // ERROR

uniform usampler2D us2dbad;  // ERROR, default precision

precision highp usampler2D;
precision highp iimage2DArray;
precision highp iimage2D;

uniform usampler2D us2d;

uniform iimage2DArray ii2dabad;  // ERROR, not writeonly
uniform writeonly iimage2DArray ii2da;

layout(r32i) uniform iimage2D iimg2D;
layout(rgba32i) uniform readonly iimage2D iimg2Drgba;
layout(rgba32f) uniform readonly image2D img2Drgba;   // ERROR, no default
layout(r32ui) uniform uimage2D uimg2D;                // ERROR, no default

void qux()
{
    int i = 4;
    imageAtomicCompSwap(iimg2D, ivec2(i,i), i, i);// ERROR no longer in 310
    imageAtomicAdd(uimg2D, ivec2(i,i), uint(i));  // ERROR no longer in 310
    imageAtomicMin(iimg2Drgba, ivec2(i,i), i);    // ERROR no longer in 310  // ERROR iimg2Drgba does not have r32i layout
    imageAtomicMax(img2Drgba, ivec2(i,i), i);     // ERROR no longer in 310  // ERROR img2Drgba is not integer image
    ivec4 pos = imageLoad(iimg2D, ivec2(i,i));
    imageStore(ii2da, ivec3(i,i,i), ivec4(0));
    imageLoad(img2Drgba, ivec2(i,i));
    imageLoad(ii2da, ivec3(i,i,i));       // ERROR, drops writeonly
}

volatile float vol; // ERROR, not an image
readonly int vol2;  // ERROR, not an image

void passr(coherent readonly iimage2D image)
{
}

layout(r32i) coherent readonly uniform iimage2D qualim1;
layout(r32i) coherent restrict readonly uniform iimage2D qualim2;

void passrc()
{
    passr(qualim1);   // ERROR, changing formats
    passr(qualim2);   // ERROR, drops restrict, ERROR, changing formats
    passr(iimg2D);    // ERROR, changing formats
}

highp layout(rg8i)     uniform readonly uimage2D i1bad; // ERROR, type mismatch
highp layout(rgba32i)  uniform readonly image2D i2bad;  // ERROR, type mismatch
highp layout(rgba32f)  uniform readonly uimage2D i3bad; // ERROR, type mismatch
layout(r8_snorm) uniform readonly iimage2D i4bad; // ERROR, type mismatch
layout(rgba32ui) uniform readonly iimage2D i5bad; // ERROR, type mismatch
layout(r8ui)     uniform readonly iimage2D i6bad; // ERROR, type mismatch

layout(binding = 0) uniform atomic_uint counter;

uint func(atomic_uint c)
{
    return atomicCounterIncrement(c);
}

uint func2(out atomic_uint c) // ERROR, output
{
    return counter;           // ERROR, type mismatch
    return atomicCounter(counter);
}

void mainAC()
{
     atomic_uint non_uniform_counter; // ERROR
     uint val = atomicCounter(counter);
     atomicCounterDecrement(counter);
}

layout(binding = 1) uniform mediump atomic_uint counterBad;  // ERROR, not highp

layout(binding = 2, offset = 4) uniform atomic_uint countArr[4];
uniform int i;

void opac()
{
    int a[3];
    a[counter];         // ERROR, non-integer
    countArr[2];
    countArr[i];
}

shared int atomi;
shared uint atomu;

void atoms()
{
    int origi = atomicAdd(atomi, 3);
    uint origu = atomicAnd(atomu, 7u);
    origi = atomicExchange(atomi, 4);
    origu = atomicCompSwap(atomu, 10u, 8u);
}

precision highp atomic_uint;
precision lowp atomic_uint;   // ERROR

precise int pfoo;  // ERROR, reserved

dmat2x4 dm;                     // ERROR
uniform samplerCubeArray sca;   // ERROR
uniform iimage2DRect i2dr;      // ERROR
highp uniform image2DMS i2dms;  // ERROR
uniform uimage2DMSArray u2dmsa; // ERROR

highp layout(r32f)  coherent volatile restrict readonly writeonly uniform  image2D okay1;
      layout(r32i)  coherent volatile restrict readonly           uniform iimage2D okay2;
highp layout(r32ui) coherent volatile restrict          writeonly uniform uimage2D okay3;
highp layout(r32f)  coherent volatile restrict                    uniform  image2D okay4;
 
highp layout(rgba32f)  coherent volatile restrict                 uniform  image2D badQ1;  // ERROR, bad qualifiers
      layout(rgba8i)   coherent volatile restrict                 uniform iimage2D badQ2;  // ERROR, bad qualifiers
highp layout(rgba16ui) coherent volatile restrict                 uniform uimage2D badQ3;  // ERROR, bad qualifiers

writeonly buffer woblock
{
    int value;
    float values[];
} wo;

void foowo()
{
    float g;
    g = wo.values[2];            // ERROR, writeonly
    float f = wo.values[2];      // ERROR, writeonly
    ++wo.values[2];              // ERROR, writeonly
    wo.values[2]--;              // ERROR, writeonly
    f + wo.values[2];            // ERROR, writeonly
    wo.values[2] - f;            // ERROR, writeonly
    bool b;
    b ? f : wo.values[2];        // ERROR, writeonly
    b ? wo.values[2] : f;        // ERROR, writeonly
    if (f == wo.values[2])       // ERROR, writeonly
        ++f;
    if (f >= wo.values[2])       // ERROR, writeonly
        ++f;
    f = vec3(wo.values[2]).x;    // ERROR, writeonly
    ~wo.value;                   // ERROR, writeonly
    wo.values[2] = 3.4;
}

buffer multioblock
{
    readonly int value;
    writeonly float values[];
} multio;

void foomultio()
{
    float g;
    g = wo.values[2];            // ERROR, writeonly
    ~wo.value;
    wo.values[2] = 3.4;
    wo.value = 2;                // ERROR, readonly
}

in inb {     // ERROR
    int a;
} inbi;

out outb {     // ERROR
    int a;
} outbi;

float t__;  // ERROR, no __ until revision 310

    // ERROR, no __ until revision 310

shared vec4 arr[2][3][4];

void devi()
{
    gl_DeviceIndex; // ERROR, no extension
    gl_ViewIndex;   // ERROR, never this stage
}

#extension GL_EXT_device_group : enable

void devie()
{
    gl_DeviceIndex;
    gl_ViewIndex;   // ERROR, never this stage
}
