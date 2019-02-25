#define POP() stack[--stack_ptr]
#define PUSH(v) stack[stack_ptr++] = v
#define PEEK() stack[stack_ptr - 1]

#define FETCH_SMALL() consts[program[++i]]

#define PUSH_POS(x, y, z) do {\
        position_stack[position_stack_ptr++]=x;\
        position_stack[position_stack_ptr++]=y;\
        position_stack[position_stack_ptr++]=z;\
    } while(0)
#define POP_POS() position_stack_ptr-=3
#define X_POS() position_stack[position_stack_ptr - 3]
#define Y_POS() position_stack[position_stack_ptr - 2]
#define Z_POS() position_stack[position_stack_ptr - 1]

__kernel void apply(
    __global float* buffer,
    __global float* consts,
    __global unsigned char* program,
    __global float* stack,
    __global float* position_stack,
    INPUT_BUFFERS,
    ulong max_stack,
    ulong max_position_stack,
    ulong width,
    ulong height,
    ulong instr_length)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t z = get_global_id(2);
    size_t pos = x + (y * width) + (z * width * height);

    size_t stack_ptr_start = pos * max_stack;
    size_t stack_ptr = pos * max_stack;
    size_t position_stack_ptr = pos * max_position_stack * 3;

    PUSH_POS((float) x, (float) y, (float) z);
    int winding = 0;

    for (ulong i = 0; i < instr_length; i++) {
        char code = program[i];
        //printf("normal: %d %d %d | translated: %f %f %f\n", x, y, z, X_POS(), Y_POS(), Z_POS());

        switch (code) {
            IMPLEMENT_INPUT_BUFFERS
            case OP_CONSTANT_SMALL: {
                float c = FETCH_SMALL();
                PUSH(c);
                break;
            }
            case OP_X: {
                float x_s = X_POS();
                PUSH(x_s);
                break;
            }
            case OP_Y: {
                float y_s = Y_POS();
                PUSH(y_s);
                break;
            }
            case OP_Z: {
                float z_s = Z_POS();
                PUSH(z_s);
                break;
            }
            case OP_ADD: {
                float l = POP();
                float r = POP();
                PUSH(l + r);
                break;
            }
            case OP_MUL: {
                float l = POP();
                float r = POP();
                PUSH(l * r);
                break;
            }
            case OP_SQUARE: {
                float v = POP();
                PUSH(v * v);
                break;
            }
            case OP_SUB: {
                float l = POP();
                float r = POP();
                PUSH(l - r);
                break;
            }
            case OP_MAX: {
                float l = POP();
                float r = POP();
                PUSH(fmax(l, r));
                break;
            }
            case OP_MIN: {
                float l = POP();
                float r = POP();
                PUSH(fmin(l, r));
                break;
            }
            case OP_ABS: {
                float v = POP();
                PUSH(fabs(v));
                break;
            }
            case OP_SQRT: {
                float v = POP();
                PUSH(sqrt(v));
                break;
            }
            case OP_NEG: {
                float v = POP();
                PUSH(-v);
                break;
            }
            case OP_DIST_TO_LINE: {
                float x1 = FETCH_SMALL();
                float y1 = FETCH_SMALL();
                float x2 = FETCH_SMALL();
                float y2 = FETCH_SMALL();
                float x_s = X_POS();
                float y_s = Y_POS();
                float2 res = dist_to_line_comp(x_s, y_s, x1, y1, x2, y2);

                float new = res.x;
                float is_left = res.y;
                if (y1 <= y_s) {
                    if(y2 > y_s) {
                        if (is_left > 0.0) {
                            winding ++;
                        }
                    }
                } else {
                    if (y2 <= y_s) {
                        if (is_left < 0.0) {
                            winding--;
                        }
                    }
                }

                PUSH(new);
                break;
            }
            case OP_COLLECT_POLY: {
                int count = program[++i];
                float minimum = INFINITY;
                for (int i = 0; i < count; i++) {
                    float d = POP();
                    minimum = min(minimum, fabs(d));
                }
                float s;
                if (winding == 0) {
                    s = 1.0;
                } else {
                    s = -1.0;
                }
                PUSH(copysign(minimum, s));
                winding = 0;
                break;
            }
            case OP_PUSH_TRANSFORM: {
                float m11 = FETCH_SMALL();
                float m12 = FETCH_SMALL();
                float m13 = FETCH_SMALL();
                float m14 = FETCH_SMALL();
                float m21 = FETCH_SMALL();
                float m22 = FETCH_SMALL();
                float m23 = FETCH_SMALL();
                float m24 = FETCH_SMALL();
                float m31 = FETCH_SMALL();
                float m32 = FETCH_SMALL();
                float m33 = FETCH_SMALL();
                float m34 = FETCH_SMALL();
                float m41 = FETCH_SMALL();
                float m42 = FETCH_SMALL();
                float m43 = FETCH_SMALL();
                float m44 = FETCH_SMALL();

                float x_s = X_POS();
                float y_s = Y_POS();
                float z_s = Z_POS();

                float x = x_s * m11 + y_s * m21 + z_s * m31 + m41;
                float y = x_s * m12 + y_s * m22 + z_s * m32 + m42;
                float z = x_s * m13 + y_s * m23 + z_s * m33 + m43;
                float w = x_s * m14 + y_s * m24 + z_s * m34 + m44;

                PUSH_POS(x / w, y / w, z / w);
                break;
            }
            case OP_POP_TRANSFORM: {
                POP_POS();
                break;
            }
            default: {
                printf("unrecognized opcode: %d\n", code);
            }
        }
    }

    buffer[pos] = POP();
    if (stack_ptr_start != stack_ptr ) {
        printf("stack_ptr_start != stack_ptr %d vs %d\n", stack_ptr_start, stack_ptr);
    }
}
