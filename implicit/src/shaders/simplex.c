//
// Description : Array and textureless GLSL 2D simplex noise function.
//      Author : Ian McEwan, Ashima Arts.
//  Maintainer : stegu
//     Lastmod : 20110822 (ijm)
//     License : Copyright (C) 2011 Ashima Arts. All rights reserved.
//               Distributed under the MIT License. See LICENSE file.
//               https://github.com/ashima/webgl-noise
//               https://github.com/stegu/webgl-noise
//

#define vec3 float3
#define vec2 float2
#define vec4 float4

float3 comp_mul_3(vec3 v, float f) {
  return (vec3)(v.x * f, v.y * f, v.z * f);
}

float3 comp_add_3(vec3 v, float f) {
  return (vec3)(v.x + f, v.y + f, v.z + f);
}

float2 comp_mul_2(vec2 v, float f) {
  return (vec2)(v.x * f, v.y * f);
}

float3 mod289_3(vec3 x) {
  return x - comp_mul_3(floor(comp_mul_3(x, (1.0 / 289.0))), 289.0);
}

float2 mod289_2(vec2 x) {
  return x - comp_mul_2(floor(comp_mul_2(x, (1.0 / 289.0))), 289.0);
}

float3 permute(vec3 x) {
  return mod289_3(
        comp_add_3(
          comp_mul_3(x,34.0),
          1.0
        ) * x);
}

float snoise(vec2 v)
{
  const vec4 C = (vec4)(0.211324865405187,  // (3.0-sqrt(3.0))/6.0
                      0.366025403784439,  // 0.5*(sqrt(3.0)-1.0)
                      -0.577350269189626,  // -1.0 + 2.0 * C.x
                      0.024390243902439); // 1.0 / 41.0
  // First corner
  vec2 i  = floor(v + dot(v, C.yy) );
  vec2 x0 = v -   i + dot(i, C.xx);

  // Other corners
  vec2 i1;
  //i1.x = step( x0.y, x0.x ); // x0.x > x0.y ? 1.0 : 0.0
  //i1.y = 1.0 - i1.x;
  i1 = (x0.x > x0.y) ? (vec2)(1.0, 0.0) : (vec2)(0.0, 1.0);
  // x0 = x0 - 0.0 + 0.0 * C.xx ;
  // x1 = x0 - i1 + 1.0 * C.xx ;
  // x2 = x0 - 1.0 + 2.0 * C.xx ;
  vec4 x12 = x0.xyxy + C.xxzz;
  x12.xy -= i1;

  // Permutations
  i = mod289_2(i); // Avoid truncation effects in permutation
  vec3 p = permute( permute( i.y + (vec3)(0.0, i1.y, 1.0 ))
                    + i.x + (vec3)(0.0, i1.x, 1.0 ));

  vec3 m = max((vec3)(0.5, 0.5, 0.5) - (vec3)(dot(x0,x0), dot(x12.xy,x12.xy), dot(x12.zw,x12.zw)), (vec3)(0.0, 0.0, 0.0));
  m = m*m ;
  m = m*m ;

  // Gradients: 41 points uniformly over a line, mapped onto a diamond.
  // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)

  // TODO: REINSTATE vec3 x = 2.0 * fract(p * C.www) - 1.0;
  //vec3 x = (vec3)(2.0, 2.0, 2.0) * p * C.www - (vec3)(1.0, 1.0, 1.0);
  vec3 garbage;
  vec3 x = (vec3)(2.0, 2.0, 2.0) * fract(p * C.www, &garbage) - (vec3)(1.0, 1.0, 1.0);
  vec3 h = fabs(x) - (vec3)(0.5, 0.5, 0.5);
  vec3 ox = floor(x + (vec3)(0.5, 0.5, 0.5));
  vec3 a0 = x - ox;

  // Normalise gradients implicitly by scaling m
  // Approximation of: m *= inversesqrt( a0*a0 + h*h );
  float magic = 1.79284291400159 - 0.85373472095314;
  m *= (vec3)(magic, magic, magic) * ( a0*a0 + h*h );

  // Compute final noise value at P
  vec3 g;
  g.x  = a0.x  * x0.x  + h.x  * x0.y;
  g.yz = a0.yz * x12.xz + h.yz * x12.yw;
  return 130.0 * dot(m, g);
}

__kernel void apply(
    __global float *buffer,
    ulong width
//    float m11, float m12,
//    float m21, float m22,
//    float m31, float m32
)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    float scalar = width / 3.0;

    buffer[pos] = snoise((float2)((float)x / scalar, (float)y / scalar));
}
