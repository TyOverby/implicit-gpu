float dist_to_line(float x, float y, float x1, float y1, float x2, float y2)
{
    float A = x - x1;
    float B = y - y1;
    float C = x2 - x1;
    float D = y2 - y1;

    float dot = A * C + B * D;
    float len_sq = C * C + D * D;
    float param = -1;

    if (len_sq != 0)
    {
        param = dot / len_sq;
    }

    float xx;
    float yy;

    if (param < 0)
    {
        xx = x1;
        yy = y1;
    }
    else if (param > 1)
    {
        xx = x2;
        yy = y2;
    }
    else
    {
        xx = x1 + param * C;
        yy = y1 + param * D;
    }

    float dx = x - xx;
    float dy = y - yy;

    return sqrt(dx * dx + dy * dy);
}

float position(float x, float y, float x1, float y1, float x2, float y2)
{
    return (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1);
}

__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float x_0 = x_s - 7;
  float y_1 = y_s - 7;

  float circle_2;
  {
    float dx_3 = x_0 - 0;
    float dy_4 = y_1 - 0;
    circle_2 = sqrt(dx_3 * dx_3 + dy_4 * dy_4) - 5;
  }

  float circle_5;
  {
    float dx_6 = x_0 - 0;
    float dy_7 = y_1 - 10;
    circle_5 = sqrt(dx_6 * dx_6 + dy_7 * dy_7) - 5;
  }

  float circle_8;
  {
    float dx_9 = x_0 - 0;
    float dy_10 = y_1 - 20;
    circle_8 = sqrt(dx_9 * dx_9 + dy_10 * dy_10) - 5;
  }

  float or_11 = min(circle_5, circle_8);

  float or_12 = min(circle_2, or_11);

  float circle_13;
  {
    float dx_14 = x_0 - 0;
    float dy_15 = y_1 - 30;
    circle_13 = sqrt(dx_14 * dx_14 + dy_15 * dy_15) - 5;
  }

  float circle_16;
  {
    float dx_17 = x_0 - 0;
    float dy_18 = y_1 - 40;
    circle_16 = sqrt(dx_17 * dx_17 + dy_18 * dy_18) - 5;
  }

  float circle_19;
  {
    float dx_20 = x_0 - 0;
    float dy_21 = y_1 - 50;
    circle_19 = sqrt(dx_20 * dx_20 + dy_21 * dy_21) - 5;
  }

  float or_22 = min(circle_16, circle_19);

  float or_23 = min(circle_13, or_22);

  float or_24 = min(or_12, or_23);

  float circle_25;
  {
    float dx_26 = x_0 - 0;
    float dy_27 = y_1 - 60;
    circle_25 = sqrt(dx_26 * dx_26 + dy_27 * dy_27) - 5;
  }

  float circle_28;
  {
    float dx_29 = x_0 - 0;
    float dy_30 = y_1 - 70;
    circle_28 = sqrt(dx_29 * dx_29 + dy_30 * dy_30) - 5;
  }

  float circle_31;
  {
    float dx_32 = x_0 - 0;
    float dy_33 = y_1 - 80;
    circle_31 = sqrt(dx_32 * dx_32 + dy_33 * dy_33) - 5;
  }

  float or_34 = min(circle_28, circle_31);

  float or_35 = min(circle_25, or_34);

  float circle_36;
  {
    float dx_37 = x_0 - 0;
    float dy_38 = y_1 - 90;
    circle_36 = sqrt(dx_37 * dx_37 + dy_38 * dy_38) - 5;
  }

  float circle_39;
  {
    float dx_40 = x_0 - 10;
    float dy_41 = y_1 - 0;
    circle_39 = sqrt(dx_40 * dx_40 + dy_41 * dy_41) - 5;
  }

  float circle_42;
  {
    float dx_43 = x_0 - 10;
    float dy_44 = y_1 - 10;
    circle_42 = sqrt(dx_43 * dx_43 + dy_44 * dy_44) - 5;
  }

  float or_45 = min(circle_39, circle_42);

  float or_46 = min(circle_36, or_45);

  float or_47 = min(or_35, or_46);

  float or_48 = min(or_24, or_47);

  float circle_49;
  {
    float dx_50 = x_0 - 10;
    float dy_51 = y_1 - 20;
    circle_49 = sqrt(dx_50 * dx_50 + dy_51 * dy_51) - 5;
  }

  float circle_52;
  {
    float dx_53 = x_0 - 10;
    float dy_54 = y_1 - 30;
    circle_52 = sqrt(dx_53 * dx_53 + dy_54 * dy_54) - 5;
  }

  float circle_55;
  {
    float dx_56 = x_0 - 10;
    float dy_57 = y_1 - 40;
    circle_55 = sqrt(dx_56 * dx_56 + dy_57 * dy_57) - 5;
  }

  float or_58 = min(circle_52, circle_55);

  float or_59 = min(circle_49, or_58);

  float circle_60;
  {
    float dx_61 = x_0 - 10;
    float dy_62 = y_1 - 50;
    circle_60 = sqrt(dx_61 * dx_61 + dy_62 * dy_62) - 5;
  }

  float circle_63;
  {
    float dx_64 = x_0 - 10;
    float dy_65 = y_1 - 60;
    circle_63 = sqrt(dx_64 * dx_64 + dy_65 * dy_65) - 5;
  }

  float circle_66;
  {
    float dx_67 = x_0 - 10;
    float dy_68 = y_1 - 70;
    circle_66 = sqrt(dx_67 * dx_67 + dy_68 * dy_68) - 5;
  }

  float or_69 = min(circle_63, circle_66);

  float or_70 = min(circle_60, or_69);

  float or_71 = min(or_59, or_70);

  float circle_72;
  {
    float dx_73 = x_0 - 10;
    float dy_74 = y_1 - 80;
    circle_72 = sqrt(dx_73 * dx_73 + dy_74 * dy_74) - 5;
  }

  float circle_75;
  {
    float dx_76 = x_0 - 10;
    float dy_77 = y_1 - 90;
    circle_75 = sqrt(dx_76 * dx_76 + dy_77 * dy_77) - 5;
  }

  float circle_78;
  {
    float dx_79 = x_0 - 20;
    float dy_80 = y_1 - 0;
    circle_78 = sqrt(dx_79 * dx_79 + dy_80 * dy_80) - 5;
  }

  float or_81 = min(circle_75, circle_78);

  float or_82 = min(circle_72, or_81);

  float circle_83;
  {
    float dx_84 = x_0 - 20;
    float dy_85 = y_1 - 10;
    circle_83 = sqrt(dx_84 * dx_84 + dy_85 * dy_85) - 5;
  }

  float circle_86;
  {
    float dx_87 = x_0 - 20;
    float dy_88 = y_1 - 20;
    circle_86 = sqrt(dx_87 * dx_87 + dy_88 * dy_88) - 5;
  }

  float or_89 = min(circle_83, circle_86);

  float circle_90;
  {
    float dx_91 = x_0 - 20;
    float dy_92 = y_1 - 30;
    circle_90 = sqrt(dx_91 * dx_91 + dy_92 * dy_92) - 5;
  }

  float circle_93;
  {
    float dx_94 = x_0 - 20;
    float dy_95 = y_1 - 40;
    circle_93 = sqrt(dx_94 * dx_94 + dy_95 * dy_95) - 5;
  }

  float or_96 = min(circle_90, circle_93);

  float or_97 = min(or_89, or_96);

  float or_98 = min(or_82, or_97);

  float or_99 = min(or_71, or_98);

  float or_100 = min(or_48, or_99);

  float circle_101;
  {
    float dx_102 = x_0 - 20;
    float dy_103 = y_1 - 50;
    circle_101 = sqrt(dx_102 * dx_102 + dy_103 * dy_103) - 5;
  }

  float circle_104;
  {
    float dx_105 = x_0 - 20;
    float dy_106 = y_1 - 60;
    circle_104 = sqrt(dx_105 * dx_105 + dy_106 * dy_106) - 5;
  }

  float circle_107;
  {
    float dx_108 = x_0 - 20;
    float dy_109 = y_1 - 70;
    circle_107 = sqrt(dx_108 * dx_108 + dy_109 * dy_109) - 5;
  }

  float or_110 = min(circle_104, circle_107);

  float or_111 = min(circle_101, or_110);

  float circle_112;
  {
    float dx_113 = x_0 - 20;
    float dy_114 = y_1 - 80;
    circle_112 = sqrt(dx_113 * dx_113 + dy_114 * dy_114) - 5;
  }

  float circle_115;
  {
    float dx_116 = x_0 - 20;
    float dy_117 = y_1 - 90;
    circle_115 = sqrt(dx_116 * dx_116 + dy_117 * dy_117) - 5;
  }

  float circle_118;
  {
    float dx_119 = x_0 - 30;
    float dy_120 = y_1 - 0;
    circle_118 = sqrt(dx_119 * dx_119 + dy_120 * dy_120) - 5;
  }

  float or_121 = min(circle_115, circle_118);

  float or_122 = min(circle_112, or_121);

  float or_123 = min(or_111, or_122);

  float circle_124;
  {
    float dx_125 = x_0 - 30;
    float dy_126 = y_1 - 10;
    circle_124 = sqrt(dx_125 * dx_125 + dy_126 * dy_126) - 5;
  }

  float circle_127;
  {
    float dx_128 = x_0 - 30;
    float dy_129 = y_1 - 20;
    circle_127 = sqrt(dx_128 * dx_128 + dy_129 * dy_129) - 5;
  }

  float circle_130;
  {
    float dx_131 = x_0 - 30;
    float dy_132 = y_1 - 30;
    circle_130 = sqrt(dx_131 * dx_131 + dy_132 * dy_132) - 5;
  }

  float or_133 = min(circle_127, circle_130);

  float or_134 = min(circle_124, or_133);

  float circle_135;
  {
    float dx_136 = x_0 - 30;
    float dy_137 = y_1 - 40;
    circle_135 = sqrt(dx_136 * dx_136 + dy_137 * dy_137) - 5;
  }

  float circle_138;
  {
    float dx_139 = x_0 - 30;
    float dy_140 = y_1 - 50;
    circle_138 = sqrt(dx_139 * dx_139 + dy_140 * dy_140) - 5;
  }

  float circle_141;
  {
    float dx_142 = x_0 - 30;
    float dy_143 = y_1 - 60;
    circle_141 = sqrt(dx_142 * dx_142 + dy_143 * dy_143) - 5;
  }

  float or_144 = min(circle_138, circle_141);

  float or_145 = min(circle_135, or_144);

  float or_146 = min(or_134, or_145);

  float or_147 = min(or_123, or_146);

  float circle_148;
  {
    float dx_149 = x_0 - 30;
    float dy_150 = y_1 - 70;
    circle_148 = sqrt(dx_149 * dx_149 + dy_150 * dy_150) - 5;
  }

  float circle_151;
  {
    float dx_152 = x_0 - 30;
    float dy_153 = y_1 - 80;
    circle_151 = sqrt(dx_152 * dx_152 + dy_153 * dy_153) - 5;
  }

  float circle_154;
  {
    float dx_155 = x_0 - 30;
    float dy_156 = y_1 - 90;
    circle_154 = sqrt(dx_155 * dx_155 + dy_156 * dy_156) - 5;
  }

  float or_157 = min(circle_151, circle_154);

  float or_158 = min(circle_148, or_157);

  float circle_159;
  {
    float dx_160 = x_0 - 40;
    float dy_161 = y_1 - 0;
    circle_159 = sqrt(dx_160 * dx_160 + dy_161 * dy_161) - 5;
  }

  float circle_162;
  {
    float dx_163 = x_0 - 40;
    float dy_164 = y_1 - 10;
    circle_162 = sqrt(dx_163 * dx_163 + dy_164 * dy_164) - 5;
  }

  float circle_165;
  {
    float dx_166 = x_0 - 40;
    float dy_167 = y_1 - 20;
    circle_165 = sqrt(dx_166 * dx_166 + dy_167 * dy_167) - 5;
  }

  float or_168 = min(circle_162, circle_165);

  float or_169 = min(circle_159, or_168);

  float or_170 = min(or_158, or_169);

  float circle_171;
  {
    float dx_172 = x_0 - 40;
    float dy_173 = y_1 - 30;
    circle_171 = sqrt(dx_172 * dx_172 + dy_173 * dy_173) - 5;
  }

  float circle_174;
  {
    float dx_175 = x_0 - 40;
    float dy_176 = y_1 - 40;
    circle_174 = sqrt(dx_175 * dx_175 + dy_176 * dy_176) - 5;
  }

  float circle_177;
  {
    float dx_178 = x_0 - 40;
    float dy_179 = y_1 - 50;
    circle_177 = sqrt(dx_178 * dx_178 + dy_179 * dy_179) - 5;
  }

  float or_180 = min(circle_174, circle_177);

  float or_181 = min(circle_171, or_180);

  float circle_182;
  {
    float dx_183 = x_0 - 40;
    float dy_184 = y_1 - 60;
    circle_182 = sqrt(dx_183 * dx_183 + dy_184 * dy_184) - 5;
  }

  float circle_185;
  {
    float dx_186 = x_0 - 40;
    float dy_187 = y_1 - 70;
    circle_185 = sqrt(dx_186 * dx_186 + dy_187 * dy_187) - 5;
  }

  float or_188 = min(circle_182, circle_185);

  float circle_189;
  {
    float dx_190 = x_0 - 40;
    float dy_191 = y_1 - 80;
    circle_189 = sqrt(dx_190 * dx_190 + dy_191 * dy_191) - 5;
  }

  float circle_192;
  {
    float dx_193 = x_0 - 40;
    float dy_194 = y_1 - 90;
    circle_192 = sqrt(dx_193 * dx_193 + dy_194 * dy_194) - 5;
  }

  float or_195 = min(circle_189, circle_192);

  float or_196 = min(or_188, or_195);

  float or_197 = min(or_181, or_196);

  float or_198 = min(or_170, or_197);

  float or_199 = min(or_147, or_198);

  float or_200 = min(or_100, or_199);

  float circle_201;
  {
    float dx_202 = x_0 - 50;
    float dy_203 = y_1 - 0;
    circle_201 = sqrt(dx_202 * dx_202 + dy_203 * dy_203) - 5;
  }

  float circle_204;
  {
    float dx_205 = x_0 - 50;
    float dy_206 = y_1 - 10;
    circle_204 = sqrt(dx_205 * dx_205 + dy_206 * dy_206) - 5;
  }

  float circle_207;
  {
    float dx_208 = x_0 - 50;
    float dy_209 = y_1 - 20;
    circle_207 = sqrt(dx_208 * dx_208 + dy_209 * dy_209) - 5;
  }

  float or_210 = min(circle_204, circle_207);

  float or_211 = min(circle_201, or_210);

  float circle_212;
  {
    float dx_213 = x_0 - 50;
    float dy_214 = y_1 - 30;
    circle_212 = sqrt(dx_213 * dx_213 + dy_214 * dy_214) - 5;
  }

  float circle_215;
  {
    float dx_216 = x_0 - 50;
    float dy_217 = y_1 - 40;
    circle_215 = sqrt(dx_216 * dx_216 + dy_217 * dy_217) - 5;
  }

  float circle_218;
  {
    float dx_219 = x_0 - 50;
    float dy_220 = y_1 - 50;
    circle_218 = sqrt(dx_219 * dx_219 + dy_220 * dy_220) - 5;
  }

  float or_221 = min(circle_215, circle_218);

  float or_222 = min(circle_212, or_221);

  float or_223 = min(or_211, or_222);

  float circle_224;
  {
    float dx_225 = x_0 - 50;
    float dy_226 = y_1 - 60;
    circle_224 = sqrt(dx_225 * dx_225 + dy_226 * dy_226) - 5;
  }

  float circle_227;
  {
    float dx_228 = x_0 - 50;
    float dy_229 = y_1 - 70;
    circle_227 = sqrt(dx_228 * dx_228 + dy_229 * dy_229) - 5;
  }

  float circle_230;
  {
    float dx_231 = x_0 - 50;
    float dy_232 = y_1 - 80;
    circle_230 = sqrt(dx_231 * dx_231 + dy_232 * dy_232) - 5;
  }

  float or_233 = min(circle_227, circle_230);

  float or_234 = min(circle_224, or_233);

  float circle_235;
  {
    float dx_236 = x_0 - 50;
    float dy_237 = y_1 - 90;
    circle_235 = sqrt(dx_236 * dx_236 + dy_237 * dy_237) - 5;
  }

  float circle_238;
  {
    float dx_239 = x_0 - 60;
    float dy_240 = y_1 - 0;
    circle_238 = sqrt(dx_239 * dx_239 + dy_240 * dy_240) - 5;
  }

  float circle_241;
  {
    float dx_242 = x_0 - 60;
    float dy_243 = y_1 - 10;
    circle_241 = sqrt(dx_242 * dx_242 + dy_243 * dy_243) - 5;
  }

  float or_244 = min(circle_238, circle_241);

  float or_245 = min(circle_235, or_244);

  float or_246 = min(or_234, or_245);

  float or_247 = min(or_223, or_246);

  float circle_248;
  {
    float dx_249 = x_0 - 60;
    float dy_250 = y_1 - 20;
    circle_248 = sqrt(dx_249 * dx_249 + dy_250 * dy_250) - 5;
  }

  float circle_251;
  {
    float dx_252 = x_0 - 60;
    float dy_253 = y_1 - 30;
    circle_251 = sqrt(dx_252 * dx_252 + dy_253 * dy_253) - 5;
  }

  float circle_254;
  {
    float dx_255 = x_0 - 60;
    float dy_256 = y_1 - 40;
    circle_254 = sqrt(dx_255 * dx_255 + dy_256 * dy_256) - 5;
  }

  float or_257 = min(circle_251, circle_254);

  float or_258 = min(circle_248, or_257);

  float circle_259;
  {
    float dx_260 = x_0 - 60;
    float dy_261 = y_1 - 50;
    circle_259 = sqrt(dx_260 * dx_260 + dy_261 * dy_261) - 5;
  }

  float circle_262;
  {
    float dx_263 = x_0 - 60;
    float dy_264 = y_1 - 60;
    circle_262 = sqrt(dx_263 * dx_263 + dy_264 * dy_264) - 5;
  }

  float circle_265;
  {
    float dx_266 = x_0 - 60;
    float dy_267 = y_1 - 70;
    circle_265 = sqrt(dx_266 * dx_266 + dy_267 * dy_267) - 5;
  }

  float or_268 = min(circle_262, circle_265);

  float or_269 = min(circle_259, or_268);

  float or_270 = min(or_258, or_269);

  float circle_271;
  {
    float dx_272 = x_0 - 60;
    float dy_273 = y_1 - 80;
    circle_271 = sqrt(dx_272 * dx_272 + dy_273 * dy_273) - 5;
  }

  float circle_274;
  {
    float dx_275 = x_0 - 60;
    float dy_276 = y_1 - 90;
    circle_274 = sqrt(dx_275 * dx_275 + dy_276 * dy_276) - 5;
  }

  float circle_277;
  {
    float dx_278 = x_0 - 70;
    float dy_279 = y_1 - 0;
    circle_277 = sqrt(dx_278 * dx_278 + dy_279 * dy_279) - 5;
  }

  float or_280 = min(circle_274, circle_277);

  float or_281 = min(circle_271, or_280);

  float circle_282;
  {
    float dx_283 = x_0 - 70;
    float dy_284 = y_1 - 10;
    circle_282 = sqrt(dx_283 * dx_283 + dy_284 * dy_284) - 5;
  }

  float circle_285;
  {
    float dx_286 = x_0 - 70;
    float dy_287 = y_1 - 20;
    circle_285 = sqrt(dx_286 * dx_286 + dy_287 * dy_287) - 5;
  }

  float or_288 = min(circle_282, circle_285);

  float circle_289;
  {
    float dx_290 = x_0 - 70;
    float dy_291 = y_1 - 30;
    circle_289 = sqrt(dx_290 * dx_290 + dy_291 * dy_291) - 5;
  }

  float circle_292;
  {
    float dx_293 = x_0 - 70;
    float dy_294 = y_1 - 40;
    circle_292 = sqrt(dx_293 * dx_293 + dy_294 * dy_294) - 5;
  }

  float or_295 = min(circle_289, circle_292);

  float or_296 = min(or_288, or_295);

  float or_297 = min(or_281, or_296);

  float or_298 = min(or_270, or_297);

  float or_299 = min(or_247, or_298);

  float circle_300;
  {
    float dx_301 = x_0 - 70;
    float dy_302 = y_1 - 50;
    circle_300 = sqrt(dx_301 * dx_301 + dy_302 * dy_302) - 5;
  }

  float circle_303;
  {
    float dx_304 = x_0 - 70;
    float dy_305 = y_1 - 60;
    circle_303 = sqrt(dx_304 * dx_304 + dy_305 * dy_305) - 5;
  }

  float circle_306;
  {
    float dx_307 = x_0 - 70;
    float dy_308 = y_1 - 70;
    circle_306 = sqrt(dx_307 * dx_307 + dy_308 * dy_308) - 5;
  }

  float or_309 = min(circle_303, circle_306);

  float or_310 = min(circle_300, or_309);

  float circle_311;
  {
    float dx_312 = x_0 - 70;
    float dy_313 = y_1 - 80;
    circle_311 = sqrt(dx_312 * dx_312 + dy_313 * dy_313) - 5;
  }

  float circle_314;
  {
    float dx_315 = x_0 - 70;
    float dy_316 = y_1 - 90;
    circle_314 = sqrt(dx_315 * dx_315 + dy_316 * dy_316) - 5;
  }

  float circle_317;
  {
    float dx_318 = x_0 - 80;
    float dy_319 = y_1 - 0;
    circle_317 = sqrt(dx_318 * dx_318 + dy_319 * dy_319) - 5;
  }

  float or_320 = min(circle_314, circle_317);

  float or_321 = min(circle_311, or_320);

  float or_322 = min(or_310, or_321);

  float circle_323;
  {
    float dx_324 = x_0 - 80;
    float dy_325 = y_1 - 10;
    circle_323 = sqrt(dx_324 * dx_324 + dy_325 * dy_325) - 5;
  }

  float circle_326;
  {
    float dx_327 = x_0 - 80;
    float dy_328 = y_1 - 20;
    circle_326 = sqrt(dx_327 * dx_327 + dy_328 * dy_328) - 5;
  }

  float circle_329;
  {
    float dx_330 = x_0 - 80;
    float dy_331 = y_1 - 30;
    circle_329 = sqrt(dx_330 * dx_330 + dy_331 * dy_331) - 5;
  }

  float or_332 = min(circle_326, circle_329);

  float or_333 = min(circle_323, or_332);

  float circle_334;
  {
    float dx_335 = x_0 - 80;
    float dy_336 = y_1 - 40;
    circle_334 = sqrt(dx_335 * dx_335 + dy_336 * dy_336) - 5;
  }

  float circle_337;
  {
    float dx_338 = x_0 - 80;
    float dy_339 = y_1 - 50;
    circle_337 = sqrt(dx_338 * dx_338 + dy_339 * dy_339) - 5;
  }

  float circle_340;
  {
    float dx_341 = x_0 - 80;
    float dy_342 = y_1 - 60;
    circle_340 = sqrt(dx_341 * dx_341 + dy_342 * dy_342) - 5;
  }

  float or_343 = min(circle_337, circle_340);

  float or_344 = min(circle_334, or_343);

  float or_345 = min(or_333, or_344);

  float or_346 = min(or_322, or_345);

  float circle_347;
  {
    float dx_348 = x_0 - 80;
    float dy_349 = y_1 - 70;
    circle_347 = sqrt(dx_348 * dx_348 + dy_349 * dy_349) - 5;
  }

  float circle_350;
  {
    float dx_351 = x_0 - 80;
    float dy_352 = y_1 - 80;
    circle_350 = sqrt(dx_351 * dx_351 + dy_352 * dy_352) - 5;
  }

  float circle_353;
  {
    float dx_354 = x_0 - 80;
    float dy_355 = y_1 - 90;
    circle_353 = sqrt(dx_354 * dx_354 + dy_355 * dy_355) - 5;
  }

  float or_356 = min(circle_350, circle_353);

  float or_357 = min(circle_347, or_356);

  float circle_358;
  {
    float dx_359 = x_0 - 90;
    float dy_360 = y_1 - 0;
    circle_358 = sqrt(dx_359 * dx_359 + dy_360 * dy_360) - 5;
  }

  float circle_361;
  {
    float dx_362 = x_0 - 90;
    float dy_363 = y_1 - 10;
    circle_361 = sqrt(dx_362 * dx_362 + dy_363 * dy_363) - 5;
  }

  float circle_364;
  {
    float dx_365 = x_0 - 90;
    float dy_366 = y_1 - 20;
    circle_364 = sqrt(dx_365 * dx_365 + dy_366 * dy_366) - 5;
  }

  float or_367 = min(circle_361, circle_364);

  float or_368 = min(circle_358, or_367);

  float or_369 = min(or_357, or_368);

  float circle_370;
  {
    float dx_371 = x_0 - 90;
    float dy_372 = y_1 - 30;
    circle_370 = sqrt(dx_371 * dx_371 + dy_372 * dy_372) - 5;
  }

  float circle_373;
  {
    float dx_374 = x_0 - 90;
    float dy_375 = y_1 - 40;
    circle_373 = sqrt(dx_374 * dx_374 + dy_375 * dy_375) - 5;
  }

  float circle_376;
  {
    float dx_377 = x_0 - 90;
    float dy_378 = y_1 - 50;
    circle_376 = sqrt(dx_377 * dx_377 + dy_378 * dy_378) - 5;
  }

  float or_379 = min(circle_373, circle_376);

  float or_380 = min(circle_370, or_379);

  float circle_381;
  {
    float dx_382 = x_0 - 90;
    float dy_383 = y_1 - 60;
    circle_381 = sqrt(dx_382 * dx_382 + dy_383 * dy_383) - 5;
  }

  float circle_384;
  {
    float dx_385 = x_0 - 90;
    float dy_386 = y_1 - 70;
    circle_384 = sqrt(dx_385 * dx_385 + dy_386 * dy_386) - 5;
  }

  float or_387 = min(circle_381, circle_384);

  float circle_388;
  {
    float dx_389 = x_0 - 90;
    float dy_390 = y_1 - 80;
    circle_388 = sqrt(dx_389 * dx_389 + dy_390 * dy_390) - 5;
  }

  float circle_391;
  {
    float dx_392 = x_0 - 90;
    float dy_393 = y_1 - 90;
    circle_391 = sqrt(dx_392 * dx_392 + dy_393 * dy_393) - 5;
  }

  float or_394 = min(circle_388, circle_391);

  float or_395 = min(or_387, or_394);

  float or_396 = min(or_380, or_395);

  float or_397 = min(or_369, or_396);

  float or_398 = min(or_346, or_397);

  float or_399 = min(or_299, or_398);

  float or_400 = min(or_200, or_399);

  buffer[pos] = or_400; 
}
