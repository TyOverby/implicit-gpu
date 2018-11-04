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
    __global char* program,
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

    size_t stack_ptr = pos * max_stack;
    size_t position_stack_ptr = pos * max_position_stack * 3;

    PUSH_POS((float) x, (float) y, (float) z);

    for (ulong i = 0; i < instr_length; i++) {
        char code = program[i];

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
            default: {
                printf("unrecognized opcode: %d\n", code);
            }
        }
    }

    buffer[pos] = POP();
}
